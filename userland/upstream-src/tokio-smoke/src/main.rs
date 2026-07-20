// Unmodified tokio on NONOS, socket path: a current-thread runtime opens a real
// async TCP connection through the mio net.sockets reactor to the host gateway
// (10.0.2.2 under QEMU user-net), sends an HTTP request, and reads the reply.
// This exercises tokio's I/O driver, TcpStream::connect (mio), the OP_POLL
// readiness path, and async read/write end to end. The connect retries because
// the network stack and DHCP take a few seconds to come up after boot.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

// Default target is the QEMU user-net host gateway, so `tokio-smoke` with no
// args proves the async path against the bundled test server at boot. Given
// `tokio-smoke <host> <port>` it fetches from any reachable endpoint.
const DEFAULT_HOST: &str = "10.0.2.2";
const DEFAULT_PORT: &str = "9000";

fn target() -> String {
    let mut a = std::env::args().skip(1);
    let host = a.next().unwrap_or_else(|| DEFAULT_HOST.to_string());
    let port = a.next().unwrap_or_else(|| DEFAULT_PORT.to_string());
    format!("{}:{}", host, port)
}

async fn once(host: &str, report: bool) -> bool {
    let mut stream = match TcpStream::connect(host).await {
        Ok(s) => s,
        Err(e) => {
            if report {
                println!("[TOKIO-SMOKE] connect error: {}", e);
            }
            return false;
        }
    };
    println!("[TOKIO-SMOKE] connected to {}", host);
    if stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await.is_err() {
        println!("[TOKIO-SMOKE] write failed");
        return false;
    }
    let mut buf = [0u8; 256];
    match stream.read(&mut buf).await {
        Ok(n) => {
            let line = core::str::from_utf8(&buf[..n])
                .unwrap_or("<binary>")
                .lines()
                .next()
                .unwrap_or("");
            println!("[TOKIO-SMOKE] read {} bytes, first line: {}", n, line);
            true
        }
        Err(_) => {
            println!("[TOKIO-SMOKE] read failed");
            false
        }
    }
}

fn main() {
    let host = target();
    println!("[TOKIO-SMOKE] start {}", host);

    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(_) => {
            println!("[TOKIO-SMOKE] runtime build failed");
            return;
        }
    };

    rt.block_on(async {
        for attempt in 0..30 {
            if once(&host, attempt % 5 == 0).await {
                println!("[TOKIO-SMOKE] done");
                return;
            }
            if attempt % 5 == 0 {
                println!("[TOKIO-SMOKE] waiting for net (attempt {})", attempt);
            }
            sleep(Duration::from_millis(1000)).await;
        }
        println!("[TOKIO-SMOKE] gave up (no net)");
    });
}
