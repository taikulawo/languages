fn main() {
    println!("dynamic size ptr size {}", std::mem::size_of::<&[i32]>());
    println!(
        "box dynamic size ptr size {}",
        std::mem::size_of::<Box<&[i32]>>()
    );
    println!("box sized ptr size {}", std::mem::size_of::<Box<i32>>());
}
