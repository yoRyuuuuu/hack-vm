use std::env;
use std::fs;
mod code_writer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("引数の数が違います");
        println!("usage: [入力ファイル名]");
        return;
    }

    let in_file_name = args[1].to_owned();
    let input = fs::read_to_string(in_file_name).unwrap();
    let commands = parser::parse(&input);
    let mut writer = code_writer::CodeWriter::new();
    writer.write_code(commands);
    print!("{}", writer);
}
