use std::{cell::RefCell, rc::Rc, time::Duration};

use tokio::{runtime::Runtime, task::LocalSet};
thread_local! {}
fn main() {
    let rt = Runtime::new().unwrap();
    let local = LocalSet::new();
    local.block_on(&rt, async move {
        let c = Rc::new(RefCell::new(1));
        let c1 = c.clone();
        tokio::task::spawn_local(async move {
            foo(c1).await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        });
        let x = c.borrow();
        foo(c.clone()).await;
    });
}

async fn foo(c: Rc<RefCell<i32>>) {
    c.borrow_mut();
}
