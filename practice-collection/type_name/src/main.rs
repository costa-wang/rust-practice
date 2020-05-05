fn gen_value<T: Default>() -> T {
    println!("Initializing an instance of {}", std::any::type_name::<T>());
    Default::default()
}

fn main() {
    let _: i32 = gen_value();
    let _: String = gen_value();
}