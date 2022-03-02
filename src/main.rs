use hex;
use indicatif::{ProgressBar, ProgressStyle};
use md5::{Digest, Md5};
use rayon::prelude::*;
use std::env;
use std::sync;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name: &String = args
        .get(1)
        .expect("error: a name is required as the first argument.");
    let id: &String = args
        .get(2)
        .expect("error: an ID is required as the second argument.");
    println!(
        "trying if any IPv4 addresses match \"{}\" (ID: \"{}\")",
        name, id
    );
    let found = sync::Arc::new(sync::Mutex::new(false));
    let found_ip = sync::Arc::new(sync::Mutex::new(String::from("<not found ip>")));
    let number_of_ipv4_addresses: u64 = u64::pow(256, 4);
    println!("number of all IPv4 addresses: {}", number_of_ipv4_addresses);
    let bar = ProgressBar::new(number_of_ipv4_addresses).with_style(
        ProgressStyle::default_bar()
            .template(
                "{msg}ETA {eta_precise}, elapsed {elapsed_precise} [{wide_bar}] {pos}/{len} {percent}%",
            )
            .progress_chars("=- "),
    );
    let bar_inc_delta = u64::pow(256, 2);
    bar.tick();
    (0..=255).into_par_iter().for_each(|a| {
        for b in 0..=255 {
            if *found.lock().unwrap() {
                return;
            }
            for c in 0..=255 {
                for d in 0..=255 {
                    let ip = format!("{}.{}.{}.{}", a, b, c, d);
                    if ip_matches_name_and_id(&ip, name, id) {
                        *found.lock().unwrap() = true;
                        *found_ip.lock().unwrap() = ip;
                        return;
                    }
                }
            }
            bar.inc(bar_inc_delta);
            bar.set_message(&format!("done: {}.{}.255.255, ", a, b));
        }
    });
    if !*found.lock().unwrap() {
        bar.finish();
        println!("not found");
        return;
    }
    bar.abandon();
    println!("found: {}", found_ip.lock().unwrap());
}

fn hash_name_and_ip(name: &str, ip: &str) -> String {
    hex::encode(Md5::digest(format!("{}{}", name, ip).as_bytes()))
}

fn ip_matches_name_and_id(ip: &str, name: &str, id: &str) -> bool {
    hash_name_and_ip(name, ip) == id
}
