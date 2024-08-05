struct Block {}
#[derive(Default)]
struct Inbound {
    block: Option<Block>,
}

impl Inbound {
    async fn not_works() -> Self {
        Self {
            ..Default::default()
        }
    }
    async fn works() -> Self {
        Self { block: None }
    }
}
// error[E0509]: cannot move out of type `Inbound`, which implements the `Drop` trait
// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=b94cc085034405baa482b7686a9272c4
// uncomment it

// impl Drop for Inbound {
//     fn drop(&mut self) {
//         self.block.take();
//     }
// }

#[tokio::main]
async fn main() {
    let inbound = Inbound::not_works().await;
}
