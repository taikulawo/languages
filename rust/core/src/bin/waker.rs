use std::{
    cell::RefCell,
    rc::Rc,
    sync::Arc,
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

unsafe fn clone(x: *const ()) -> RawWaker {
    let vdata = x as *const Vdata;
    Arc::increment_strong_count(vdata);
    let v = Arc::from_raw(x as *const Vdata);
    RawWaker::new(Arc::into_raw(v) as *const (), &TABLE)
}

unsafe fn drop(x: *const ()) {
    let vdata: ArcVdata = Arc::from_raw(x as *mut _);
    std::mem::drop(vdata);
}
unsafe fn wake(x: *const ()) {
    let vdata: ArcVdata = Arc::from_raw(x as *mut _);
    let s = format!("wake! {}", vdata.who());
    println!("{}", s);
    vdata.schedule()
}
unsafe fn wake_by_ref(x: *const ()) {
    let vdata = x as *const ArcVdata;
    let s = format!("wake by ref ! {}", (*vdata).who());
    println!("{}", s);
    (*vdata).schedule()
}
struct Scheduler {}
impl Scheduler {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    pub fn schedule(&self) {
        println!("call scheduler!");
    }
}
struct Vdata {
    scheduer: Arc<Scheduler>,
}
type ArcVdata = Arc<Vdata>;
impl Vdata {
    pub fn new() -> Self {
        Self {
            scheduer: Scheduler::new(),
        }
    }
    pub fn who(&self) -> &str {
        return "I'm Vdata";
    }
    pub fn schedule(&self) {
        self.scheduer.schedule()
    }
}

impl Clone for Vdata {
    fn clone(&self) -> Self {
        Self {
            scheduer: self.scheduer.clone(),
        }
    }
}
type Signer<'a> = Box<dyn Bar + 'a>;

#[tokio::main]
async fn main() {
    let local = task::LocalSet::new();
    let s = Scheduler::new();
    // 为什么current thread spawn 也要求Future Send？
    // current thread只有一个线程，也可以将rt从thread a move 到thread b
    // 使得future都会被Send到另一个thread
    let f = local.run_until(async move {
        tokio::spawn(async move {
            let b = Arc::into_raw(Arc::new(Vdata::new()));
            let raw_waker = RawWaker::new(b as *const _, &TABLE);
            let waker = unsafe { Waker::from_raw(raw_waker) };
            let ctx = Context::from_waker(&waker);
            let r = Rc::new(RefCell::new(ctx));
            let f: Signer = Box::new(Foo22 { cx: r.clone() });
            let r = r;
            let ctx = Context::from_waker(&waker);
            *r.borrow_mut() = ctx;
            f.say();
            let ctx = Context::from_waker(&waker);
            *r.borrow_mut() = ctx;
        });
        tokio::spawn(async move {
            let b = Arc::into_raw(Arc::new(Vdata::new()));
            let raw_waker = RawWaker::new(b as *const _, &TABLE);
            let waker = unsafe { Waker::from_raw(raw_waker) };
            let ctx = Context::from_waker(&waker);
            let r = Rc::new(RefCell::new(ctx));
            let f: Signer = Box::new(Foo22 { cx: r.clone() });
            let r = r;
            let ctx = Context::from_waker(&waker);
            *r.borrow_mut() = ctx;
            f.say();
            let ctx = Context::from_waker(&waker);
            *r.borrow_mut() = ctx;
        });
    });
    f.await;
}
