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

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    x().await
}
