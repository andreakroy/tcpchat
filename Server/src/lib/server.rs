use super::message::*;
use chrono::{ DateTime, Local };
use futures::Future;
use futures::channel::mpsc::{ channel, Receiver, Sender };
use futures::stream::{ FuturesUnordered, Stream, StreamExt };
use std::io::{ BufRead, BufReader, BufWriter, Write, Result };
use tokio::net::{ SocketAddr, TcpListener, TcpStream };
use std::thread;

const BUFFER_SIZE : usize = 10;

pub struct Server {
    listener: TcpListener,
    connections: FuturesUnordered<Client>,
}

pub struct Client {
    address: SocketAddr,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Client {
    pub fn broadcast(&mut self, message: Message) -> Result<()> {
        self.writer.write(message.text.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn poll(&mut self) {
        return
    }
}

impl Server {
    pub fn new(address: SocketAddr) -> Result<Server> {
        Ok(Server {
            listener: TcpListener::bind(address)?,
            connections: FuturesUnordered::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let mut cli = self.listen().await?;
        }
        Ok(())
    }
    
    async fn listen(&mut self) -> Result<Client> {
        match self.listener.accept() {
            Ok((stream, address)) => {
                println!("{}", address);
                let sc = stream.try_clone()?;
                Ok(Client {
                    address,
                    reader: BufReader::new(stream),
                    writer: BufWriter::new(sc)
                })
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    }
