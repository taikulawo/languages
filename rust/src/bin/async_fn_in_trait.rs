use async_trait::async_trait;

trait Foo {
    async fn say(&self);
}

#[async_trait]
trait Bar {
    async fn say(&self);
}

trait NoAsyncTrait {
    fn say(&self);
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
struct NoAsyncTrait1 {}

impl NoAsyncTrait for NoAsyncTrait1 {
    fn say(&self) {
        println!("no async fn in trait are object safety");
    }
}

type FooT = Box<dyn Foo>;
type BarT = Box<dyn Bar>;
type NoAsyncTraitT = Box<dyn NoAsyncTrait>;

#[tokio::main]
async fn main() {
    let f1 = Foo1 {};
    let b1 = Bar1 {};
    let b0: BarT = Box::new(b1);
    let n0: NoAsyncTraitT = Box::new(NoAsyncTrait1 {});
    b0.say().await;
    f1.say().await;
    n0.say();
    // GAT is not object-safety, so `async fn in trait` too
    // let f0: FooT = Box::new(f1);
}
