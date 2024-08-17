use std::error::Error;

fn main() {
    println!("dynamic size ptr size {}", std::mem::size_of::<&[i32]>());
    println!(
        "box dynamic size ptr size {}",
        std::mem::size_of::<Box<&[i32]>>()
    );
    println!("box sized ptr size {}", std::mem::size_of::<Box<i32>>());
}

type BoxedError = Box<dyn Error>;
fn foo<E: Error + 'static>(err: E) {
    let err: BoxedError = Box::new(err);
    // Box<dyn Error> can't implement Error because that makes the existing impl<E: Error> From<E> for Box<dyn Error> conflict with the other impl<T> From<T> for T
    // compiler error!
    // bar(err)
}

fn bar<T: std::error::Error>(err: T) {}
