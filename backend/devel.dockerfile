FROM alpine

ARG SERVICE
ENV RUST_BACKTRACE=1

WORKDIR /opt/code
COPY ./target/x86_64-unknown-linux-musl/debug/${SERVICE} /app

ENTRYPOINT [ "/app" ]
