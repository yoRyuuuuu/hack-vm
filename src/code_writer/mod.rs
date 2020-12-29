use std::fmt::{Display, Formatter, Result};

use crate::parser::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeWriter {
    pub code: Vec<String>,
    pub return_adds: HashMap<String, i16>,
}

impl CodeWriter {
    pub fn new() -> Self {
        let mut writer = Self {
            code: vec![],
            return_adds: HashMap::new(),
        };
        writer.write_boot();
        writer
    }

    fn write_boot(&mut self) {
        self.set_a("256");
        self.set_d_from_a();
        self.set_a("SP");
        self.set_m_from_d();
        self.write_call("Sys.init", 0);
    }

    pub fn write_code(&mut self, commands: Vec<Command>) {
        for command in commands {
            match command {
                Command::Stack(act, seg, index) => self.write_push_pop(act, seg, index),
                Command::Add => self.write_binary_operation("+"),
                Command::Sub => self.write_binary_operation("-"),
                Command::And => self.write_binary_operation("&"),
                Command::Or => self.write_binary_operation("|"),
                Command::Eq => self.write_compare("JEQ"),
                Command::Lt => self.write_compare("JLT"),
                Command::Gt => self.write_compare("JGT"),
                Command::Neg => self.write_prefix_operation("-"),
                Command::Not => self.write_prefix_operation("!"),
                Command::Label(label_name, func_name) => self.write_label(label_name, Some(func_name)),
                Command::Goto(label_name, func_name) => self.write_goto(label_name, Some(func_name)),
                Command::If(label_name, func_name) => self.write_if(label_name, Some(func_name)),
                Command::Function(label_name, locals_num) => {
                    self.write_function(label_name, locals_num)
                }
                Command::Call(label_name, locals_num) => self.write_call(label_name, locals_num),
                Command::Return => {
                    self.write_return();
                }
            };
        }
    }

