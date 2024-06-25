FROM --platform=$TARGETPLATFORM ubuntu:22.04 AS builder
WORKDIR /usr/src/csml
COPY target/release/csml_server amd64_csml_server
COPY target/aarch64-unknown-linux-gnu/release/csml_server arm64_csml_server

FROM --platform=$TARGETPLATFORM ubuntu:22.04

ARG TARGETPLATFORM
ARG TARGETARCH

ENV amd64_path=target/release/csml_server
ENV arm64_path=target/aarch64-unknown-linux-gnu/release/csml_server

RUN apt update && apt install -y ca-certificates libpq-dev && apt clean
RUN update-ca-certificates

WORKDIR /usr/src/csml

COPY --from=builder /usr/src/csml/${TARGETARCH}_csml_server server

RUN chmod 755 server

RUN groupadd -r csml && useradd -r -g csml csml
USER csml

EXPOSE 5000

CMD ./server
