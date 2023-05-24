use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;

lazy_static! {
    static ref BENCHMARKING: bool = true;
}

fn timestamp_now_nanoseconds() -> String {
    format!("{}", (chrono::offset::Local::now().timestamp_nanos() as f64) / 1000000000.00)
}

fn timestamp_now_seconds() -> String {
    format!("{}", chrono::offset::Local::now().timestamp())
}

pub fn set_publish_over(benchmark_id: usize, scenario_id: Arc<String>) {
    if !*BENCHMARKING {
        return;
    }

    let path = format!("data/latency/{}.csv", scenario_id);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{},{}", timestamp_now_nanoseconds(), benchmark_id) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn count_producer_throughput(scenario_id: Arc<String>) {
    if !*BENCHMARKING {
        return;
    }

    let path = format!("data/throughput/producer/{}.csv", scenario_id);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", timestamp_now_seconds()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn count_consumer_throughput(scenario_id: Arc<String>) {
    if !*BENCHMARKING {
        return;
    }

    let path = format!("data/throughput/consumer/{}.csv", scenario_id);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", timestamp_now_seconds()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}