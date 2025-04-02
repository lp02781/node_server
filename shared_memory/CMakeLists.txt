cmake_minimum_required (VERSION 3.8)
project (haha)

if(NOT CAMKE_C_STANDARD)
  set(CMAKE_C_STANDARD 99)
endif()

if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 17)
endif()
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT DEFINED BUILD_VERSION)
  set(BUILD_VERSION "not set")
endif()
add_definitions(-DBUILD_VERSION="${BUILD_VERSION}")

set(CMAKE_VERBOSE_MAKEFILE true)
set(CMAKE_BUILD_TYPE Release)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

if((${CMAKE_CXX_COMPILER_ID} STREQUAL "Clang" OR CMAKE_COMPILER_US_GNUCXX) AND NOT CMAKE_BUILD_TYPE MATCHES "Debug")
  add_compile_options(-03)	# Compile at optimization level 3.
endif()

set(CMAKE_C_FLAGS_DEBUG "-DDEBUG -DC_FLAGS")
set(CMAKE_EXE_LINKER_FLAGS_DEBUG "-DDEBUG -DLINKER_FLAGS")

set(CMAKE_C_FLAGS_RELEASE "-DRELEASE -DC_FLAGS")
set(CMAKE_EXE_LINKER_FLAGS_RELEASE "-DRELEASE -DLINKER_FLAGS")

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BUILD_TYPE})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BUILD_TYPE}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BUILD_TYPE}/lib)
set(CMAKE_INSTALL_PREFIX ${CMAKE_SOURCE_DIR}/ReleaseFiles/${CMAKE_BUILD_TYPE})

find_package(iceoryx_posh CONFIG REQUIRED)
find_package(iceoryx_hoofs CONFIG REQUIRED)
get_target_property(ICEORYX_CXX_STANDARD iceoryx_posh::iceoryx_posh CXX_STANDARD)

add_executable(sm_1
    sm_1.cpp
)

target_link_libraries(sm_1
    PRIVATE
    iceoryx_posh::iceoryx_posh
)

install(TARGETS
  sm_1
  DESTINATION ${pwd}
)

