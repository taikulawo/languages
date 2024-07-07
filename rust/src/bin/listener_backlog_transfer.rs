use std::{
    error::Error,
    net::{SocketAddr, TcpListener},
    os::fd::AsRawFd,
    thread::sleep,
    time::Duration,
};

use socket2::{Domain, SockAddr, Socket, Type};

fn main() -> Result<(), Box<dyn Error>> {
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
    // fork，将fd发到别的进程
    // let new_sock = socket2::Socket::
    Ok(())
}
