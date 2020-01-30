use std::thread;
fn main() {
  let handle =  thread::spawn(|| {
    "Hello from a new thread!"
});
println!("{}",handle.join().unwrap());
}
