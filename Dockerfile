FROM rust:1.74

WORKDIR /usr/src/metrics-dir-stat
COPY . .

RUN cargo install --path .

CMD ["metrics-dir-stat"]
