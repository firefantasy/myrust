use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use byteorder::{ByteOrder, BigEndian};
use std::sync::{Arc};

#[derive(Debug, Default)]
pub struct RequestResponse {
    pub serial: u32,
    pub payload: String
}

#[inline]
pub async fn read_from(lock: Arc<Mutex<TcpStream>>) -> RequestResponse {
    let mut conn = lock.lock().await;
    let mut buf = vec![0;4];
    conn.read(&mut buf).await.unwrap();
    let length = BigEndian::read_u32(&mut buf) as usize;
    conn.read(&mut buf).await.unwrap();
    let serial = BigEndian::read_u32(&mut buf);
    let mut buf = vec![0;length-4];
    conn.read(&mut buf).await.unwrap();
    let payload = std::str::from_utf8(&buf[..]).unwrap();
    return RequestResponse{serial: serial, payload: payload.to_string()};
}

#[inline]
pub async fn write_to(r: RequestResponse, lock: Arc<Mutex<TcpStream>>) {
    let mut conn = lock.lock().await;
    let payload_bytes = r.payload.as_bytes();
    let mut serial_bytes = [0;4];
    BigEndian::write_u32(&mut serial_bytes, r.serial);
    let length =  (payload_bytes.len() + 4) as u32;
    let mut length_byte = [0;4];
    BigEndian::write_u32(&mut length_byte, length);
    conn.write(&length_byte).await.unwrap();
    conn.write(&serial_bytes).await.unwrap();
    conn.write(&payload_bytes).await.unwrap();
}


#[inline]
pub async fn read_from_v2(conn: &mut TcpStream) -> RequestResponse {
    let mut buf = vec![0;4];
    conn.read(&mut buf).await.unwrap();
    let length = BigEndian::read_u32(&mut buf) as usize;
    conn.read(&mut buf).await.unwrap();
    let serial = BigEndian::read_u32(&mut buf);
    let mut buf = vec![0;length-4];
    conn.read(&mut buf).await.unwrap();
    let payload = std::str::from_utf8(&buf[..]).unwrap();
    return RequestResponse{serial: serial, payload: payload.to_string()};
}

#[inline]
pub async fn write_to_v2(r: RequestResponse, conn: &mut TcpStream) {
    let payload_bytes = r.payload.as_bytes();
    let mut serial_bytes = [0;4];
    BigEndian::write_u32(&mut serial_bytes, r.serial);
    let length =  (payload_bytes.len() + 4) as u32;
    let mut length_byte = [0;4];
    BigEndian::write_u32(&mut length_byte, length);
    conn.write(&length_byte).await.unwrap();
    conn.write(&serial_bytes).await.unwrap();
    conn.write(&payload_bytes).await.unwrap();
}
