FROM rustlang/rust:nightly

WORKDIR /app

COPY Cargo.toml ./

COPY src ./src

RUN cargo build --release

# JavaScript frontend served by Actix at "/".
COPY static ./static

CMD ["cargo", "run", "--release"]
