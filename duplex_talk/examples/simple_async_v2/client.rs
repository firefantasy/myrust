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
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let lock = Arc::new(Mutex::new(stream));

    let w = Arc::clone(&lock);
    let t = task::spawn(async move {
        lidaye_listen(w).await; 
    });
    let now = Instant::now();
    lidaye_say(lock).await;    
    // thread::sleep(Duration::from_secs(10));
    t.await.unwrap();
    println!("耗时：{:?}", now.elapsed());
}

async fn lidaye_listen(lock: Arc<Mutex<TcpStream>>) {
    let mut rev_count = 0;
    println!("lidaye listen start");
    loop {
        if rev_count >= LISTENTOTAL {
            break
        }
        // let r = Arc::clone(&rlock);
        let mut conn = lock.lock().await;
        // println!("read start"); 
        let data = read_from_v2(&mut conn).await;  
        if data.payload == Z0 {
            write_to_v2(RequestResponse{serial: data.serial, payload:L1.to_string()}, &mut conn).await;
            // write_to(RequestResponse{serial: data.serial, payload:L1.to_string()}, w).await;
        } else if data.payload == Z3 { 
        } else if data.payload == Z5 {
        } else {
            println!("李大爷听不懂： {}", data.payload);
            break
        }
        rev_count += 1;
    }
}

async fn lidaye_say(lock: Arc<Mutex<TcpStream>>) {
    let mut next_serial: u32 = 0;
    println!("lidaye_say start");
    let now = Instant::now();
    for _ in 0..SAYTOTAL {
        // let new_lock = Arc::clone(&lock);
        // task::spawn(async move {
        //     let mut conn = new_lock.lock().await;
        //     write_to_v2(RequestResponse{serial: next_serial, payload: L2.to_string()}, &mut conn).await;
        //     write_to_v2(RequestResponse{serial: next_serial, payload: L4.to_string()}, &mut conn).await;
        // });
        let mut conn = lock.lock().await;
        write_to_v2(RequestResponse{serial: next_serial, payload: L2.to_string()}, &mut conn).await;
        write_to_v2(RequestResponse{serial: next_serial, payload: L4.to_string()}, &mut conn).await;
        next_serial += 1;
        // println!("say toal: {}", next_serial);
    }
    println!("lidaye say finish, use {:?}", now.elapsed());
}