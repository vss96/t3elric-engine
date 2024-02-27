
FROM rust:1.70.0

WORKDIR /usr/src/t3elric-engine

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]