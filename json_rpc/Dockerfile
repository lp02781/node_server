FROM ubuntu:22.04

RUN apt update -y
RUN apt install -y \
    cmake \
    gcc \
    g++ \
    supervisor \
    pkg-config \
    build-essential \
    nlohmann-json3-dev 
RUN rm -rf /var/lib/apt/lists/*

WORKDIR /opt

COPY json_rpc_1.cpp /opt/
COPY json_rpc_2.cpp /opt/

RUN g++ json_rpc_1.cpp -o json_rpc_1 -std=c++11
RUN g++ json_rpc_2.cpp -o json_rpc_2 -std=c++11

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]