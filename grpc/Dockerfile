FROM ubuntu:22.04

RUN apt update -y
RUN apt install -y \
    cmake \
    gcc \
    g++ \
    supervisor \
    pkg-config \
    build-essential \
    autoconf \
    libtool pkg-config \
    libprotobuf-dev \
    protobuf-compiler \
    libprotoc-dev \
    libgrpc++-dev
    nlohmann-json3-dev 
RUN rm -rf /var/lib/apt/lists/*

RUN git clone --recurse-submodules -b v1.61.0 https://github.com/grpc/grpc
$ mkdir -p cmake/build
$ cd cmake/build
$ cmake -DCMAKE_CXX_STANDARD=17 ../..
$ make

WORKDIR /opt
COPY sensor.proto /opt/
RUN 

COPY grpc_1.cpp /opt/
COPY grpc_2.cpp /opt/

RUN g++ grpc_1.cpp -o grpc_1 -std=c++11
RUN g++ grpc_2.cpp -o grpc_2 -std=c++11

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]