//! SigmaOS-network-performance-monitoring — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_network_performance_monitoring_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-network-performance-monitoring: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-network-performance-monitoring: ok");
}
