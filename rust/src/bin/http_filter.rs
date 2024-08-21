use std::io;

use async_trait::async_trait;
#[async_trait]
trait HttpFilter: Send + Sync {
    async fn request_header_filter(
        &mut self,
        ctx: &mut Session,
        sess: &mut RoffHttpServerSession,
    ) -> io::Result<HttpFilterStatus> {
        Ok(HttpFilterStatus::Continue)
    }
    async fn response_filter(&self) -> io::Result<HttpFilterStatus> {
        Ok(HttpFilterStatus::Continue)
    }
}
struct RoffHttpServerSession {}
struct Session {}
enum HttpFilterStatus {
    Continue,
}
struct Foo {}

trait HttpHeaderFilterEnable {
    fn request_header_filter_enable(&self) -> bool;
}
#[async_trait]
trait HttpHeaderFilter: Send + Sync {
    // do something
    async fn request_header_filter(
        &mut self,
        ctx: &mut Session,
        sess: &mut RoffHttpServerSession,
    ) -> io::Result<HttpFilterStatus>;
}

trait HttpRespFilterEnable {
    fn request_resp_filter_enable(&self) -> bool;
}
impl<T> HttpRespFilterEnable for T
where
    T: HttpFilter + ?Sized,
{
    fn request_resp_filter_enable(&self) -> bool {
        true
    }
}
#[async_trait]
trait HttpRespFilter: Send + Sync {
    // do som
    async fn request_header_filter(&mut self) -> io::Result<HttpFilterStatus>;
}

#[async_trait]
impl<T> HttpRespFilter for T
where
    T: HttpFilter + ?Sized,
{
    async fn request_header_filter(&mut self) -> io::Result<HttpFilterStatus> {
        Ok(HttpFilterStatus::Continue)
    }
}

// 对实现HttpHeaderFilter的结构默认实现true
impl<T> HttpHeaderFilterEnable for T
where
    T: HttpFilter + ?Sized,
{
    fn request_header_filter_enable(&self) -> bool {
        true
    }
}

#[async_trait]
impl HttpHeaderFilter for Foo {
    async fn request_header_filter(
        &mut self,
        ctx: &mut Session,
        sess: &mut RoffHttpServerSession,
    ) -> io::Result<HttpFilterStatus> {
        println!("our module implementation");
        Ok(HttpFilterStatus::Continue)
    }
}

#[async_trait]
impl<T> HttpFilter for T
where
    T: HttpHeaderFilter,
{
    async fn request_header_filter(
        &mut self,
        ctx: &mut Session,
        sess: &mut RoffHttpServerSession,
    ) -> io::Result<HttpFilterStatus> {
        (self as &mut dyn HttpHeaderFilter)
            .request_header_filter(ctx, sess)
            .await
    }
}

#[tokio::main]
async fn main() {
    let mut foo: Box<dyn HttpFilter> = Box::new(Foo {});
    let mut s1 = Box::new(RoffHttpServerSession {});
    let mut s2 = Session {};
    foo.request_header_filter(&mut s2, &mut s1).await.unwrap();
    let enabled = foo.request_header_filter_enable();
    println!("http header filter {enabled}");
    let enabled = foo.request_resp_filter_enable();
    println!("http resp filter {enabled}");
    foo.response_filter().await.unwrap();
}