    fn write_label<'a>(&mut self, label_name: &'a str, func_name: Option<&'a str>) {
        match func_name {
            Some(func_name) => self.append_lines(&format!("({}${})", func_name, label_name)),
            None => self.append_lines(&format!("({})", label_name)),
        }
    }

    fn write_goto<'a>(&mut self, label_name: &'a str, func_name: Option<&'a str>) {
        match func_name {
            Some(func_name) => self.set_a(&format!("{}${}", func_name, label_name)),
            None => self.set_a(label_name),
        }
        self.append_lines("0;JMP");
    }

    fn write_if<'a>(&mut self, label_name: &'a str, func_name: Option<&'a str>) {
        self.dec_sp();
        self.set_d_from_st();
        match func_name {
            Some(func_name) => self.set_a(&format!("{}${}", func_name, label_name)),
            None => self.set_a(label_name),
        }
        self.append_lines("D;JNE");
    }

    fn write_call<'a>(&mut self, label_name: &str, locals_num: i16) {
        self.append_lines(&format!("// call {} {}", label_name, locals_num));
        let count = self.return_adds.entry(label_name.to_string()).or_insert(0);
        *count += 1;
        let ret = format!("{}$RETURN{}", label_name, count);

        self.set_a(&ret);
        self.set_d_from_a();
        self.set_a("SP");
        self.set_a_from_m();
        self.set_m_from_d();
        self.inc_sp();

        let adds = ["LCL", "ARG", "THIS", "THAT"];
        for a in adds.iter() {
            self.set_a(a);
            self.set_d_from_m();
            self.set_a("SP");
            self.set_a_from_m();
            self.set_m_from_d();
            self.inc_sp();
        }

        self.set_a("SP");
        self.set_d_from_m();
        for _ in 0..locals_num + 5 {
            self.append_lines("D=D-1");
        }
        self.set_a("ARG");
        self.set_m_from_d();

        self.set_a("SP");
        self.set_d_from_m();
        self.set_a("LCL");
        self.set_m_from_d();

        self.write_goto(label_name, None);

        self.append_lines(&format!("({})", ret));
    }

    fn write_function(&mut self, label_name: &str, locals_num: i16) {
        self.append_lines(&format!("// function {} {}", label_name, locals_num));
        self.write_label(label_name, None);
        for _ in 0..locals_num {
            self.set_a("0");
            self.set_d_from_a();
            self.set_a("SP");
            self.set_a_from_m();
            self.set_m_from_d();
            self.inc_sp();
        }
    }

    fn write_return(&mut self) {
        self.append_lines(&format!("// return"));
        self.set_a("LCL");
        self.set_d_from_m();

        self.set_a("13");
        self.set_m_from_d();

        self.set_a("13");
        self.set_a_from_m();
        self.dec_a(5);
        self.set_d_from_m();
        self.set_a("14");
        self.set_m_from_d();

        self.dec_sp();
        self.set_a_from_m();
        self.set_d_from_m();
        self.set_a("ARG");
        self.set_a_from_m();
        self.set_m_from_d();

        self.set_a("ARG");
        self.set_d_from_m();
        self.append_lines("D=D+1");
        self.set_a("SP");
        self.set_m_from_d();

        let adds = ["THAT", "THIS", "ARG", "LCL"];
        for (i, a) in adds.iter().enumerate() {
            self.set_a("13");
            self.set_a_from_m();
            self.dec_a(i as i16 + 1);
            self.set_d_from_m();
            self.set_a(a);
            self.set_m_from_d();
        }

        self.set_a("14");
        self.set_a_from_m();
        self.append_lines("0;JMP");
    }

    fn write_push_pop(&mut self, act: StackAction, seg: Segment, index: i16) {
        match act {
            StackAction::Push => self.write_push_segment(seg, index),
            StackAction::Pop => self.write_pop_segment(seg, index),
        }
    }

    fn write_binary_operation(&mut self, op: &str) {
        self.dec_sp();
        self.set_d_from_st();
        self.dec_sp();
        self.set_a_from_m();
        self.append_lines(&format!("M=M{}D", op));
        self.inc_sp();
    }

    fn write_prefix_operation(&mut self, op: &str) {
        self.dec_sp();
        self.set_a_from_m();
        self.append_lines(&format!("M={}M", op));
        self.inc_sp();
    }

    fn write_compare(&mut self, cmp: &str) {
        self.dec_sp();
        self.set_d_from_st();

        self.dec_sp();
        self.set_a_from_m();
        self.append_lines("D=M-D");
        // 比較
        let num = self.get_line_num();
        self.set_a(&(num + 7).to_string());
        self.append_lines(&format!("D;{}", cmp));
        // falseをスタックにpush
        self.set_a("SP");
        self.set_a_from_m();
        self.append_lines(&format!("M=0"));

        let num = self.get_line_num();
        self.set_a(&(num + 5).to_string());
        self.append_lines(&format!("0;JMP"));
        // trueをスタックにpush
        // -1を代入
        self.set_a("SP");
        self.set_a_from_m();
        self.append_lines(&format!("M=-1"));
        self.inc_sp();
    }

    fn write_push_segment(&mut self, seg: Segment, index: i16) {
        self.append_lines(&format!("// push {:?} {}", seg, index));
        let symbol = Self::get_symbol(&seg, &index);

        self.set_a(&symbol);
        match seg {
            Segment::Constant => self.set_d_from_a(),
            Segment::Temp | Segment::Pointer => self.set_d_from_m(),
            _ => {
                self.set_a_from_m();
                self.inc_a(index);
                self.set_d_from_m();
            }
        }

        self.set_a("SP");
        self.set_a_from_m();
        self.set_m_from_d();
        self.inc_sp();
    }

    fn write_pop_segment(&mut self, seg: Segment, index: i16) {
        let symbol = Self::get_symbol(&seg, &index);

        self.dec_sp();
        self.set_d_from_st();

        self.set_a(&symbol);
        match seg {
            Segment::Temp | Segment::Pointer | Segment::Constant => (),
            _ => {
                self.set_a_from_m();
                self.inc_a(index);
            }
        }
        self.set_m_from_d();
    }

    fn get_symbol(seg: &Segment, index: &i16) -> String {
        match seg {
            Segment::Constant => index.to_string(),
            Segment::Local => "LCL".to_string(),
            Segment::Argument => "ARG".to_string(),
            Segment::That => "THAT".to_string(),
            Segment::This => "THIS".to_string(),
            Segment::Temp => (5 + index).to_string(),
            Segment::Pointer => (3 + index).to_string(),
            Segment::Static => (16).to_string(),
        }
    }

    fn inc_a(&mut self, index: i16) {
        for _ in 0..index {
            self.append_lines("A=A+1");
        }
    }

    fn dec_a(&mut self, index: i16) {
        for _ in 0..index {
            self.append_lines("A=A-1");
        }
    }

    fn set_d_from_st(&mut self) {
        self.set_a("SP");
        self.set_a_from_m();
        self.set_d_from_m();
    }

    fn set_a(&mut self, symbol: &str) {
        self.append_lines(&format!("@{}", symbol));
    }

    fn set_d_from_a(&mut self) {
        self.append_lines("D=A");
    }

    fn set_a_from_m(&mut self) {
        self.append_lines("A=M");
    }

    fn set_m_from_d(&mut self) {
        self.append_lines("M=D");
    }

    fn set_d_from_m(&mut self) {
        self.append_lines("D=M");
    }

    fn inc_m(&mut self) {
        self.append_lines("M=M+1");
    }

    fn dec_m(&mut self) {
        self.append_lines("M=M-1");
    }

    fn inc_sp(&mut self) {
        self.set_a("SP");
        self.inc_m();
    }

    fn dec_sp(&mut self) {
        self.set_a("SP");
        self.dec_m();
    }

    fn append_lines(&mut self, line: &str) {
        self.code.push(line.to_string());
    }

    fn get_line_num(&mut self) -> usize {
        self.code.len() - self.code.iter().filter(|line| (line.starts_with("(") && line.ends_with(")") || line.starts_with("//"))).count()
    }
}

impl Display for CodeWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            self.code.iter().fold(String::from(""), |l, r| l + r + "\n")
        )
    }
}
