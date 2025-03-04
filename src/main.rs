use serde::Serialize;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct PingStatus {
    text: String,
    tooltip: String,
    class: String,
}

fn check_host_connectivity(
    hostname: &str,
    port: u16,
    timeout: Duration,
    mid_threshold: u32,
    bad_threshold: u32,
) -> Result<PingStatus, Box<dyn std::error::Error>> {
    // Resolve hostname to socket address
    let addr = format!("{}:{}", hostname, port)
        .to_socket_addrs()?
        .next()
        .ok_or("Could not resolve hostname")?;

    // Measure connection time
    let start = Instant::now();

    let connection_result = TcpStream::connect_timeout(&addr, timeout);

    let rtt_ms = start.elapsed().as_millis() as u32;

    match connection_result {
        Ok(_) => {
            // Determine class based on connection time
            let class = if rtt_ms < mid_threshold {
                "good"
            } else if rtt_ms < bad_threshold {
                "mid"
            } else {
                "bad"
            };

            Ok(PingStatus {
                text: "🌍".to_string(),
                tooltip: rtt_ms.to_string(),
                class: class.to_string(),
            })
        }
        Err(_) => Ok(PingStatus {
            text: "🌍".to_string(),
            tooltip: "".to_string(),
            class: "down".to_string(),
        }),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Default configuration, can be overridden by environment variables or CLI args
    let hostname = std::env::var("PING_HOSTNAME").unwrap_or_else(|_| "www.google.com".to_string());
    let port: u16 = std::env::var("PING_PORT")
        .map(|v| v.parse().unwrap_or(80))
        .unwrap_or(80);
    let mid_threshold: u32 = std::env::var("PING_MID_THRESHOLD")
        .map(|v| v.parse().unwrap_or(50))
        .unwrap_or(50);
    let bad_threshold: u32 = std::env::var("PING_BAD_THRESHOLD")
        .map(|v| v.parse().unwrap_or(150))
        .unwrap_or(150);
    let timeout = Duration::from_secs(
        std::env::var("PING_TIMEOUT")
            .map(|v| v.parse().unwrap_or(1))
            .unwrap_or(1),
    );

    // Perform connectivity check and print JSON
    let status = check_host_connectivity(&hostname, port, timeout, mid_threshold, bad_threshold)?;
    println!("{}", serde_json::to_string(&status)?);

    Ok(())
}
