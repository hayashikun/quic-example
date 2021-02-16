FROM nginx:latest

ENV DEBIAN_FRONTEND "noninteractive"

ADD default.conf /etc/nginx/conf.d
ADD certificate/fullchain.pem /usr/local/nginx/
ADD certificate/privkey.pem /usr/local/nginx/
EXPOSE 443
