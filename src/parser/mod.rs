#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Add,
    Sub,
    Eq,
    Lt,
    Gt,
    And,
    Or,
    Neg,
    Not,
    Stack(StackAction, Segment<'a>, i16),
    Label(&'a str, &'a str),
    Goto(&'a str, &'a str),
    If(&'a str, &'a str),
    Function(&'a str, i16),
    Call(&'a str, i16),
    Return,
}
#[derive(Debug, PartialEq)]
pub enum Segment<'a> {
    Constant,
    Local,
    Argument,
    This,
    That,
    Temp,
    Pointer,
    Static(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum StackAction {
    Push,
    Pop,
}

pub fn parse<'a>(input: &'a str, func_name: &'a str) -> Vec<Command<'a>> {
    input
        .lines()
        .map(|l| l.split("//").nth(0).unwrap())
        .filter(|l| *l != "")
        .map(|l| l.trim())
        .map(|l| parse_command(l, func_name))
        .collect()
}

fn parse_command<'a>(line: &'a str, func_name: &'a str) -> Command<'a> {
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
        l if l.starts_with("push") => parse_push_pop_command(line, StackAction::Push, func_name),
        l if l.starts_with("pop") => parse_push_pop_command(line, StackAction::Pop, func_name),
        l if l.starts_with("label") => {
            let label_name = l.split(' ').nth(1).unwrap();
            Command::Label(label_name, func_name)
        }
        l if l.starts_with("goto") => {
            let label_name = l.split(' ').nth(1).unwrap();
            Command::Goto(label_name, func_name)
        }
        l if l.starts_with("if") => {
            let label_name = l.split(' ').nth(1).unwrap();
            Command::If(label_name, func_name)
        }
        l if l.starts_with("function") => {
            let label_name = l.split(' ').nth(1).unwrap();
            let locals_num = l.split(' ').nth(2).unwrap().parse::<i16>().unwrap();
            Command::Function(label_name, locals_num)
        }
        l if l.starts_with("return") => Command::Return,
        l if l.starts_with("call") => {
            let label_name = l.split(' ').nth(1).unwrap();
            let locals_num = l.split(' ').nth(2).unwrap().parse::<i16>().unwrap();
            Command::Call(label_name, locals_num)
        }
        _ => unreachable!(),
    }
}

fn parse_push_pop_command<'a>(line: &'a str, action: StackAction, func_name: &'a str) -> Command<'a> {
    let split: Vec<&str> = line.split(' ').collect();
    let (segment, index) = (split[1], split[2].parse::<i16>().unwrap());

    match segment {
        "constant" => Command::Stack(action, Segment::Constant, index),
        "local" => Command::Stack(action, Segment::Local, index),
        "argument" => Command::Stack(action, Segment::Argument, index),
        "this" => Command::Stack(action, Segment::This, index),
        "that" => Command::Stack(action, Segment::That, index),
        "temp" => Command::Stack(action, Segment::Temp, index),
        "pointer" => Command::Stack(action, Segment::Pointer, index),
        "static" => Command::Stack(action, Segment::Static(func_name), index),
        _ => unreachable!(),
    }
}
