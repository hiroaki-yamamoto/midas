FROM nginx:alpine

COPY ./dist/ /srv/http
COPY nginx.conf /etc/nginx/
RUN mkdir -p /etc/certs/root /etc/certs/svc
