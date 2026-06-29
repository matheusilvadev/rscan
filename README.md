<h1 align="center" style="font-weight: bold;"># rscan 💻</h1>

<p align="center">
  <strong>A high-performance asynchronous TCP port scanner written in Rust.</strong><br>
  Built with Tokio, featuring bounded concurrency, DNS resolution caching, and opportunistic service banner detection.
</p>

---

## Overview

**rscan** is a lightweight command-line TCP port scanner developed in Rust to explore asynchronous network programming and systems programming concepts.

The scanner leverages Tokio's asynchronous runtime to efficiently scan large port ranges while maintaining stable resource usage through bounded concurrency. It also performs DNS resolution only once per target and attempts to capture service banners from open ports whenever available.

---

## Features

* ⚡ Asynchronous TCP port scanning
* 🌐 Single DNS resolution per target host
* 🔒 Bounded concurrency using Tokio semaphores
* 🏷️ Opportunistic service banner detection
* 🎨 Colorized terminal output
* ⚙️ Configurable timeout and port ranges
* 📦 Lightweight and dependency-efficient

---

## Technologies

* **Rust**
* **Tokio**
* **Clap v4**
* **Colored**

---

## Getting Started

### Prerequisites

Install the following tools before building the project:

* Rust & Cargo
* Git

### Clone the repository

```bash
git clone git@github.com:matheusilvadev/rscan.git
cd rscan
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run -- <host>
```

Example:

```bash
cargo run -- scanme.nmap.org
```

---

## Command-Line Options

| Argument    | Short | Description                             |  Default |
| ----------- | :---: | --------------------------------------- | :------: |
| `<host>`    |   —   | Target hostname or IP address           | Required |
| `--ports`   |  `-p` | Port range or comma-separated port list | `1-1024` |
| `--timeout` |  `-t` | Timeout per port in milliseconds        |   `500`  |
| `--verbose` |  `-v` | Display closed and filtered ports       |  `false` |

---

---

## Project Structure

```text
src/
├── main.rs        # Application entry point
├── cli.rs         # Command-line argument parsing
├── scanner.rs     # Asynchronous scanning engine
├── output.rs      # Terminal output formatting
```

---

## How It Works

1. Resolves the target hostname into an IP address.
2. Spawns asynchronous scan tasks using the Tokio runtime.
3. Limits concurrent socket connections with a semaphore to prevent resource exhaustion.
4. Attempts to establish a TCP connection to each target port.
5. Classifies each port as **Open**, **Closed**, or **Filtered**.
6. Captures and displays a service banner when the remote service provides one.
7. Sorts and prints the scan results in a human-readable format.
---
## License

This project is available for educational and research purposes.
