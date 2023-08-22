FROM rust:1.71.0 AS builder

WORKDIR /rust/src/app

COPY . .

RUN cargo generate-lockfile
RUN cargo build --release

FROM debian:bullseye

RUN apt-get update && apt-get install -y \
    ca-certificates

COPY --from=builder /rust/src/app/target/release/nurl /

CMD ["./nurl"]

EXPOSE 80
