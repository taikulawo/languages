use url::Url;

fn main() {
    unsafe {
        let mut r = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        libc::getrlimit(libc::RLIMIT_NOFILE, &mut r);
        println!("first {:?}", r);
        let mut r = libc::rlimit {
            rlim_cur: 1000000,
            rlim_max: 1000000,
        };
        libc::setrlimit(libc::RLIMIT_NOFILE, &r);
        libc::getrlimit(libc::RLIMIT_NOFILE, &mut r);
        println!("second {:?}", r);
    }
    println!("Hello, world!");
}
