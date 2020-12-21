#[derive(Debug, PartialEq)]
pub enum Command {
    Add,
    Sub,
    Eq,
    Push(Segment, i64),
    // Pop(Segment, i64),
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
    // Local,
    // Argument,
    // This,
    // That,
    // Temp,
    // Pointer
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
        l if l.starts_with("push") => parse_push_command(line),
        _ => unreachable!()
    }
}

fn parse_push_command(line: &str) -> Command {
    let split: Vec<&str> = line.split(' ').collect();
    let (segment, num) = (split[1], split[2].parse::<i64>().unwrap());
    match segment {
        "constant" => Command::Push(Segment::Constant, num),
        _ => panic!(),
    }
}


mod tests {
    use crate::parser::*;
    fn test_parse_push_command() {
        let input = "push constant 7";
        assert_eq!(
            parse_push_command(input),
            Command::Push(Segment::Constant, 7)
        );
        let input = "push constant 8";
        assert_eq!(
            parse_push_command(input),
            Command::Push(Segment::Constant, 8)
        );
    }
}
