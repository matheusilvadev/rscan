use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rscan",
    about = "A simple Rust port scanner",
)]
pub struct Cli {
    /// Target IP host (ex: 192.168.1.1 or google.com)
    pub host: String,

    /// Port range to scan (ex: 80, 1-1024, 22, 80, 443)
    #[arg(short, long, default_value = "1-1024")]
    pub ports: String,

    /// Timeout per port in milliseconds
    #[arg(short, long, default_value_t = 500)]
    pub timeout: u64,

    /// It also displays closed and filtered ports
    #[arg(short, long)]
    pub verbose: bool,

}

pub fn parse_ports(ports_str: &str) -> Vec<u16> {
    let mut  ports = Vec::new();

    for part in ports_str.split(',') {
        let part = part.trim();

        if part.contains('-') {
            // Range "1-1024"
            let bounds: Vec<&str> = part.splitn(2, '-').collect();
            if let (Ok(start), Ok(end)) = (
                bounds[0].parse::<u16>(),
                bounds[1].parse::<u16>(),
            ) {
                ports.extend(start..=end);
            }
        } else {

            // Single port "80"
            if let Ok(p) = part.parse::<u16>() {
                ports.push(p);
            }
        }
    }
    ports
}