use colored::Colorize;
use crate::scanner::{ScanResult, PortState};

fn service_name(port: u16) -> &'static str {
    match port {
        21   => "FTP",
        22   => "SSH",
        23   => "Telnet",
        25   => "SMTP",
        53   => "DNS",
        80   => "HTTP",
        110  => "POP3",
        143  => "IMAP",
        443  => "HTTPS",
        3306 => "MySQL",
        5432 => "PostgreSQL",
        6379 => "Redis",
        8080 => "HTTP-Alt",
        8443 => "HTTPS-Alt",
        _    => "unknown",
    }
}

pub fn print_header(host: &str, total: usize) {
    println!("{}", "─".repeat(60));
    println!("  Host:   {}", host.bold());
    println!("  Portas: {}", total);
    println!("{}", "─".repeat(60));
    println!("{:<10} {:<12} {}", "PORTA", "ESTADO", "SERVIÇO");
    println!("{}", "─".repeat(60));
}

pub fn print_result(result: &ScanResult, verbose: bool) {
    let port_str = format!("{}/tcp", result.port);
    let default_service = service_name(result.port);

    match &result.state {
        PortState::Open(maybe_banner) => {
            // Display the captured banner when available; otherwise fall back
            // to the default service name associated with the port.
            let service_display = match maybe_banner {
                Some(banner) => banner.yellow(),
                None => default_service.cyan(),
            };

            println!(
                "{:<10} {:<12} {}",
                port_str,
                "open".green().bold(),
                service_display
            );
        }
        PortState::Closed if verbose => {
            println!(
                "{:<10} {:<12}",
                port_str,
                "closed".red()
            );
        }
        PortState::Filtered if verbose => {
            println!(
                "{:<10} {:<12}",
                port_str,
                "filtered".yellow()
            );
        }
        _ => {}
    }
}

pub fn print_summary(results: &[ScanResult]) {
    let open = results.iter()
        .filter(|r| matches!(r.state, PortState::Open(_)))
        .count();

    println!("{}", "─".repeat(60));
    println!("  {} porta(s) aberta(s) encontrada(s)", open.to_string().green().bold());
    println!("{}", "─".repeat(60));
}