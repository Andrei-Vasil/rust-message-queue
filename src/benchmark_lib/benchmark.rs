use chrono::Utc;
use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::io::Write;

lazy_static! {
    static ref BENCHMARKING: bool = true;
    static ref BENCHARK_ID: usize = 4;
    static ref LATENCY_BENCHMARK_FILE: String = format!("data/latency/{}-{}.csv", Utc::now().format("%Y-%m-%d"), *BENCHARK_ID).to_string();
    static ref PRODUCER_THROUGHPUT_BENCHMARK_FILE: String = format!("data/throughput/producer/{}-{}.csv", Utc::now().format("%Y-%m-%d"), *BENCHARK_ID).to_string();
    static ref CONSUMER_THROUGHPUT_BENCHMARK_FILE: String = format!("data/throughput/consumer/{}-{}.csv", Utc::now().format("%Y-%m-%d"), *BENCHARK_ID).to_string();
}

fn timestamp_now_nanoseconds() -> String {
    format!("{}", (chrono::offset::Local::now().timestamp_nanos() as f64) / 1000000000.00)
}

fn timestamp_now_seconds() -> String {
    format!("{}", chrono::offset::Local::now().timestamp())
}

pub fn set_publish_over(benchmark_id: usize) {
    if !*BENCHMARKING {
        return;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(LATENCY_BENCHMARK_FILE.clone())
        .unwrap();

    if let Err(e) = writeln!(file, "{},{}", timestamp_now_nanoseconds(), benchmark_id) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn count_producer_throughput() {
    if !*BENCHMARKING {
        return;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(PRODUCER_THROUGHPUT_BENCHMARK_FILE.clone())
        .unwrap();

    if let Err(e) = writeln!(file, "{}", timestamp_now_seconds()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn count_consumer_throughput() {
    if !*BENCHMARKING {
        return;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(CONSUMER_THROUGHPUT_BENCHMARK_FILE.clone())
        .unwrap();

    if let Err(e) = writeln!(file, "{}", timestamp_now_seconds()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}