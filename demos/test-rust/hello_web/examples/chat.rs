use futures::SinkExt;
use std::{collections::HashMap, error::Error, net::SocketAddr, sync::Arc};
use std::{env, io};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
};

// https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("server running on {}", addr);

    let state = Arc::new(Mutex::new(Shared::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            tracing::debug!("accepted connection");
            if let Err(e) = process(state, stream, addr).await {
                tracing::info!("an error occurred; error = {:?}", e);
            }
        });
    }
}

async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    // feature: codec
    let mut lines = Framed::new(stream, LinesCodec::new());

    // use futures::SinkExt;  需要引入
    lines.send("Please enter your username:").await?;
    let username = match lines.next().await {
        Some(Ok(line)) => line,
        _ => {
            tracing::error!("Failed to get username from {}. Client disconnected.", addr);
            return Ok(());
        }
    };

    let mut peer = Peer::new(state.clone(), lines).await?;
    {
        let mut state = state.lock().await;
        let msg = format!("{} has joined the chat", username);
        tracing::info!("{}", msg);
        state.broadcast(addr, &msg).await;
    }

    loop {
        tokio::select! {
            // 收到其它连接的消息，发送给当前用户
            Some(msg) = peer.rx.recv() => {
                peer.lines.send(&msg).await?;
            }
            result = peer.lines.next() => match result {
                Some(Ok(msg)) => {
                    let mut state = state.lock().await;
                    let msg = format!("{}: {}", username, msg);

                    state.broadcast(addr, &msg).await;
                }
                Some(Err(e)) => {
                    tracing::error!(
                        "an error occurred while processing messages for {}; error = {:?}",
                        username,
                        e
                    );
                }
                None => break,
            }
        }
    }

    // 断开连接
    {
        let mut state = state.lock().await;
        state.peers.remove(&addr);
        let msg = format!("{} has left the chat", username);
        tracing::info!("{}", msg);
        state.broadcast(addr, &msg).await;
    }

    Ok(())
}

type Tx = mpsc::UnboundedSender<String>;

type Rx = mpsc::UnboundedReceiver<String>;

struct Shared {
    peers: HashMap<SocketAddr, Tx>,
}

impl Shared {
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }
    async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
        for peer in self.peers.iter_mut() {
            if *peer.0 != sender {
                let _ = peer.1.send(message.into());
            }
        }
    }
}

struct Peer {
    lines: Framed<TcpStream, LinesCodec>,
    rx: Rx,
}

impl Peer {
    async fn new(
        state: Arc<Mutex<Shared>>,
        lines: Framed<TcpStream, LinesCodec>,
    ) -> io::Result<Peer> {
        let addr = lines.get_ref().peer_addr()?;
        let (tx, rx) = mpsc::unbounded_channel();

        // 新的客户端连接，插入共享的状态
        state.lock().await.peers.insert(addr, tx);

        Ok(Self { lines, rx })
    }
}
