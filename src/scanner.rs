use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Debug)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
}

#[derive(Debug)]
pub struct ScanResult {
    pub port: u16,
    pub state: PortState,
}

pub async fn scan_port(host: &str, port: u16, timeout_ms: u64) -> ScanResult {
    // Assembles the "host:port" address and resolves the DNS.
    let addr_str = format!("{}:{}", host, port);

    let addr = match addr_str.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(a) => a,
            None => return ScanResult { port, state: PortState::Filtered },
        },
        Err(_) => return ScanResult { port, state: PortState::Filtered },
    };

    // Attempts to connect within the timeout
    match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(addr)).await {
        Ok(Ok(_))  => ScanResult { port, state: PortState::Open },
        Ok(Err(_)) => ScanResult { port, state: PortState::Closed },
        Err(_)     => ScanResult { port, state: PortState::Filtered },
    }
}

pub async fn scan_all(
    host: &str,
    ports: Vec<u16>,
    timeout_ms: u64,
) -> Vec<ScanResult> {
    // Creates an asynchronous task for each port simultaneously
    let tasks: Vec<_> = ports
        .into_iter()
        .map(|port| {
            let host = host.to_string();
            tokio::spawn(async move {
                scan_port(&host, port, timeout_ms).await
            })
        })
        .collect();

    // Awaits all tasks to complete and collects the results
    let mut results = Vec::new();
    for task in tasks {
        if let Ok(result) = task.await {
            results.push(result);
        }
    }

    // Sorts by port number
    results.sort_by_key(|r| r.port);
    results
}