use std::time::Duration;

use chrono::format::{DelayedFormat, StrftimeItems};

use crate::utils::{self, proxy::Proxy, size::format_size};

pub fn initial_log() {
    let config = utils::config::get_config();

    println!("🚀 Proxy Analyzer starting (Raw TCP/HTTP)...");
    println!("   Listening on: 127.0.0.1:{}", config.listen_port);
    if let Some(ref dest) = config.default_destination {
        println!("   Default destination: {}", dest);
    } else {
        println!("   Default destination: None (use X-Proxy-Destination header)");
    }
    println!("   Analyzing requests...\n");
    println!("📊 Request Analytics:");
    println!("----------------------------------------");
}

pub fn final_log(
    proxy: &Proxy,
    date: DelayedFormat<StrftimeItems>,
    duration: Duration,
    size: usize,
) {
    println!(
        "[{}] {} {} -> {} ({:.2}ms) [Response: {}]",
        date,
        proxy.request.method,
        proxy.request.path,
        proxy.request.destination.clone().to_string(),
        duration.as_secs_f64() * 1000.0,
        format_size(size)
    );
}
