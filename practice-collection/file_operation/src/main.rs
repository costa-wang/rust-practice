use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};


fn main() {
   // let mut file = File::create("data.txt").expect("create failed");
   // println!("文件创建成功:{:?}",file);
   // file.write_all("简单教程".as_bytes()).expect("write failed");
   // file.write_all("\n简单编程".as_bytes()).expect("write failed");
   // let mut file = std::fs::File::open("data.txt").unwrap();
   // let mut contents = String::new();
   // file.read_to_string(&mut contents).unwrap();
   // print!("{}", contents);
   let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
   file.write_all("www.twle.cn".as_bytes()).expect("write failed");
   file.write_all("\n简单教程".as_bytes()).expect("write failed");
   file.write_all("\n简单编程".as_bytes()).expect("write failed");
   let mut file = std::fs::File::open("data.txt").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents)
}
