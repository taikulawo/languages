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
    fn get<T: Decode>(&mut self, inner: T) {}
}
struct Bar {}
impl ReaderExt for Bar {
    type Item = Decoder;
    fn get(&mut self, inner: Self::Item) {}
}
struct Decoder {}
impl Decode for Decoder {}

fn main() {
    let foo = Foo {};
    // // 泛型trait无法装箱
    // // vtable只有一份，无法为出现的每个T实现
    // let b: Box<dyn BufExt> = Box::new(foo);

    // 但 associated type 可以
    // Item定义在trait，compiler可以为每个trait实现vtable
    let f: Box<dyn ReaderExt<Item = Decoder>> = Box::new(Bar {});
}
