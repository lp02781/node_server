FROM rustlang/rust:nightly

WORKDIR /app

RUN apt update && apt install -y \
    llvm-dev \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib/

COPY Cargo.toml sm_rust_pub.rs sm_rust_sub.rs ./

RUN cargo build --release

CMD ["sh", "-c", "cargo run --release --bin sm_rust_pub & cargo run --release --bin sm_rust_sub && wait"]
