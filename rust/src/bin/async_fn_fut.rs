#![allow(unused)]
#[allow(unused_variables)]
use std::{future::Future, io};

use futures::future::BoxFuture;

#[tokio::main]
async fn main() {}

struct RustlsStream<T> {
    inner: T,
    fut: Option<BoxFuture<'static, ()>>,
}

impl<T> RustlsStream<T> {
    async fn foo(&self) {}
}

impl<T> Future for RustlsStream<T> {
    type Output = io::Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}
