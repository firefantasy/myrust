use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::task;
use std::sync::{Arc};
use std::time::{Instant};
use duplex_talk::{config::*, dialogue::*, async_helper::*};


#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, addr)= listener.accept().await?;
        println!("碰见一个李大爷：{}", addr);
        tokio::spawn(async move {
            process_socket(socket).await;
        });
    }
    // let (mut socket, addr)= listener.accept().await?;
    // println!("碰见一个李大爷：{}", addr); 
    // process_socket(&mut socket).await;
    // Ok(())
    // start_server().await;
    // Ok(())
}

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (socket, addr)= listener.accept().await.unwrap();
    println!("碰见一个李大爷：{}", addr);
    process_socket(socket).await;
}

async fn zhangdaye_listen(conn: Arc<Mutex<TcpStream>>) {
    let mut rev_count = 0;
    println!("zhangdaye_listen start !!!!!!!");
    
    loop  {
        if rev_count >= LISTENTOTAL {
            break
        }
        let w = Arc::clone(&conn);

        let data = read_from(w).await; 
        if data.payload == L2 {
            let w = Arc::clone(&conn);
            task::spawn(async move {
                write_to(RequestResponse{serial: data.serial, payload:Z3.to_string()}, w).await;
            });

            // write_to(RequestResponse{serial: data.serial, payload:Z3.to_string()}, w).await;
        } else if data.payload == L4 {
            let w = Arc::clone(&conn);
            task::spawn(async move {
                write_to(RequestResponse{serial: data.serial, payload:Z5.to_string()}, w).await
            });

            // write_to(RequestResponse{serial: data.serial, payload:Z5.to_string()}, w).await
        } else if data.payload == L1 {
        }
        else {
            println!("张大爷听着： {}", data.payload);
            break
        }
        rev_count += 1;
        // println!("zhangdaye_listen write !!!!!!!");        
    }
}

async fn zhangdaye_say(conn: Arc<Mutex<TcpStream>>) {
    let mut next_serial: u32= 0;
    println!("zhangdaye_say!!!!!!!");
    let now = Instant::now();
    for _ in 0..SAYTOTAL {
        let w = Arc::clone(&conn);
        tokio::spawn(async move {
            write_to(RequestResponse{serial: next_serial, payload:Z0.to_string()}, w).await;
        });
        next_serial += 1;
        // println!("say toal: {}", next_serial);
    }
    println!("zhangdaye_say 耗时：{:?}", now.elapsed());
}

async fn process_socket(conn: TcpStream) {
    let lock = Arc::new(Mutex::new(conn));
    let conn_lock = Arc::clone( &lock);
    let t = tokio::spawn(async move {
        zhangdaye_listen(conn_lock).await;

    });
    let conn_lock = Arc::clone( &lock);
    zhangdaye_say(conn_lock).await;
    t.await.unwrap();
    println!("process ok")
}