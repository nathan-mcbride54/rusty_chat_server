use tokio::{
    io::{AsyncBufReadExt,  AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

const  MAX_CLIENTS: usize = 10;

#[tokio::main]
async fn main() {

    // Create TCP Listener and bind to port 8080. Await connections, and unwrap content.
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let(tx, _rx) = broadcast::channel(MAX_CLIENTS);

    loop { // New connection!

        // Create a socket that accepts connections on our TCP listener.
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {  // Spawn a new thread to run the below code.

            // Split the socket into Read / Write components
            let (reader, mut writer) = socket.split();

            // Create a new buffer reader, and a line to serve as the string buffer.
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {

                        if result.unwrap() == 0 { break; }

                        tx.send((line.clone(), addr)).unwrap();

                        line.clear();
                    }
                    result = rx.recv() => {

                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
