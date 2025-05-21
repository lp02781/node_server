FROM ubuntu:22.04

RUN apt update -y
RUN apt install -y \
    cmake \
    gcc \
    g++ \
    supervisor \
    pkg-config \
    libboost-all-dev \
    nlohmann-json3-dev 

RUN rm -rf /var/lib/apt/lists/*

WORKDIR /opt
COPY tcp_client.cpp /opt/
COPY tcp_server.cpp /opt/

RUN g++ tcp_client.cpp -o tcp_client -std=c++11
RUN g++ tcp_server.cpp -o tcp_server -std=c++11

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]