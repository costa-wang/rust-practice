use std::fs::{self, File};
use std::io::Read;
use std::io::Write;

fn main() {
   let mut command_line: std::env::Args = std::env::args();
   command_line.next().unwrap();

   // 跳过程序名
   // 原文件
   let source = command_line.next().unwrap();
   // println!("{}",&source);
   let metadata = fs::metadata(&source);
   match metadata {
        Ok(meta) => {
            println!("{:?}",meta)
        }
        Err(err) => {
            panic!(err);
        }
    }
   // println!("源文件元数据：{:?}",metadata);
   // 新文件
   let destination = command_line.next().unwrap();
   let mut file_in = File::open(source).unwrap();
   let mut file_out = File::create(destination).unwrap();
   let mut buffer = [0u8; 4096];
   loop {
      let nbytes = file_in.read(&mut buffer).unwrap();
      file_out.write(&buffer[..nbytes]).unwrap();
      if nbytes < buffer.len() { break; }
   }
}
