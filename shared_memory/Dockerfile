FROM ubuntu:22.04

RUN apt update && apt install -y \
    cmake \
    gcc \
    g++ \
    supervisor \
    git \
    libacl1-dev \
    libncurses5-dev \
    ninja-build \
    pkg-config \
    libyaml-cpp-dev \
    nlohmann-json3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt
COPY iceoryx /opt/iceoryx

WORKDIR /opt/iceoryx
RUN cmake -Bbuild -Hiceoryx_meta
RUN cmake -Bbuild -Hiceoryx_meta -DCMAKE_PREFIX_PATH=$(PWD)/build/dependencies/
RUN cmake --build build
RUN cmake --build build --target install

WORKDIR /opt
COPY sm_cpp_1.cpp /opt/
COPY sm_cpp_2.cpp /opt/
COPY CMakeLists.txt /opt/
RUN mkdir -p /etc/iceoryx
COPY roudi_config.toml /etc/iceoryx/

RUN mkdir -p /opt/build
WORKDIR /opt/build
RUN cmake ..
RUN make
RUN make install

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
