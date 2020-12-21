#[derive(Debug, PartialEq)]
pub enum Command {
    Add,
    Sub,
    Eq,
    Lt,
    Gt,
    And,
    Or,
    Neg,
    Not,
    Stack(StackAction, Segment, i64)
    // Label(&'a str),
    // Goto(&'a str),
    // If(&'a str),
    // Funtion(&'a str),
    // Return(&'a str),
    // Call(&'a str),
}
#[derive(Debug, PartialEq)]
pub enum Segment {
    Constant,
    Local,
    Argument,
    This,
    That,
    // Temp,
    // Pointer
}

#[derive(Debug, PartialEq)]
pub enum StackAction {
    Push,
    Pop,
}

pub fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| l.split("//").nth(0).unwrap())
        .filter(|l| *l != "")
        .map(|l| l.trim())
        .map(|l| parse_command(l))
        .collect()
}

fn parse_command(line: &str) -> Command {
    match line {
        l if l.starts_with("add") => Command::Add,
        l if l.starts_with("sub") => Command::Sub,
        l if l.starts_with("eq") => Command::Eq,
        l if l.starts_with("lt") => Command::Lt,
        l if l.starts_with("gt") => Command::Gt,
        l if l.starts_with("and") => Command::And,
        l if l.starts_with("or") => Command::Or,
        l if l.starts_with("neg") => Command::Neg,
        l if l.starts_with("not") => Command::Not,
        l if l.starts_with("push") => parse_push_pop_command(line, StackAction::Push),
        l if l.starts_with("pop") => parse_push_pop_command(line, StackAction::Pop),
        _ => unreachable!()
    }
}

fn parse_push_pop_command(line: &str, action: StackAction) -> Command {
    let split: Vec<&str> = line.split(' ').collect();
    let (segment, num) = (split[1], split[2].parse::<i64>().unwrap());

    match segment {
        "constant" => Command::Stack(action, Segment::Constant, num),
        // "local" => Command::Stack(Segment::Local, num),
        // "argument" => Command::Stack(Segment::Argument, num),
        // "this" => Command::Push(Command::This,)
        _ => panic!(),
    }
}


mod tests {
    use crate::parser::*;
    fn test_parse_push_command() {
        let input = "push constant 7";
        assert_eq!(
            parse_push_pop_command(input, StackAction::Push),
            Command::Stack(StackAction::Push, Segment::Constant, 7)
        );
        let input = "push constant 8";
        assert_eq!(
            parse_push_pop_command(input, StackAction::Pop),
            Command::Stack(StackAction::Push, Segment::Constant, 7)
        );
    }
}
