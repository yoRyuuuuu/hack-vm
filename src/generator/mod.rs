use crate::parser::*;

pub fn code(commands: Vec<Command>) -> String {
    let mut code = String::from("");
    let mut line_num = 0;
    for command in commands {
        let (c, n) = match command {
            Command::Push(seg, val) => push(seg, val),
            Command::Add => binary_opration("+"),
            Command::Sub => binary_opration("-"),
            Command::Eq => binary_opration("="),
        };
        code += &c;
        line_num += n
    }
    code
}

fn push(seg: Segment, val: i64) -> (String, usize) {
    match seg {
        Segment::Constant => {
            let code = vec![
                set_a(&val.to_string()),
                set_d_from_a(),
                set_a("SP"),
                set_a_from_m(),
                set_m_from_d(),
                set_a("SP"),
                inc_m(),
            ];
            let num = code.len();
            (join_code(code), num)
        }
    }
}

fn binary_opration(op: &str) -> (String, usize) {
    let code = vec![
        set_a("SP"),
        dec_m(),
        set_a_from_m(),
        set_d_from_m(),
        set_a("SP"),
        dec_m(),
        set_a_from_m(),
        format!("M=M{}D", op),
        set_a("SP"),
        inc_m(),
    ];
    let num = code.len();
    (join_code(code), num)
}

fn set_a(val: &str) -> String {
    format!("@{}", val)
}

fn set_d_from_a() -> String {
    format!("D=A")
}

fn set_a_from_m() -> String {
    format!("A=M")
}

fn set_m_from_d() -> String {
    format!("M=D")
}

fn set_d_from_m() -> String {
    format!("D=M")
}

fn set_m_from_a() -> String {
    format!("M=A")
}

fn inc_m() -> String {
    format!("M=M+1")
}

fn dec_m() -> String {
    format!("M=M-1")
}

fn join_code(code: Vec<String>) -> String {
    code.into_iter().fold("".to_string(), |l, r| l + &r + "\n")
}

mod tests {

}
