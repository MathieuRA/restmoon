use crate::utils;

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
