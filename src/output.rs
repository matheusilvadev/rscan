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
        _    => "",
    }
}

pub fn print_header(host: &str, total: usize) {
    println!("{}", "─".repeat(45));
    println!("  Host:   {}", host.bold());
    println!("  Portas: {}", total);
    println!("{}", "─".repeat(45));
    println!("{:<10} {:<12} {}", "PORTA", "ESTADO", "SERVIÇO");
    println!("{}", "─".repeat(45));
}

pub fn print_result(result: &ScanResult, verbose: bool) {
    let port_str = format!("{}/tcp", result.port);
    let service  = service_name(result.port);

    match result.state {
        PortState::Open => {
            println!(
                "{:<10} {:<12} {}",
                port_str,
                "open".green().bold(),
                service.cyan()
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
        _ => {} // silencioso sem --verbose
    }
}

pub fn print_summary(results: &[ScanResult]) {
    let open = results.iter()
        .filter(|r| matches!(r.state, PortState::Open))
        .count();

    println!("{}", "─".repeat(45));
    println!("  {} porta(s) aberta(s) encontrada(s)", open.to_string().green().bold());
    println!("{}", "─".repeat(45));
}