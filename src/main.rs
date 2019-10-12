use tokio::prelude::*;
use tokio::net::{UdpSocket, UdpFramed};
use async_std::sync::Arc;
use std::borrow::BorrowMut;
use std::sync::Mutex;
use std::cell::RefCell;
use tokio::sync::mpsc::{channel, Sender};
use futures::future::join;
use std::net::SocketAddr;

async fn x() -> Result<(), Box<dyn std::error::Error>> {
    let (mut sender, mut receiver) = channel::<(Vec<u8>, SocketAddr)>(1000);


    let mut server = UdpSocket::bind("127.0.0.1:6000").await?;
    let (mut rx, mut tx) = server.split();
    println!("UDP listen on 0.0.0.0:6000");


    tokio::spawn(async move {
        loop {
            let data = receiver.recv().await;
            if let Some(x) = data {

                let (buffer, peer) = x;
                println!("receive {} {:?} ", peer, buffer);
                tx.send_to(&buffer[..], &peer).await.unwrap();
            }
        }
    });

    loop {
        println!("waiting for message");
        let mut buffer = vec![0u8; 1024];
        let result = rx.recv_from(&mut buffer).await;
        match result {
            Ok(data) => {
                println!("get {} {}", data.0, data.1);
                let mut sender_arc = sender.clone();

                tokio::spawn(request_handler(sender_arc, buffer, data));

            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
}

async fn request_handler(sender: Sender<(Vec<u8>, SocketAddr)>, buffer: Vec<u8>, data:(usize, SocketAddr)) {
//    let tid: [_;2]=buffer[0..2];
    let flags: u16 = u16::from_be_bytes([buffer[2], buffer[3]]);
    let questions: u32 = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
//    let flags: u16 = ((buffer[2] as u16) << 8) & (buffer[3] as u16);
    if (flags & 0xF900 ) == 0x100 && questions == 0x00010000 {
        let type_index = find_type(buffer).unwrap_or(2);
        dbg!(type_index);
    }
    dbg!(flags);
}

fn find_type(buffer: Vec<u8>) -> Option<i32> {
    let mut i :i32 =12;
    loop {
        let option = buffer.get(i as usize)?;
        if *option == 0 {
            return Some(i+1);
        }else {
            i += *option as i32 + 1
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    x().await
}
