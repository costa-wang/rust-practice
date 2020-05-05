// 微信文章:Rust生命周期bound用于泛型的引用

trait Print {
    fn output(&self);
}

struct Inner<'a>{
    data: &'a i32,
}

impl<'a> Print for Inner<'a> {
    fn output(&self) {
        println!("Inner data: {}", self.data);
    }
}

// 通过trait解耦
struct Outer<'b, T: 'b + Print> {
    pub inner: &'b T,
}

fn main() {
    let data = 10;
    {
        let a = Inner{ data: &data };
        let outer = Outer{ inner: &a };
        outer.inner.output();
    }
}
