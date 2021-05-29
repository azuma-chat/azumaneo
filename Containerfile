FROM docker.io/library/alpine:latest AS builder
WORKDIR /build
RUN apk add build-base cargo
COPY Cargo.toml Cargo.lock sqlx-data.json .
COPY src src/
COPY migrations migrations/
RUN cargo build --release

FROM scratch
COPY --from=builder /build/target/release/azumaneo .
CMD ["./azumaneo"]
