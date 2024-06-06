use async_trait::async_trait;

trait Foo {
    async fn say(&self);
}

#[async_trait]
trait Bar {
    async fn say(&self);
}
struct Foo1 {}

impl Foo for Foo1 {
    async fn say(&self) {
        println!("foo#say");
    }
}
struct Bar1 {}
#[async_trait]
impl Bar for Bar1 {
    async fn say(&self) {
        println!("bar#say");
    }
}

type FooT = Box<dyn Foo>;
type BarT = Box<dyn Bar>;

#[tokio::main]
async fn main() {
    let f1 = Foo1 {};
    let b1 = Bar1 {};
    let b0: BarT = Box::new(b1);
    b0.say().await;
    f1.say().await;
    // GAT is not object-safety, so `async fn in trait` too
    // let f0: FooT = Box::new(f1);
}
