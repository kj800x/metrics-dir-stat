use fs_extra::dir::get_size;
use metrics::{describe_gauge, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct TrackedDirectory {
    name: String,
    path: String,
}

fn read_tracked_directories() -> Result<Vec<TrackedDirectory>, Box<dyn Error>> {
    let f = File::open(std::env::var("TRACKED_DIRS_FILE").unwrap_or("dirs.csv".to_string()))
        .expect("Expected TRACKED_DIRS_FILE to be set");
    let reader = BufReader::new(f);

    let dirs: Vec<TrackedDirectory> = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader)
        .records()
        .map(|r| {
            let string_record = r.unwrap();
            TrackedDirectory {
                name: string_record[0].to_string(),
                path: string_record[1].to_string(),
            }
        })
        .collect();

    Ok(dirs)
}

fn main() {
    tracing_subscriber::fmt::init();

    let builder = PrometheusBuilder::new();
    builder
        .idle_timeout(MetricKindMask::ALL, Some(Duration::from_secs(10 * 60)))
        .with_http_listener(([0, 0, 0, 0], 9090))
        .install()
        .expect("Failed to install Prometheus recorder");

    describe_gauge!(
        "metricsdirstat_directory_size_bytes",
        "The current size of a tracked directory in bytes."
    );
    describe_gauge!(
        "metricsdirstat_scan_time_ms",
        "The time it took to do the last scan in ms."
    );

    let tracked_dirs = read_tracked_directories().unwrap();

    loop {
        println!("Measuring dirs...");

        tracked_dirs.iter().for_each(|x| {
            let now = Instant::now();
            let folder_size = get_size(x.path.clone()).unwrap();
            gauge!("metricsdirstat_directory_size_bytes", folder_size as f64, "name" => x.name.clone());
            let elapsed = now.elapsed();
            gauge!("metricsdirstat_scan_time_ms", elapsed.as_millis() as f64, "name" => x.name.clone());

        });

        // Sleep 5 min.
        println!("Sleeping...");
        // TODO set back to 5 min
        thread::sleep(Duration::from_secs(5 * 60));
    }
}
