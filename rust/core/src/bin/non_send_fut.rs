#[tokio::main]
async fn main() {
    let s = tokio::task::LocalSet::new();
    s.run_until(async {
        s.spawn_local(async move { println!("hello from spawn local") }).await.unwrap();
        tokio::task::spawn_local(async move { println!("hello from spawn local") }).await.unwrap();
        println!("run until");
    }).await;
}
