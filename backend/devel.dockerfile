FROM rust:alpine

ARG SERVICE
ENV SERVICE=${SERVICE}
WORKDIR /opt/code

ENTRYPOINT [ "./run.sh" ]
