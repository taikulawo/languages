use std::any::Any;

trait IO {}

impl<T> IO for T {}
type Stream = Box<dyn IO>;
struct Foo {
    name: String,
}
fn main() {
    let f = Foo {
        name: "hello".into(),
    };
    let b1: Box<dyn Any> = Box::new(f);
    let b2: Stream = match b1.downcast::<Foo>() {
        Ok(v) => {
            println!("{}", v.name);
            v
        }
        Err(_) => return,
    };
    let b3: Stream = Box::new(b2);
    let b3: Box<dyn Any> = Box::new(b3);
    match b3.downcast::<Foo>() {
        Ok(v) => {
            println!("{}", v.name);
        }
        Err(_) => {
            // Stream多Box几次就无法转换为Foo
            // 所以想要downcast回去，只能Box一次
            println!("not Foo anymore")
        }
    };
}
