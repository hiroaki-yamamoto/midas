FROM golang:alpine

ARG SERVICE
ENV SERVICE=${SERVICE}

RUN mkdir -p /opt/code
WORKDIR /opt/code

ENTRYPOINT [ "./run.sh" ]
