FROM node:alpine

RUN corepack enable

ARG CODE_PATH

COPY ${CODE_PATH} /opt/code
WORKDIR /opt/code
RUN pnpm i
ENTRYPOINT [ "node", "index.js" ]
