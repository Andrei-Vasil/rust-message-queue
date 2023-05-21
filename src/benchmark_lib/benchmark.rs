use chrono::Utc;
use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::io::Write;

lazy_static! {
    static ref BENCHMARKING: bool = true;
    static ref BENCHARK_ID: usize = 1;
    static ref BENCHMARK_FILE: String = format!("data/{}-{}.csv", Utc::now().format("%Y-%m-%d"), *BENCHARK_ID).to_string();
}

fn timestamp_now() -> String {
    format!("{}", (chrono::offset::Local::now().timestamp_nanos() as f64) / 1000000000.00)
}

pub fn set_publish_over(benchmark_id: usize) {
    if !*BENCHMARKING {
        return;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(BENCHMARK_FILE.clone())
        .unwrap();

    if let Err(e) = writeln!(file, "{},{}", timestamp_now().as_str(), benchmark_id) {
        eprintln!("Couldn't write to file: {}", e);
    }
}