use std::{
    env::{self, Args},
    error::Error,
    io,
    net::{SocketAddr, TcpListener},
    os::fd::{AsRawFd, RawFd},
    process::exit,
    thread::sleep,
    time::Duration,
};

use libc::fork;
use nix::{
    sys::socket::{socket, AddressFamily, SockFlag, SockType},
    NixPath,
};
use socket2::{Domain, SockAddr, Socket, Type};

fn do_upgrade() {}

fn send_fd_to<P>(fd: Vec<RawFd>, payload: &[u8], path: &P) -> io::Result<usize>
where
    P: NixPath,
{
    let send_fd = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::SOCK_NONBLOCK,
        None,
    );
    Ok(0)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    for ar in args {
        match &*ar {
            "--upgrade" => {
                do_upgrade();
                return Ok(());
            }
            _ => {}
        }
    }
    let mut s = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    let bind_addr = "0.0.0.0:14400";
    let addr: SockAddr = bind_addr.parse::<SocketAddr>()?.into();
    s.bind(&addr)?;
    s.listen(4096)?;
    s.set_nodelay(true)?;
    s.set_reuse_address(true)?;
    let listener: TcpListener = s.into();
    let fd = listener.as_raw_fd();
    let handler = std::thread::spawn(move || loop {
        let (s, addr) = match listener.accept() {
            Ok(x) => x,
            Err(err) => continue,
        };
        // 不处理，卡住，让backlog存连接
        sleep(Duration::from_secs(9999999));
    });
    sleep(Duration::from_secs(10));
    unsafe {
        let pid = fork();
        match pid {
            1.. => {
                // parent
            }
            0 => {
                // child
            }
            _ => {
                eprintln!("fork error");
                exit(0)
            }
        }
        if pid == 0 {
            // child
        }
    }
    // fork，将fd发到别的进程
    // let new_sock = socket2::Socket::
    Ok(())
}
