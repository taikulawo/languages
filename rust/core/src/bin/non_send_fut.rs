#[tokio::main]
async fn main() {
    let s = tokio::task::LocalSet::new();
    tokio::spawn(async {
        println!("tokio spawn");
    })
    .await.unwrap();
    s.run_until(async {
        s.spawn_local(async move { println!("hello from spawn local") })
            .await
            .unwrap();
        tokio::task::spawn_local(async move { println!("hello from spawn local") })
            .await
            .unwrap();
        tokio::spawn(async move { println!("hello from spawn Send") })
            .await
            .unwrap();
        println!("run until");
    })
    .await;
}
