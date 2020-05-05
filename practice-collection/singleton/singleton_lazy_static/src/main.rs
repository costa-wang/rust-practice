#[macro_use] 
extern crate lazy_static;

use std::sync::Mutex;

pub struct Pool {
    pub name: String,
}

impl Pool {
    fn new(name:String)->Self{
        Pool {
           name,
        }
    }
}

lazy_static!{
    static ref POOL: Mutex<Pool> = Mutex::new(Pool::new("I'm a pool".to_string()));
    }

fn main() {
    println!("{}", std::any::type_name::<POOL>());
    let pool = POOL.lock().unwrap();
    println!("{}", pool.name);
}
