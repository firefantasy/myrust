
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Mutex, Arc};
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::prelude::*;
use std::str;
use std::net::TcpStream;
use byteorder::{ByteOrder, BigEndian};
use duplex_talk::{config::*, dialogue::*};

fn main() {
    struct RequestResponse {
        serial: u32,
        payload: String
    }

    fn zhangdaye_listen(mut read_sock: TcpStream, write_lock: Arc<Mutex<TcpStream>>) {
        let mut rev_count = 0;
        loop  {
            if rev_count >= LISTENTOTAL {
                break
            }
            let r = read_from(&mut read_sock);
            if r.payload  == L2 {
                // let conn = Arc::clone(&write_lock);
                // thread::spawn(move||write_to(RequestResponse{serial:r.serial, payload:Z3.to_string()}, conn));
                let lock = Arc::clone(&write_lock);
                write_to(RequestResponse{serial:r.serial, payload:Z3.to_string()}, lock)
            } else if r.payload == L4 {
                // let conn = Arc::clone(&write_lock);
                // thread::spawn(move||write_to(RequestResponse{serial:r.serial, payload:Z5.to_string()}, conn));
                let lock = Arc::clone(&write_lock);
                write_to(RequestResponse{serial:r.serial, payload:Z5.to_string()}, lock)
            } else if r.payload == L1 {
            } else {
                println!("张大爷听不懂： {}", r.payload);
                break
            }
            rev_count += 1;
        }
        // println!("zhangdaye_listen ok")
    }


    fn zhangdaye_say(write_lock: Arc<Mutex<TcpStream>>) {
        let mut next_serial: u32 = 0;
        for _ in 0..SAYTOTAL {
            let conn = Arc::clone(&write_lock);
            write_to(RequestResponse{serial: next_serial, payload: Z0.to_string()}, conn);
            next_serial += 1;
        }
        // println!("zhangdaye_say ok")
    }

    fn lidaye_listen(mut conn: TcpStream, write_lock: Arc<Mutex<TcpStream>>) {
        let mut rev_count = 0;

        loop {
            if rev_count >= LISTENTOTAL {
                break
            }
            let r = read_from(&mut conn);
            if r.payload == Z0 {
                let write_conn = Arc::clone(&write_lock);
                // thread::spawn(move || write_to(RequestResponse{serial: r.serial, payload: L1.to_string()}, write_conn));
                write_to(RequestResponse{serial: r.serial, payload: L1.to_string()}, write_conn);
            } else if r.payload == Z3.to_string() {
                // do nothing
            } else if r.payload == Z5.to_string() {
                // do nothing
            } else {
                println!("李大爷听不懂: {}", r.payload);
                break
            }
            rev_count += 1;
        }
        // println!("lidaye_listen ok")
    }

    fn lidaye_say(write_lock: Arc<Mutex<TcpStream>>) {
        let mut next_serial: u32 = 0;
        for _ in 0..SAYTOTAL {
            // let conn = Arc::clone(&write_lock);
            // write_to(RequestResponse{serial: next_serial, payload:L2.to_string()}, conn);
            // next_serial += 1;
            // let conn = Arc::clone(&write_lock);
            // write_to(RequestResponse{serial: next_serial, payload:L4.to_string()}, conn);
            // next_serial += 1;
            let conn = Arc::clone(&write_lock);
            let conn_1 = Arc::clone(&write_lock);
            let serial = next_serial + 1;
            let serial_1 = next_serial + 2;   
            next_serial += 2;
            thread::spawn(move|| {
                write_to(RequestResponse{serial: serial, payload:L2.to_string()}, conn);
                write_to(RequestResponse{serial: serial_1, payload:L4.to_string()}, conn_1);
            });
        }
        // println!("lidaye_say ok")
    }

    #[inline]
    fn write_to(r: RequestResponse, lock: Arc<Mutex<TcpStream>>) {
        let mut conn = lock.lock().unwrap();
        let payload_bytes = r.payload.as_bytes();
        let mut serial_bytes = [0;4];
        BigEndian::write_u32(&mut serial_bytes, r.serial);
        let length =  (payload_bytes.len() + 4) as u32;
        let mut length_byte = [0;4];
        BigEndian::write_u32(&mut length_byte, length);
        conn.write(&length_byte).unwrap();
        conn.write(&serial_bytes).unwrap();
        conn.write(&payload_bytes).unwrap();
    }

    #[inline]
    fn read_from(conn: &mut TcpStream) -> RequestResponse {
        let mut buf = vec![0;4];
        conn.read(&mut buf).unwrap();
        let length = BigEndian::read_u32(&mut buf) as usize;
        conn.read(&mut buf).unwrap();
        let serial = BigEndian::read_u32(&mut buf);
        let mut buf = vec![0;length-4];
        conn.read(&mut buf).unwrap();
        let payload = str::from_utf8(&buf[..]).unwrap();
        return RequestResponse{serial: serial, payload: payload.to_string()};
    }


    fn start_server() {
        let loopback = Ipv4Addr::new(127, 0, 0, 1);
        let socket = SocketAddrV4::new(loopback, 9999);
        let listener = TcpListener::bind(socket).unwrap();
        println!("张大爷在胡同口等着 ...");
        // let t: thread::JoinHandle<()> = thread::spawn(||{});
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("碰见一个李大爷: {}", stream.peer_addr().unwrap());
                    let write_stream = stream.try_clone().unwrap();
                    let write_lock = Arc::new(Mutex::new(write_stream));
                    let write_lock_1 = Arc::clone(&write_lock);
                    let t = thread::spawn(move|| {
                        zhangdaye_listen(stream, write_lock)
                    });
                    zhangdaye_say(write_lock_1);
                    t.join().unwrap();
                    return
                    // return t;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
    }

    fn start_client() {
        let stream = TcpStream::connect("127.0.0.1:9999").unwrap();
        let write_stream = stream.try_clone().unwrap();
        let write_lock = Arc::new(Mutex::new(write_stream));
        let write_conn = Arc::clone(&write_lock);
        let t = thread::spawn(move || {
            lidaye_listen(stream, write_lock);
        });
        lidaye_say(write_conn);
        t.join().unwrap();
    }
    let t = thread::spawn(||start_server());
    thread::sleep(Duration::from_secs(1));
    let now = Instant::now();
    start_client();
    t.join().unwrap();
    println!("耗时：{:?}", now.elapsed());
}
