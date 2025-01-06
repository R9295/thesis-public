#!/bin/bash -eu
# Copyright 2019 Google Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
################################################################################
cd $SRC
git clone https://github.com/R9295/php-src php-src && cd php-src && git checkout PHP-7.4.33 && git pull && cd ..
cd $SRC/php-src
# PHP's zend_function union is incompatible with the object-size sanitizer
export CFLAGS="$CFLAGS -fno-sanitize=object-size -Wno-incompatible-function-pointer-types"
export CXXFLAGS="$CXXFLAGS -fno-sanitize=object-size"

# Disable JIT profitability checks.
export CFLAGS="$CFLAGS -DPROFITABILITY_CHECKS=0"

# Make sure the right assembly files are picked
BUILD_FLAG=""
if [ "$ARCHITECTURE" = "i386" ]; then
    BUILD_FLAG="--build=i686-pc-linux-gnu"
fi

# build project
./buildconf --force
./configure $BUILD_FLAG \
    --disable-all \
    --enable-option-checking=fatal \
    --enable-fuzzer \
    --enable-exif \
    --enable-opcache \
    --without-pcre-jit \
    --disable-phpdbg \
    --disable-cgi \
    --with-pic


make -j$(nproc)

cp sapi/fuzzer/php-fuzz-execute $OUT/fuzz

