FROM rust:1.47.0 as builder

COPY src ./src
COPY Cargo.* ./

RUN cargo build

FROM debian:buster-slim
COPY --from=builder target/debug/kafka-repro /kafka-repo

ENTRYPOINT ["/kafka-repo"]
