FROM ubuntu:latest

ENV DEBIAN_FRONTEND "noninteractive"

RUN apt-get update \
    && apt-get install -y \
        git \
        mercurial \
        build-essential \
        curl \
        libpcre3 \
        libpcre3-dev \
        zlib1g \
        zlib1g-dev \
        ninja-build \
        cmake \
        perl \
        golang

RUN git clone https://boringssl.googlesource.com/boringssl \
    && cd boringssl \
    && mkdir build \
    && cd build \
    && cmake -GNinja .. \
    && ninja bssl

RUN hg clone -b quic https://hg.nginx.org/nginx-quic \
    && cd nginx-quic \
    && ./auto/configure \
        --with-debug \
        --with-http_v3_module \
        --with-cc-opt="-I../boringssl/include" \
        --with-ld-opt="-L../boringssl/build/ssl -L../boringssl/build/crypto" \
    && make \
    && make install

ADD nginx.conf /usr/local/nginx/conf
ADD certificate/fullchain.pem /usr/local/nginx/
ADD certificate/privkey.pem /usr/local/nginx/
EXPOSE 443

CMD ["/usr/local/nginx/sbin/nginx", "-g", "daemon off;"]