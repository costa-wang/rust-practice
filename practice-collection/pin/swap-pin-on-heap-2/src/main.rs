use std::ops::Deref;
use std::ptr::NonNull;
use std::marker::PhantomPinned;
use std::pin::Pin;

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unpin for Unmovable {}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);

        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }

    fn get_data<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().data
    }
}

fn main() {
    let mut still_unmoved = Unmovable::new("hello".to_string());
    let mut new_unmoved = Unmovable::new("world".to_string());

    println!("{}, {}",still_unmoved.as_ref().get_data(), new_unmoved.as_ref().get_data());

    std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);

    println!("{}, {}",still_unmoved.as_ref().get_data(), new_unmoved.as_ref().get_data());
}