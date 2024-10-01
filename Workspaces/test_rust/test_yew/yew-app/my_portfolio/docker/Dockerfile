FROM rust:1.81 

WORKDIR /app
COPY ./backend/Cargo.toml ./Cargo.toml
COPY ./backend/Cargo.lock ./Cargo.lock
COPY ./backend/src ./src

RUN cargo build --release

CMD [ "./target/release/my_backend" ]

