use std::{
    collections::HashMap,
    env::{self, current_exe},
    error::Error,
    fmt::Display,
    io::{IoSlice, IoSliceMut, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    os::fd::{AsFd, AsRawFd, FromRawFd, IntoRawFd, RawFd},
    sync::{Arc, Condvar, Mutex},
    thread::{self, sleep},
    time::{self, Duration},
};

use anyhow::anyhow;
use nix::{
    errno::Errno,
    sys::{
        self,
        socket::{self, socket, AddressFamily, Backlog, RecvMsg, SockFlag, SockType, UnixAddr},
        stat,
    },
    NixPath,
};
use socket2::{Domain, SockAddr, Socket, Type};
const UNIX_PATH: &'static str = "/var/run/transfer.sock";
const BIND_ADDR: &'static str = "0.0.0.0:14400";
#[cfg(target_os = "linux")]
const MAX_RETRY: usize = 5;
#[cfg(target_os = "linux")]
const RETRY_INTERVAL: time::Duration = time::Duration::from_secs(1);
const RESPONSE: &[u8] = b"HTTP";
use std::os::fd::BorrowedFd;
#[cfg(target_os = "linux")]
fn accept_with_retry(listen_fd: BorrowedFd<'_>) -> anyhow::Result<i32> {
    let mut retried = 0;
    loop {
        match socket::accept(listen_fd.as_raw_fd()) {
            Ok(fd) => return Ok(fd),
            Err(e) => {
                if retried > MAX_RETRY {
                    return Err(anyhow!(e));
                }
                match e {
                    Errno::EAGAIN => {
                        eprintln!(
                            "No incoming socket transfer, sleep {RETRY_INTERVAL:?} and try again"
                        );
                        retried += 1;
                        thread::sleep(Duration::from_secs(2));
                    }
                    _ => {
                        eprintln!("Error accepting socket transfer: {e}");
                        // fallback to cool start
                        return Err(anyhow!(e));
                    }
                }
            }
        }
    }
}

fn do_upgrade() {
    let mut fds = Fds::new();
    fds.get_from_sock(UNIX_PATH).unwrap();
    let fd = fds.get(BIND_ADDR).unwrap();
    let mut s = unsafe { Socket::from_raw_fd(*fd) };
    s.listen(65535).unwrap();
    let listener: TcpListener = s.into();
    println!("new process start listen ");
    loop {
        let (mut stream, addr) = match listener.accept() {
            Ok(x) => x,
            Err(err) => {
                eprintln!("accept error {err:?}");
                continue;
            }
        };
        stream.write_all(RESPONSE).unwrap();
        println!("accept new connection");
    }
}
// TODO pidfd_getfd syscall
fn get_fds_from<P>(path: &P, payload: &mut [u8]) -> anyhow::Result<(Vec<RawFd>, usize)>
where
    P: ?Sized + NixPath + Display,
{
    const MAX_FDS: usize = 32;
    let listen_fd = socket::socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::SOCK_NONBLOCK,
        None,
    )
    .unwrap();
    let unix_addr = UnixAddr::new(path).unwrap();
    match nix::unistd::unlink(path) {
        Ok(()) => {
            println!("unlink done");
        }
        Err(_) => {}
    };
    socket::bind(listen_fd.as_raw_fd(), &unix_addr).unwrap();
    sys::stat::fchmodat(
        None,
        path,
        stat::Mode::all(),
        stat::FchmodatFlags::FollowSymlink,
    )
    .unwrap();
    let backlog = Backlog::new(8).unwrap();
    socket::listen(&listen_fd, backlog).unwrap();
    let fd = match accept_with_retry(listen_fd.as_fd()) {
        Ok(fd) => fd,
        Err(err) => {
            eprintln!("read error {err:?}");
            if nix::unistd::close(listen_fd.into_raw_fd()).is_ok() {
                nix::unistd::unlink(path).unwrap();
            }
            return Err(anyhow!(err));
        }
    };
    println!("received fd {}", fd);
    let mut io_vec = [IoSliceMut::new(payload); 1];
    let mut cmsg_buf = nix::cmsg_space!([RawFd; MAX_FDS]);
    let msg: RecvMsg<UnixAddr> = socket::recvmsg(
        fd,
        &mut io_vec,
        Some(&mut cmsg_buf),
        socket::MsgFlags::empty(),
    )
    .unwrap();

    let mut fds: Vec<RawFd> = Vec::new();
    for cmsg in msg.cmsgs().unwrap() {
        if let socket::ControlMessageOwned::ScmRights(mut vec_fds) = cmsg {
            fds.append(&mut vec_fds)
        } else {
            eprintln!("Unexpected control messages: {cmsg:?}")
        }
    }

    //cleanup
    if nix::unistd::close(listen_fd.into_raw_fd()).is_ok() {
        nix::unistd::unlink(path).unwrap();
    }

    Ok((fds, msg.bytes))
}
fn send_fds_to<P>(fds: Vec<RawFd>, payload: &[u8], path: &P) -> anyhow::Result<usize>
where
    P: ?Sized + NixPath + Display,
{
    let send_fd = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::SOCK_NONBLOCK,
        None,
    )?;
    let unix_addr = UnixAddr::new(path)?;
    let mut nonblocking_polls = 0;
    let mut retried = 0;
    let conn_result: anyhow::Result<usize> = loop {
        match socket::connect(send_fd.as_raw_fd(), &unix_addr) {
            Ok(_) => break Ok(0),
            Err(err) => match err {
                Errno::ENOENT | Errno::ECONNREFUSED | Errno::EACCES => {
                    retried += 1;
                    if retried > 3 {
                        eprintln!("max retry exceeded");
                        break Err(anyhow!(err));
                    }
                }
                Errno::EINPROGRESS => {
                    nonblocking_polls += 1;
                    if nonblocking_polls >= 3 {
                        eprintln!("err");
                        break Err(anyhow!(err));
                    }
                }
                _ => break Err(anyhow!(err)),
            },
        }
    };
    use nix::sys::socket::ControlMessage;
    let result = match conn_result {
        Ok(..) => {
            let io_vec = [IoSlice::new(payload); 1];
            let scm = ControlMessage::ScmRights(fds.as_slice());
            let cmsg = [scm; 1];
            loop {
                match socket::sendmsg(
                    send_fd.as_raw_fd(),
                    &io_vec,
                    &cmsg,
                    socket::MsgFlags::empty(),
                    None::<&UnixAddr>,
                ) {
                    Ok(result) => break Ok(result),
                    Err(e) => match e {
                        /* handle nonblocking IO */
                        Errno::EAGAIN => {
                            nonblocking_polls += 1;
                            if nonblocking_polls >= 3 {
                                eprintln!(
                                    "Sendmsg() not ready after retries when sending socket to: {}",
                                    path
                                );
                                break Err(anyhow!(e));
                            }
                            eprintln!("Sendmsg() not ready, will try again in {:?}", 3);
                            thread::sleep(Duration::from_secs(3));
                        }
                        _ => break Err(anyhow!(e)),
                    },
                }
            }
        }
        Err(err) => Err(err),
    };
    let _ = nix::unistd::close(send_fd.into_raw_fd());
    result
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

    let addr: SockAddr = BIND_ADDR.parse::<SocketAddr>()?.into();
    s.set_nodelay(true)?;
    s.set_reuse_address(true)?;
    // 如果把 set_reuse_port 注释，快速连续运行，第二个请求读到0导致panic
    // 猜测：虽然从fd获得了listener，没走bind，但新进程的 tcplistener 内部bind不了监听地址，而reuseport能bind
    s.set_reuse_port(true).unwrap();
    s.bind(&addr).unwrap();
    s.listen(4096).unwrap();
    let listener: TcpListener = s.into();
    let fd = listener.as_raw_fd();
    let mut fds = Fds::new();
    fds.add(BIND_ADDR.into(), fd);
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    std::thread::spawn(move || {
        loop {
            let (s, addr) = match listener.accept() {
                Ok(x) => x,
                Err(err) => continue,
            };
            println!("accept first connection");
            let (lock, cvar) = &*pair;
            let g = lock.lock().unwrap();
            cvar.wait(g).unwrap();
            break;
        }
        println!("drop old listener");
        drop(listener)
    });
    // 第一个请求挂住
    std::thread::spawn(move || {
        connect_addr();
    });
    let j_h = std::thread::spawn(move || {
        for n in 0..5 {
            let mut stream = TcpStream::connect(BIND_ADDR).unwrap();
            println!("send {n} request, waiting for response");
            let mut s = vec![0; 1024];
            let n = stream.read(&mut s).unwrap();
            println!("{n} request done");
            assert_eq!(RESPONSE, &s[..n]);
        }
    });
    sleep(Duration::from_secs(1));

    let exe = current_exe().unwrap();
    let mut sub = std::process::Command::new(exe);
    sub.arg("--upgrade");
    let h = sub.spawn().unwrap();
    sleep(Duration::from_secs(1));

    fds.send_to_sock(UNIX_PATH).unwrap();
    // fork，将fd发到别的进程
    sleep(Duration::from_secs(2));
    let (lock, cvar) = &*pair2;
    cvar.notify_one();
    // 等第一个listener drop
    sleep(Duration::from_secs(1));
    j_h.join().unwrap();
    Ok(())
}
fn connect_addr() {
    let mut stream = TcpStream::connect(BIND_ADDR).unwrap();
    sleep(Duration::MAX);
}
struct Fds {
    map: HashMap<String, RawFd>,
}

