## -----------------------------------------------------------------------------
## Build
## -----------------------------------------------------------------------------
FROM rust:1.56-slim-buster as build-stage

RUN apt update && apt install -y --no-install-recommends pkg-config libssl-dev libsqlite3-dev

WORKDIR "/build"

# Install and build crates
COPY Cargo.* /build/
RUN mkdir /build/src && echo "fn main() {}" > /build/src/main.rs
RUN cargo build --release

# Build app
COPY src/ /build/src/
COPY migrations/ /build/migrations/
RUN touch src/main.rs && cargo build --release

## -----------------------------------------------------------------------------
## Package
## -----------------------------------------------------------------------------
FROM ubuntu:21.10

RUN apt update && apt install -y --no-install-recommends libssl-dev libsqlite3-dev openssl ca-certificates

COPY --from=build-stage "/build/target/release/tg-book-of-grumdges" "/app/tg-book-of-grumdges"

WORKDIR "/app"
ENTRYPOINT ["/app/tg-book-of-grumdges"]
