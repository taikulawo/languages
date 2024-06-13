#![allow(dead_code, unused)]
use std::future::{poll_fn, Future};
use std::pin::pin;
use std::task::{ready, Context, Poll};
struct Foo {}
impl Foo {
    pub async fn bar(&mut self) {}
    pub fn hi(&mut self, cx: &mut Context<'_>) {}
}
#[tokio::main]
async fn main() {
    let mut f = Foo {};
    let fut = f.bar();
    let mut fut = pin!(fut);
    // !compile error
    // poll_fn(|cx| {
    //     ready!(fut.as_mut().poll(cx));
    //     f.hi(cx);
    //     return Poll::Ready(());
    // })
    // .await;
}
