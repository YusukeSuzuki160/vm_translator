mod vm_translator;
use std::env;
use std::fs;
use vm_translator::{vm_parser::Parser, vm_writer::CodeWriter};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let dir_path = args[0].clone();
    let mut reader = Parser::new();
    let dir = fs::read_dir(dir_path.clone()).unwrap();
    let mut writer = CodeWriter::new();
    for item in dir.into_iter() {
        let path = item.unwrap().path();
        if path.to_str().unwrap().ends_with(".vm") {
            reader.vm_code.push(format!(
                "start {}",
                path.to_str()
                    .unwrap()
                    .split('/')
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
            ));
            reader.load_file(path.to_str().unwrap().to_string());
            writer.set_filename(
                path.to_str()
                    .unwrap()
                    .split('/')
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .to_string(),
            );
        }
    }
    writer.write_file(&dir_path, &mut reader);
}
