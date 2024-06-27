use tokio::task;

fn main() {
    let mut builder = tokio::runtime::Builder::new_current_thread();
    let rt = builder.enable_all().build().unwrap();
    let s = tokio::task::LocalSet::new();
    s.block_on(&rt, async move {
        tokio::spawn(async {
            println!("tokio spawn");
        })
        .await
        .unwrap();
        tokio::task::spawn_local(async {
            task::spawn_local(async move { println!("hello from spawn local") })
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
        .await
        .unwrap();
    });
}
