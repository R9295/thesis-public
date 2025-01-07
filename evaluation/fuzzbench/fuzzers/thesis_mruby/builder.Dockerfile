ARG parent_image
FROM $parent_image

RUN rm -rf /thesis
RUN mkdir /thesis

COPY libafl-fuzzer /thesis/libafl-fuzzer
COPY ruby /thesis/ruby
COPY thesis_derive /thesis/thesis_derive
COPY thesis /thesis/thesis

RUN echo 1
# Install libstdc++ to use llvm_mode.
RUN apt-get update && \
    apt-get install -y wget libstdc++-10-dev libtool-bin automake flex bison \
                       libglib2.0-dev libpixman-1-dev python3-setuptools unzip \
                       apt-utils apt-transport-https ca-certificates joe curl \
                       python3-dev gzip

# Install latest Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /rustup.sh && \
    sh /rustup.sh -y

RUN PATH="/root/.cargo/bin/:$PATH" rustup default nightly

RUN cd /thesis/ruby && PATH="/root/.cargo/bin/:$PATH" cargo +nightly build --release

# Download afl++.
RUN git clone -b dev https://github.com/AFLplusplus/AFLplusplus /afl
# Build without Python support as we don't need it.
# Set AFL_NO_X86 to skip flaky tests.
RUN cd /afl && \
    unset CFLAGS CXXFLAGS && \
    export CC=clang AFL_NO_X86=1 && \
    PYTHON_INCLUDE=/ make install && \
    cp utils/aflpp_driver/libAFLDriver.a /
