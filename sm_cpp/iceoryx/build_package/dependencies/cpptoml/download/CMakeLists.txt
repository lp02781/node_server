# Copyright (c) 2021 by Homalozoa. All rights reserved.
# Copyright (c) 2020 by Robert Bosch GmbH. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.16)

project(cpptoml-download NONE)

include(ExternalProject)
ExternalProject_Add(ext_cpptoml
  GIT_REPOSITORY    https://github.com/skystrife/cpptoml.git
  GIT_TAG           v0.1.1
  SOURCE_DIR        "/home/jetstream/Workspace/iceoryx/build_package/dependencies/cpptoml/src"
  BINARY_DIR        "/home/jetstream/Workspace/iceoryx/build_package/dependencies/cpptoml/build"
  CONFIGURE_COMMAND ""
  BUILD_COMMAND     ""
  INSTALL_COMMAND   ""
  TEST_COMMAND      ""
)
