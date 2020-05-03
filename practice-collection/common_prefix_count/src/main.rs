pub use core::cmp;

fn main() {
    let input = vec![
        ("doe", "reindeer"),
        ("dog", "puppy"),
        ("dogglesworth", "cat"),
        ];
    
    // println!("{}", input.len());
    let shared_nibble_count = common_prefix_count(&input);
    println!("{}", shared_nibble_count);
}

fn common_prefix_count<A, B>(input: &[(A, B)]) -> usize where 
A: AsRef<[u8]>,
B: AsRef<[u8]>
{
    let (key, _value) = (input[0].0.as_ref(), input[0].1.as_ref());

    let shared_nibble_count = input.iter().skip(1).fold(key.len(), |acc, &(ref k, _)| {
        cmp::min( shared_prefix_length(key, k.as_ref()), acc )
    });

    shared_nibble_count
}

fn shared_prefix_length<T: Eq>(first: &[T], second: &[T]) -> usize {
	first.iter()
		.zip(second.iter())
		.position(|(f, s)| f != s)
		.unwrap_or_else(|| cmp::min(first.len(), second.len()))
}