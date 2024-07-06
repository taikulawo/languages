#![allow(unused)]
#[allow(unused_variables)]
trait Decode {}
trait BufExt {
    fn get<T: Decode>(&mut self, inner: T);
}
trait ReaderExt {
    type Item: Decode;
    fn get(&mut self, inner: Self::Item);
}
struct Foo {}
impl BufExt for Foo {
    fn get<T: Decode>(&mut self, _inner: T) {}
}
struct Bar {}
impl ReaderExt for Bar {
    type Item = Decoder;
    fn get(&mut self, _inner: Self::Item) {}
}
struct Decoder {}
impl Decode for Decoder {}

fn main() {
    let foo = Foo {};
    // // 泛型trait无法装箱
    // // vtable只有一份，无法为出现的每个T实现
    // // BufExt#get 根据T不同能产生不同的函数，一个vtable不能满足需求
    // // Box<dyn BufExt> 从外部来看不知道方法是generic的，而vtable必须确定，且只有一个。
    // let b: Box<dyn BufExt> = Box::new(foo);

    // 但 associated type 可以
    // Item定义在trait，compiler可以为每个trait实现vtable
    let f: Box<dyn ReaderExt<Item = Decoder>> = Box::new(Bar {});
}
