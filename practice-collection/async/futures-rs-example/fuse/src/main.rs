/// Fuse a stream such that [`poll_next`](Stream::poll_next) will never
/// again be called once it has finished. This method can be used to turn
/// any `Stream` into a `FusedStream`.

/// Fuse一个流，这样['poll_next'](Stream::poll_next)在完成后就不会再被调用。
/// 此方法可用于将任何“Stream”转换为“FusedStream”。

/// Normally, once a stream has returned [`None`] from
/// [`poll_next`](Stream::poll_next) any further calls could exhibit bad
/// behavior such as block forever, panic, never return, etc. If it is known
/// that [`poll_next`](Stream::poll_next) may be called after stream
/// has already finished, then this method can be used to ensure that it has
/// defined semantics.
/// 通常情况下，一旦一个流从['poll_next'](stream::poll_next)返回了['None']，
/// 任何进一步的调用都可能表现出不好的行为，例如永久阻塞、恐慌、从不返回等。
/// 如果知道['poll_next'](Stream::poll_next)可能在Stream完成后调用，
/// 那么可以使用这个方法来确保它已经定义了语义。
///
/// The [`poll_next`](Stream::poll_next) method of a `fuse`d stream
/// is guaranteed to return [`None`] after the underlying stream has
/// finished.
/// 一个'fused'流的[' poll_next '](Stream::poll_next)方法保证在底层流完成后返回['None']。

use futures::executor::block_on_stream;
use futures::stream::{self, StreamExt};
use futures::task::Poll;

fn main() {
    let mut x = 0;
    let stream = stream::poll_fn(|_| {
        x += 1;
        match x {
            0..=2 => Poll::Ready(Some(x)),
            3 => Poll::Ready(None),
            _ => panic!("should not happen")
        }
    }).fuse();
    
    let mut iter = block_on_stream(stream);
    // print!("{}",iter.next().unwrap());
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
}
