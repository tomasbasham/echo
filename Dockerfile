FROM rust:1.50.0-alpine3.13 as builder

WORKDIR /usr/src/app

RUN apk add --no-cache build-base ca-certificates musl pkgconf protoc \
  && rustup target add x86_64-unknown-linux-musl \
  && cargo init --bin --name dummy .

COPY Cargo.toml .
COPY Cargo.lock .

# Set up our environment variables so that we cross-compile using musl-libc by
# default.
ENV X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_DIR=/usr/local/musl/
ENV X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_STATIC=1
ENV PQ_LIB_STATIC_X86_64_UNKNOWN_LINUX_MUSL=1
ENV PG_CONFIG_X86_64_UNKNOWN_LINUX_GNU=/usr/bin/pg_config
ENV PKG_CONFIG_ALLOW_CROSS=true
ENV PKG_CONFIG_ALL_STATIC=true
ENV LIBZ_SYS_STATIC=1
ENV TARGET=musl

# Cache the application dependencies
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY . .

# Build the real application
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/echo /echo
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

ENTRYPOINT ["/echo"]
