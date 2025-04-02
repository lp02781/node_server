FROM ubuntu:22.04

RUN apt update && apt install -y \
    cmake \
    gcc \
    g++ \
    git \
    libacl1-dev \
    libncurses5-dev \
    ninja-build \
    pkg-config \
    libyaml-cpp-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt
RUN git clone --recursive https://github.com/eclipse-iceoryx/iceoryx.git

WORKDIR /opt/iceoryx
RUN cmake -Bbuild -Hiceoryx_meta -DCMAKE_INSTALL_PREFIX=/usr/local && \
    cmake --build build && \
    cmake --install build

ENV ICEORYX_INSTALL_DIR=/usr/local

#WORKDIR /opt
#COPY sm_1.cpp /opt/sm_1.cpp
#COPY CMakeLists.txt /opt/

#RUN mkdir -p /opt/build && \
#    cd /opt && \
#    cmake -Bbuild -DCMAKE_PREFIX_PATH=$ICEORYX_INSTALL_DIR . && \
#    cmake --build build && \
#    make sm_1

#CMD ["/opt/build/sm_1"]
CMD ["/usr/local/bin/iox-roudi"]