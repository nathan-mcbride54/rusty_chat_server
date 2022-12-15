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

    let(tx, _rx) = broadcast::channel::<String>(MAX_CLIENTS);

    loop { // New connection!

        // Create a socket that accepts connections on our TCP listener.
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {  // Spawn a new thread to run the below code.

            // Split the socket into Read / Write components
            let (sock_read, mut sock_write) = socket.split();

            // Create a new buffer reader, and a line to serve as the string buffer.
            let mut reader = BufReader::new(sock_read);
            let mut line = String::new();

            loop { // The R/W loop
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break; // Nothing to read
                }

                tx.send(line.clone()).unwrap();

                let msg = rx.recv().await.unwrap();

                sock_write.write_all(msg.as_bytes()).await.unwrap();
                line.clear(); // Empty the buffer
            }
        });
    }
}
