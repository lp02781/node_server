set -e
set -x

sudo apt install -y \
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
    libboost-all-dev \
    nlohmann-json3-dev

cd iceoryx
if [ -d "build" ]; then
  cd ..
else
  cmake -Bbuild -Hiceoryx_meta
  cmake -Bbuild -Hiceoryx_meta -DCMAKE_PREFIX_PATH=$(PWD)/build/dependencies/
  cmake --build build
  sudo cmake --build build --target install
  cd ..
fi

if [ -d "build" ]; then
  cd build
else
  mkdir -p build
  cd build
fi

cmake ..
make
sudo make install

cd ..

sudo mkdir -p /etc/iceoryx
sudo cp roudi_config.toml /etc/iceoryx/

#./iceoryx/build/iox-roudi
#./build/Release/sm_cpp_pub
#./build/Release/sm_cpp_sub


