FROM alpine

ARG SERVICE
ENV SERVICE=${SERVICE}
ENV RUST_BACKTRACE=1

WORKDIR /opt/code

ENTRYPOINT [ "./run.sh" ]
