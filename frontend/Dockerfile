FROM nginx:alpine

COPY ./dist/frontend/ /srv/http
COPY nginx.conf /etc/nginx/
RUN mkdir -p /etc/certs
