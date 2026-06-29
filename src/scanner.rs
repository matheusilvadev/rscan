use std::net::{IpAddr, ToSocketAddrs, SocketAddr};
use std::time::Duration;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::sync::Semaphore;

// Limits concurrent scans to avoid exhausting system file descriptors.
const MAX_CONCURRENT_SCANS: usize = 1024;

#[derive(Debug, Clone, Copy)]
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

/// Scans a single TCP port on a resolved IP address.
pub async fn scan_port(ip: IpAddr, port: u16, timeout_ms: u64) -> ScanResult {
    let socket_addr = SocketAddr::new(ip, port);

    match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(socket_addr)).await {
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
    // Resolve the target host once before scanning.
    let target_ip = match format!("{}:80", host).to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr.ip(),
            None => {
                eprintln!("Erro: Não foi possível resolver o host {}", host);
                return Vec::new();
            }
        },
        Err(e) => {
            eprintln!("Erro ao resolver DNS de {}: {}", host, e);
            return Vec::new();
        }
    };

    // Use a semaphore to bound asynchronous concurrency.
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_SCANS));
    let mut tasks = Vec::new();

    for port in ports {
        let sem = Arc::clone(&semaphore);

        tasks.push(tokio::spawn(async move {
            // Acquire a permit before starting a new scan.
            let _permit = sem.acquire_owned().await.unwrap();

            scan_port(target_ip, port, timeout_ms).await
        }));
    }

    // Collect scan results after all tasks complete.
    let mut results = Vec::new();
    for task in tasks {
        if let Ok(result) = task.await {
            results.push(result);
        }
    }

    // Sort results by port number for consistent output.
    results.sort_by_key(|r| r.port);
    results
}