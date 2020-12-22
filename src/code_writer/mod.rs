use std::fmt::{Display, Formatter, Result};

use crate::parser::*;

#[derive(Debug)]
pub struct CodeWriter {
    pub code: Vec<String>,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self { code: vec![] }
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
            };
        }
    }

    fn write_push_pop(&mut self, act: StackAction, seg: Segment, index: i64) {
        match act {
            StackAction::Push => self.write_push_segment(seg, index),
            StackAction::Pop => self.write_pop_segment(seg, index),
        }
    }

    fn write_binary_operation(&mut self, op: &str) {
        self.set_a("SP");
        self.dec_m();
        self.set_a_from_m();
        self.set_d_from_m();
        self.set_a("SP");
        self.dec_m();
        self.set_a_from_m();
        self.append_lines(&format!("M=M{}D", op));
        self.set_a("SP");
        self.inc_m();
    }

    fn write_prefix_operation(&mut self, op: &str) {
        self.set_a("SP");
        self.dec_m();
        self.set_a_from_m();
        self.append_lines(&format!("M={}M", op));
        self.set_a("SP");
        self.inc_m();
    }

    fn write_compare(&mut self, cmp: &str) {
        self.set_a("SP");
        self.dec_m();
        self.set_a_from_m();
        self.set_d_from_m();
        self.set_a("SP");
        self.dec_m();
        self.set_a_from_m();
        self.append_lines(&format!("D=M-D"));
        // 比較
        self.set_a(&(self.code.len() + 9).to_string());
        self.append_lines(&format!("D;{}", cmp));
        // falseをスタックにpush
        self.set_a("SP");
        self.set_a_from_m();
        self.append_lines(&format!("M=0"));
        self.set_a("SP");
        self.inc_m();

        self.set_a(&(self.code.len() + 7).to_string());
        self.append_lines(&format!("0;JMP"));
        // trueをスタックにpush
        // -1を代入
        self.set_a("SP");
        self.set_a_from_m();
        self.append_lines(&format!("M=-1"));
        self.set_a("SP");
        self.inc_m();
    }

    fn write_push_segment(&mut self, seg: Segment, index: i64) {
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
        self.set_a("SP");
        self.inc_m();
    }

    fn write_pop_segment(&mut self, seg: Segment, index: i64) {
        let symbol = Self::get_symbol(&seg, &index);

        self.set_a("SP");
        self.dec_m();
        self.set_a_from_m();
        self.set_d_from_m();

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

    fn get_symbol(seg: &Segment, index: &i64) -> String {
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

    fn inc_a(&mut self, index: i64) {
        for _ in 0..index {
            self.append_lines("A=A+1");
        }
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

    fn append_lines(&mut self, line: &str) {
        self.code.push(line.to_string());
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
