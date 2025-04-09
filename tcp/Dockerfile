FROM ubuntu:22.04

RUN apt update -y
RUN apt install -y \
    cmake \
    gcc \
    g++ \
    supervisor \
    pkg-config \
    nlohmann-json3-dev 

RUN rm -rf /var/lib/apt/lists/*

WORKDIR /opt
COPY tcp_1.cpp /opt/
COPY tcp_2.cpp /opt/

RUN g++ tcp_1.cpp -o tcp_1 -std=c++11
RUN g++ tcp_2.cpp -o tcp_2 -std=c++11

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]