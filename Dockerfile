FROM rust:1.70.0

WORKDIR /usr/src/t3elric-engine

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]

LABEL org.opencontainers.image.source=https://github.com/vss96/t3elric-engine
LABEL org.opencontainers.image.description="My ST3P compliant engine image"
LABEL org.opencontainers.image.licenses=MIT