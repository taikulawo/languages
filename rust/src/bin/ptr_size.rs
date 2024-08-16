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
    let _: BoxedError = Box::new(err);
}
