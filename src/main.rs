mod cli;
mod scanner;
mod output;

use clap::Parser;
use cli::{Cli, parse_ports};
use scanner::scan_all;
use output::{print_header, print_result, print_summary};

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let ports = parse_ports(&args.ports);
    let total = ports.len();

    print_header(&args.host, total);

    let results = scan_all(&args.host, ports, args.timeout).await;

    for result in &results {
        print_result(&result, args.verbose);
    }

    print_summary(&results);
}