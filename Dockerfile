FROM docker.io/rust:latest as builder

WORKDIR /workspace

COPY . .

RUN cargo clean && cargo build --release

FROM registry.fedoraproject.org/fedora-minimal:40

WORKDIR /app

EXPOSE 8080

COPY --from=builder /workspace/target/release/rust-webapp-sample /app/application

ENTRYPOINT [ "/app/application" ]
