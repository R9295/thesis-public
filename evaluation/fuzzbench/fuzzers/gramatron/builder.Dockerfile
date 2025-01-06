# Copyright 2020 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

ARG parent_image
FROM $parent_image

# Install libstdc++ to use llvm_mode.
RUN apt-get update && \
    apt-get install -y wget libstdc++-10-dev libc++-dev libtool-bin automake flex bison \
                       libglib2.0-dev libpixman-1-dev python3-setuptools unzip \
                       apt-utils apt-transport-https ca-certificates curl \
                       python3-dev gzip  libjson-c-dev libncurses5 \ 
                       libc6-dev autoconf automake libtool bison re2c pkg-config build-essential


RUN  apt-get update -y &&  apt-get install -y \
  build-essential \
  rsync \
  curl \
  zlib1g-dev \
  libncurses5-dev \
  libgdbm-dev \
  libnss3-dev \
  libssl-dev \
  libreadline-dev \
  libffi-dev \
  virtualenv \
  libbz2-dev \
  liblzma-dev \
  libsqlite3-dev

RUN cd /tmp/ && \
  curl -O https://www.python.org/ftp/python/3.10.15/Python-3.10.15.tar.xz && \
  tar -xvf Python-3.10.15.tar.xz && \
  cd Python-3.10.15 && \
  ./configure --enable-loadable-sqlite-extensions --enable-optimizations && \
   make -j install && \
   rm -r /tmp/Python-3.10.15.tar.xz /tmp/Python-3.10.15

RUN cp /usr/local/lib/libpython3.10.so.1.0 $OUT/

RUN wget https://github.com/llvm/llvm-project/releases/download/llvmorg-10.0.1/clang+llvm-10.0.1-x86_64-linux-gnu-ubuntu-16.04.tar.xz && tar -xvf clang+llvm-10.0.1-x86_64-linux-gnu-ubuntu-16.04.tar.xz && rsync -avh --force clang+llvm-10.0.1-x86_64-linux-gnu-ubuntu-16.04/* /usr/local/

RUN rm -rf /grammatron

RUN git clone https://github.com/R9295/Gramatron/ /gramatron && cd /gramatron && git pull && cd src && CC=clang-10 ./setup.sh && cd /gramatron/src/afl-gf/examples/aflpp_driver && make && cp libAFLDriver.a /
