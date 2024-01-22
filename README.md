# metrics-dir-stat

## Metrics

* `metricsdirstat_directory_size_bytes`, labels: name, The current size of a tracked directory in bytes
* `metricsdirstat_scan_time_ms`, labels: name, The time it took to do the last scan in ms

## Configuration

* Probably want to mount your data directory into the docker image.
* Also need to mount whatever path has your `dirs.csv` (if different from your data dir).
* Set your path to `dirs.csv` as the `TRACKED_DIRS_FILE` environment variable.

## Example docker compose

```yaml
version: '3.8'
services:
  metrics-dir-stat:
    image: ghcr.io/kj800x/metrics-dir-stat:master
    container_name: metrics-dir-stat
    volumes:
      - /data:/data:ro
    ports:
      - "26625:9090/tcp"
    environment:
      - TRACKED_DIRS_FILE=/data/docker/metrics-dir-stat/dirs.csv
    restart: unless-stopped
```

## License

MIT
