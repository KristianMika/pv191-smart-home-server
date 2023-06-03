FROM rustembedded/cross:aarch64-unknown-linux-gnu

ENV PYO3_CROSS_PYTHON_VERSION=3.5
ENV PYO3_CROSS_LIB_DIR="/usr/lib/"
ENV LD_LIBRARY_PATH="/usr/lib"
ENV LIBRARY_PATH="/usr/lib"

# RUN apt-get update && \
#     apt-get install --assume-yes python3-dev libpython3-dev


RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install --assume-yes libpq-dev:arm64 python3-dev:arm64 libpython3-dev:arm64 || apt-get install --fix-broken -y

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install --assume-yes libpq-dev:arm64 python3-dev:arm64 libpython3-dev:arm64 || apt-get install --fix-broken -y

RUN ln -s /usr/lib/python3.5/config-3.5m-x86_64-linux-gnu/libpython3.5.so /usr/lib/libpython3.5m.so
RUN ln -s /usr/lib/python3.5/config-3.5m-x86_64-linux-gnu/libpython3.5.so /usr/lib/python3.5m.so

