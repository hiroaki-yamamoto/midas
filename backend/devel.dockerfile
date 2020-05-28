FROM golang:alpine

ARG SERVICE
ENV SERVICE=${SERVICE}

ENTRYPOINT [ "./run.sh" ]
