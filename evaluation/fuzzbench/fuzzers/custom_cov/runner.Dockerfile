FROM gcr.io/fuzzbench/base-image

RUN apt update && apt -y install libexpat1-dev zlib1g-dev libjson-c-dev 

WORKDIR $SRC

# This makes interactive docker runs painless:
ENV LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/out"
#ENV AFL_MAP_SIZE=2621440
ENV PATH="$PATH:/out"
