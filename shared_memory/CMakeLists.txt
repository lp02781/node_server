cmake_minimum_required(VERSION 3.8)
project(sm_cpp)

set(CMAKE_CXX_STANDARD 17)

# Add include directories for Iceoryx
include_directories(/opt/iceoryx/include)

# Add the executable
add_executable(sm_1 sm_1.cpp)

# Link Iceoryx libraries
target_link_libraries(sm_1 /opt/iceoryx/lib/libiceoryx_posh.so)
