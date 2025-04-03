FROM rustlang/rust:nightly

WORKDIR /app

RUN apt update && apt install -y \
    llvm-dev \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib/

COPY Cargo.toml sm_rust_1.rs sm_rust_2.rs ./

RUN cargo build --release

CMD ["sh", "-c", "cargo run --release --bin sm_rust_1 & cargo run --release --bin sm_rust_2 && wait"]
