
FROM rust:1.48 as builder
WORKDIR /usr/src/tracing-app
COPY . .
RUN rustup component add rustfmt
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get clean && \
    apt-get update && \
    rm -rf /var/lib/apt/lists/* \
    apt-get clean
COPY --from=builder /usr/local/cargo/bin/tracingapp-server /usr/local/bin/tracingapp-server
COPY --from=builder /usr/local/cargo/bin/tracingapp-client /usr/local/bin/tracingapp-client
CMD ["tracingapp-server"]

