use std::sync::Arc;

pub struct Pool {
    pub name: String,
}

pub fn establish_connection() -> Arc<Pool> {
    static mut POOL: Option<Arc<Pool>> = None;
    unsafe {
        Arc::clone(POOL.get_or_insert_with(|| {
            println!("init pool ~~~~~~~~~~~~");
            Arc::new(Pool {
                name: "I'm a pool".to_string(),
            })
        }))
    }
}

fn main() {
    let a = establish_connection();
    println!("{}", a.name);
    
    let b = establish_connection();
    println!("{}", b.name);
}