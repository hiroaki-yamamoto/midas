FROM node:alpine

RUN apk --no-cache --update upgrade && apk --no-cache add yarn

RUN mkdir -p /opt/code
WORKDIR /opt/code

ENTRYPOINT [ "./run.sh" ]
