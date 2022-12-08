use std::error::Error;
use std::sync::{Arc};
use std::time::{Instant};
use tokio::net::TcpStream;
use tokio::task;
use tokio::sync::Mutex;
use duplex_talk::{config::*, dialogue::*, async_helper::*};

#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
async fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let lock = Arc::new(Mutex::new(stream));

    let w = Arc::clone(&lock);
    let t = task::spawn(async move {
        lidaye_listen(w).await; 
    });
    lidaye_say(lock).await;
    // thread::sleep(Duration::from_secs(10));
    t.await.unwrap();
    println!("耗时：{:?}", now.elapsed());
    Ok(())
}

pub async fn start_client() {
    let now = Instant::now();
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let lock = Arc::new(Mutex::new(stream));

    let w = Arc::clone(&lock);
    let t = task::spawn(async move {
        lidaye_listen(w).await; 
    });
    lidaye_say(lock).await;
    // thread::sleep(Duration::from_secs(10));
    t.await.unwrap();
    println!("耗时：{:?}", now.elapsed());
}

async fn lidaye_listen(conn: Arc<Mutex<TcpStream>>) {
    let mut rec_count = 0;
    println!("lidaye listen start");
    loop {
        if rec_count >= LISTENTOTAL {
            break
        }
        // let r = Arc::clone(&rlock);
        let w = Arc::clone(&conn);
        // println!("read start"); 
        let data = read_from(w).await;  
        if data.payload == Z0 {
            let w = Arc::clone(&conn);
            task::spawn(async move {
                write_to(RequestResponse{serial: data.serial, payload:L1.to_string()}, w).await;
            });
            // write_to(RequestResponse{serial: data.serial, payload:L1.to_string()}, w).await;
        } else if data.payload == Z3 { 
        } else if data.payload == Z5 {
        } else {
            println!("李大爷听不懂： {}", data.payload);
            break
        }
        rec_count += 1;
        // task::yield_now().await;
        // println!("reve total: {}", rec_count); 
    }
}

async fn lidaye_say(conn: Arc<Mutex<TcpStream>>) {
    let mut next_serial: u32 = 0;
    println!("lidaye_say start");
    let now = Instant::now();
    for _ in 0..SAYTOTAL {
        let w = Arc::clone(&conn);
        let w1 = Arc::clone(&conn);
        task::spawn(async move {
            write_to(RequestResponse{serial: next_serial, payload: L2.to_string()}, w).await;
            write_to(RequestResponse{serial: next_serial, payload: L4.to_string()}, w1).await;
        });
        next_serial += 1;
        // println!("say toal: {}", next_serial);
    }
    println!("lidaye say finish, use {:?}", now.elapsed());
}