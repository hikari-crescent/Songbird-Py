#!bin/sh

#https://signes.pl/update-cmake-to-version-3-in-centos/

sudo set -ex \
  && for key in C6C265324BBEBDC350B513D02D2CEF1034921684; do \
    gpg --keyserver hkp://p80.pool.sks-keyservers.net:80 --recv-keys "$key" || \
    gpg --keyserver hkp://ipv4.pool.sks-keyservers.net --recv-keys "$key" || \
    gpg --keyserver hkp://pgp.mit.edu:80 --recv-keys "$key" ; \
  done
  
ENV CMAKE_VERSION 3.6.2

sudo set -ex \
  && curl -fsSLO --compressed https://cmake.org/files/v3.6/cmake-${CMAKE_VERSION}-Linux-x86_64.tar.gz \
  && curl -fsSLO --compressed https://cmake.org/files/v3.6/cmake-${CMAKE_VERSION}-SHA-256.txt.asc \
  && curl -fsSLO --compressed https://cmake.org/files/v3.6/cmake-${CMAKE_VERSION}-SHA-256.txt \
  && gpg --verify cmake-${CMAKE_VERSION}-SHA-256.txt.asc cmake-${CMAKE_VERSION}-SHA-256.txt \
  && grep "cmake-${CMAKE_VERSION}-Linux-x86_64.tar.gz\$" cmake-${CMAKE_VERSION}-SHA-256.txt | sha256sum -c - \
  && tar xzf cmake-${CMAKE_VERSION}-Linux-x86_64.tar.gz -C /usr/local --strip-components=1 --no-same-owner \
  && rm -rf cmake-${CMAKE_VERSION}*
