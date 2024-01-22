# metrics-dir-stat

## Metrics

* `metricsdirstat_directory_size_bytes`, labels: name, The current size of a tracked directory in bytes
* `metricsdirstat_scan_time_ms`, labels: name, The time it took to do the last scan in ms

## Configuration

* Probably want to mount your data directory into the docker image.
* Also need to mount whatever path has your `dirs.csv` (if different from your data dir).
* Set your path to `dirs.csv` as the `TRACKED_DIRS_FILE` environment variable.

## Example docker compose

TODO

## License

MIT
