FROM rustlang/rust:nightly-bullseye as builder

# Install cargo-binstall for fast prebuilt binary installations
RUN wget -qO- https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz | tar -xzf - -C /usr/local/cargo/bin

# Install cargo-leptos and wasm-bindgen-cli via binstall instead of compiling from source
RUN cargo binstall cargo-leptos -y
RUN cargo binstall wasm-bindgen-cli@0.2.108 -y

# Add nightly rust-src and wasm target
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown

# Install Node.js for Tailwind CSS plugins (@tailwindcss/typography)
RUN apt-get update && apt-get install -y nodejs npm && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Install npm dependencies first for better layer caching
COPY package.json package-lock.json ./
RUN npm install

# Copy the rest of the source
COPY . .

RUN cargo leptos --manifest-path=./Cargo.toml build --release -vv

FROM rustlang/rust:nightly-bullseye-slim as runner
COPY --from=builder /app/target/release/server /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="akarras"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD ["/app/server"]
