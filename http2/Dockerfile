FROM nginx:latest

ENV DEBIAN_FRONTEND "noninteractive"

ADD default.conf /etc/nginx/conf.d
ADD certificate.crt /etc/nginx/
ADD certificate.key /etc/nginx/
EXPOSE 443
