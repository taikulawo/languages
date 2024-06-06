use std::{
    cell::RefCell,
    rc::Rc,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

use tokio::task;

struct Foo22<'a> {
    cx: Rc<RefCell<Context<'a>>>,
}
trait Bar {
    fn say(&self);
}
impl<'a> Bar for Foo22<'a> {
    fn say(&self) {
        let cx = self.cx.borrow_mut();
        let new_ctx = Context::from_waker(cx.waker());
        println!("hello world");
        new_ctx.waker().wake_by_ref();
        new_ctx.waker().clone().wake();
    }
}
static TABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

unsafe fn clone(d: *const ()) -> RawWaker {
    RawWaker::new(d, &TABLE)
}

unsafe fn drop(x: *const ()) {
    let vdata: BoxedVdata = Box::from_raw(x as *mut _);
    std::mem::drop(vdata);
}
unsafe fn wake(x: *const ()) {
    let vdata: BoxedVdata = Box::from_raw(x as *mut _);
    let s = format!("wake! {}", vdata.who());
    println!("{}", s);
}
unsafe fn wake_by_ref(x: *const ()) {
    let vdata: BoxedVdata = Box::from_raw(x as *mut _);
    let s = format!("wake by ref ! {}", vdata.who());
    println!("{}", s);
}
struct Vdata {}
type BoxedVdata = Box<Vdata>;
impl Vdata {
    pub fn who(&self) -> &str {
        return "I'm Vdata";
    }
}
type Signer<'a> = Box<dyn Bar + 'a>;

#[tokio::main]
async fn main() {
    let local = task::LocalSet::new();

    task::LocalSet::new()
        .run_until(async move {
            local
                .spawn_local(async move {
                    let b = Box::into_raw(Box::new(Vdata {}));
                    let raw_waker = RawWaker::new(b as *const _, &TABLE);
                    let waker = unsafe { Waker::from_raw(raw_waker) };
                    let ctx = Context::from_waker(&waker);
                    let r = Rc::new(RefCell::new(ctx));
                    let f: Signer = Box::new(Foo22 { cx: r.clone() });
                    let ctx = Context::from_waker(&waker);
                    *r.borrow_mut() = ctx;
                    f.say();
                    let ctx = Context::from_waker(&waker);
                    *r.borrow_mut() = ctx;
                })
                .await
                .unwrap();
        })
        .await;
}
