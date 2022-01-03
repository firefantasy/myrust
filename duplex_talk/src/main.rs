
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Mutex, Arc};
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::prelude::*;
use std::str;
use std::net::TcpStream;
use byteorder::{ByteOrder, BigEndian};


const Z0: &str = "吃了没，您吶?";
const Z3: &str = "嗨！吃饱了溜溜弯儿。";
const Z5: &str  = "回头去给老太太请安！";
const L1: &str  = "刚吃。";
const L2: &str  = "您这，嘛去？";
const L4: &str = "有空家里坐坐啊。";

const SAYTOTAL: u32 = 100000;
const LISTENTOTAL: u32 = SAYTOTAL * 3;

fn main() {
    let liWriteLock: Mutex<u8> =  Mutex::new(0);    // 李大爷的写锁
    let zhangWriteLock: Mutex<u8> = Mutex::new(0); // 张大爷的写锁

    struct RequestResponse {
        Serial: u32,
        Payload: String
    }


    fn zhangDaYeListen(conn: &mut TcpStream, zhangWriteLock: Arc<Mutex<u8>>) {
        let mut zRecvCount = 0;
        
        loop  {
            if zRecvCount >= LISTENTOTAL {
                break
            }
            let r = readFrom(conn);
            let c_mutex = Arc::clone(&zhangWriteLock);
            let mut first_stream = conn.try_clone().unwrap();
            if r.Payload  == L2 {
                writeTo(RequestResponse{Serial:r.Serial, Payload:Z3.to_string()}, 
                    conn, c_mutex)
            } else if r.Payload == L4 {
                writeTo(RequestResponse{Serial:r.Serial, Payload:Z5.to_string()}, 
                    conn, c_mutex)
            } else if r.Payload == L1 {

            } else {
                println!("张大爷听不懂： {}", r.Payload);
                break
            }
            zRecvCount += 1;
        }
    }


    fn zhangDaYeSay(conn: &mut TcpStream, zhangWriteLock: Arc<Mutex<u8>>) {
        let mut nextSerial: u32 = 0;
        for i in 0..SAYTOTAL {
            let c_mutex = Arc::clone(&zhangWriteLock);
            let mut first_stream = conn.try_clone().unwrap();
            writeTo(RequestResponse{Serial: nextSerial, Payload: Z0.to_string()}, 
                conn, c_mutex);
            nextSerial += 1;
        }
    }

    fn liDaYeListen(conn: &mut TcpStream, liWriteLock: Arc<Mutex<u8>>) {
        let mut lRecvCount = 0;

        loop {
            if lRecvCount >= LISTENTOTAL {
                break
            }
            let r = readFrom(conn);
            let c_mutex = Arc::clone(&liWriteLock);
            if r.Payload == Z0 {
                writeTo(RequestResponse{Serial: r.Serial, Payload: L1.to_string()}, 
                    conn, c_mutex)
            } else if r.Payload == Z3.to_string() {
                // do nothing
            } else if r.Payload == Z5.to_string() {
                // do nothing
            } else {
                println!("李大爷听不懂: {}", r.Payload);
                break
            }
            lRecvCount += 1;
        }
    }

    fn liDaYeSay(conn: &mut TcpStream, liWriteLock: Arc<Mutex<u8>>) {
        let mut nextSerial: u32 = 0;
        for _ in 0..SAYTOTAL {
            let c_mutex = Arc::clone(&liWriteLock);
            let mut first_stream = conn.try_clone().unwrap();
            thread::spawn(move || {writeTo(RequestResponse{Serial: nextSerial, Payload:L2.to_string()}, 
            &mut first_stream, c_mutex)});
            nextSerial += 1;
            let c_mutex = Arc::clone(&liWriteLock);
            let mut first_stream = conn.try_clone().unwrap();
            thread::spawn(move || {writeTo(RequestResponse{Serial: nextSerial, Payload:L4.to_string()}, 
            &mut first_stream, c_mutex)});
            nextSerial += 1;
        }
    }


    fn writeTo<T>(r: RequestResponse, conn: &mut TcpStream, lock: Arc<Mutex<T>>) {
        let mut guard = match lock.lock() {
            Ok(guard) => {
                let payload_bytes = r.Payload.as_bytes();
                let mut serial_bytes = [0;4];
                BigEndian::write_u32(&mut serial_bytes, r.Serial);
                let length =  (payload_bytes.len() + 4) as u32;
                let mut length_byte = [0;4];
                BigEndian::write_u32(&mut length_byte, length);
                match conn.write(&length_byte) {
                    Ok(_) =>(
                        // println!("write length ok")
                    ),
                    Err(e) => (
                        println!("write length faild: {}", e)
                    )
                };
                match conn.write(&serial_bytes) {
                    Ok(_) =>(
                        // println!("write serial ok")
                    ),
                    Err(e) => (
                        println!("write serial faild: {}", e)
                    )
                };
                match conn.write(&payload_bytes){
                    Ok(_) =>(
                        // println!("write content ok")
                    ),
                    Err(e) => (
                        println!("write content faild: {}", e)
                    )
                };
            },
            Err(poisoned) => {
                println!("server accept client incoming failed")
            }
        };
    }

    fn readFrom(conn: &mut TcpStream) -> RequestResponse {
        let mut buf = vec![0;4];
        match conn.read(&mut buf){
            Ok(_) =>(
                // println!("read length ok")
            ),
            Err(e) => (
                println!("read length faild: {}", e)
            )
        };
        let length = BigEndian::read_u32(&mut buf) as usize;
        match conn.read(&mut buf) {
            Ok(_) =>(
                // println!("read serail ok")
            ),
            Err(e) => (
                println!("read serial faild: {}", e)
            )
        };
        let serial = BigEndian::read_u32(&mut buf);
        let mut buf = vec![0;length-4];
        match conn.read(&mut buf) {
            Ok(_) =>(
                // println!("read content ok")
            ),
            Err(e) => (
                println!("read content faild: {}", e)
            )
        };
        let payload = str::from_utf8(&buf[..]).unwrap();
        // println!("read from payload: {}", payload);
        return RequestResponse{Serial: serial, Payload: payload.to_string()};
    }


    fn startServer(zhangWriteLock: Mutex<u8>) -> thread::JoinHandle<()> {
        let loopback = Ipv4Addr::new(127, 0, 0, 1);
        let socket = SocketAddrV4::new(loopback, 9999);
        let listener = TcpListener::bind(socket).unwrap();
        println!("张大爷在胡同口等着 ...");
        let t: thread::JoinHandle<()> = thread::spawn(||{});
        let mutex = Arc::new(zhangWriteLock);
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("碰见一个李大爷: {}", stream.peer_addr().unwrap());
                    let mut first_stream = stream.try_clone().unwrap();
                    let a_mutex = Arc::clone(&mutex);
                    let t = thread::spawn(move|| {
                        zhangDaYeListen(&mut first_stream, a_mutex)
                    });

                    let a_mutex = Arc::clone(&mutex);
                    thread::spawn(move|| {
                        zhangDaYeSay(&mut stream, a_mutex)
                    });
                    return t;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
        t
    }

    fn startClient(liWriteLock: Mutex<u8>) -> thread::JoinHandle<()> {
        let mut stream = TcpStream::connect("127.0.0.1:9999").unwrap();
        let mut first_stream = stream.try_clone().unwrap();
        let mutex = Arc::new(liWriteLock);
        let a_mutex = Arc::clone(&mutex);
        let t = thread::spawn(move || {
            liDaYeListen(&mut first_stream, a_mutex);
        });
        let a_mutex = Arc::clone(&mutex);
        thread::spawn(move|| {
            liDaYeSay(&mut stream, a_mutex)
        });
        t
    }

    let t = thread::spawn(||{startServer(zhangWriteLock).join()});
    thread::sleep(Duration::from_secs(1));
    let t1 = thread::spawn(||startClient(liWriteLock).join());
    let now = Instant::now();
    t.join().unwrap();
    t1.join().unwrap();
    // thread::sleep(Duration::from_secs(1));
    println!("耗时：{:?}", now.elapsed());
}
