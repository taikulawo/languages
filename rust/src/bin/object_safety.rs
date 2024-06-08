use async_trait::async_trait;

#[async_trait]
trait ObjectSafety /*: SuperTrait */ {
    async fn f(&self);
    async fn g(&mut self);
    async fn foo(self);
}

// trait SuperTrait: Send {
//     fn t(self);
// }

// impl SuperTrait for ObjectSafety {
//     fn t(self) {
//         todo!()
//     }
// }
struct MyType {}
#[async_trait]
impl ObjectSafety for MyType {
    async fn f(&self) {
        println!("call f");
    }
    async fn g(&mut self) {
        println!("call g");
    }
    async fn foo(self) {
        println!("call g");
    }
}
#[tokio::main]
async fn main() {
    let mut v = MyType {};
    let object: &mut dyn ObjectSafety = &mut v;
    object.f().await;
    object.g().await;
    let _f = object.f();
    // compile error due to lifetime borrow on f
    // let g = object.g();
}
