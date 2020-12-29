use std::env;
use std::fs;
mod code_writer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut inputs = vec![];
    let mut commands = vec![];
    for name in args[1..].iter() {
        inputs.push(fs::read_to_string(name).unwrap());
    }

    for (i, name) in args[1..].iter().enumerate() {
        let func_name = name.split("/").last().unwrap().split('.').nth(0).unwrap();
        let mut temp =  parser::parse(&inputs[i], func_name);
        commands.append(&mut temp);
    }
    let mut writer = code_writer::CodeWriter::new();
    writer.write_code(commands);
    print!("{}", writer);
}