impl Fds {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn add(&mut self, bind: String, fd: RawFd) {
        self.map.insert(bind, fd);
    }
    pub fn get(&self, bind: &str) -> Option<&RawFd> {
        self.map.get(bind)
    }
    pub fn serialize(&self) -> (Vec<String>, Vec<RawFd>) {
        self.map.iter().map(|(key, val)| (key.clone(), val)).unzip()
    }
    pub fn deserialize(&mut self, binds: Vec<String>, fds: Vec<RawFd>) {
        assert_eq!(binds.len(), fds.len());
        for (bind, fd) in binds.into_iter().zip(fds) {
            self.map.insert(bind, fd);
        }
    }
    pub fn send_to_sock<P>(&self, path: &P) -> anyhow::Result<usize>
    where
        P: ?Sized + NixPath + std::fmt::Display,
    {
        let (vec_addr, vec_fds) = self.serialize();
        let mut ser_buf: [u8; 2048] = [0; 2048];
        println!("send fd {:?}", vec_fds);
        let ser_key_size = serialize_vec_string(&vec_addr, &mut ser_buf);
        send_fds_to(vec_fds, &ser_buf[..ser_key_size], path)
    }
    pub fn get_from_sock<P>(&mut self, path: &P) -> anyhow::Result<()>
    where
        P: ?Sized + NixPath + std::fmt::Display,
    {
        let mut de_buf: [u8; 2048] = [0; 2048];
        let (fds, bytes) = get_fds_from(path, &mut de_buf)?;
        let keys = deserialize_vec_string(&de_buf[..bytes])?;
        self.deserialize(keys, fds);
        Ok(())
    }
}

fn serialize_vec_string(vec_string: &[String], mut buf: &mut [u8]) -> usize {
    // There are many ways to do this. Serde is probably the way to go
    // But let's start with something simple: space separated strings
    let joined = vec_string.join(" ");
    // TODO: check the buf is large enough
    buf.write(joined.as_bytes()).unwrap()
}

fn deserialize_vec_string(buf: &[u8]) -> anyhow::Result<Vec<String>> {
    let joined = std::str::from_utf8(buf)?;
    Ok(joined.split_ascii_whitespace().map(String::from).collect())
}
