Dradis
===

Dradis is a library that wraps the common wireless library `iw`'s wireless network scanning tool.

Example
---
```
use dradis::WifiScan;

fn main() {
    println!("Starting scan...");
    let local_networks = match WifiScan::scan("wlp58s0".to_string()) {
        Ok(scan) => scan,
        Err(err) => panic!("FAIL"),
    };
    println!("{} Networks:", local_networks.networks.len());
    local_networks.networks.into_iter().map(|network| {
        println!("{}: Encryption: {}",
                 network.essid.unwrap(),
                 network.encryption);
    });
}
```
