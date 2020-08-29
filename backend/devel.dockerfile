FROM scratch

ARG SERVICE
ENV SERVICE=${SERVICE}

COPY ./target/x86_64-unknown-linux-musl/debug/${SERVICE} /app
ENTRYPOINT [ "/app" ]
