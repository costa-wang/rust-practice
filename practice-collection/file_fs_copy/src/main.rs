use std::fs;

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
            println!("{:?}",meta);
        }
        Err(err) => {
            panic!(err);
        }
    }
   // println!("源文件元数据：{:?}",metadata);
   // 新文件
   let destination = command_line.next().unwrap();
   let result = fs::copy(&source,&destination);
   match result {
        Ok(res) => {
        println!("{:?}",res);
        }
        Err(err) => {
           panic!(err)
        } 
    }
}
