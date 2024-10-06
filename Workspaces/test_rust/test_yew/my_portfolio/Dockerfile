FROM rust:latest
WORKDIR /app
COPY . .
RUN corgo build --release
CMD ["./target/release/my-portfolio"]
