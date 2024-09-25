FROM rust:1.77.2

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["app_learn"]

FROM rust:1.77.2 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust_learn /usr/local/bin/rust_learn
CMD ["rust_learn"]


