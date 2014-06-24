#!/bin/sh

BASE_URL='http://static.rust-lang.org/dist/'

RUST_BUILD_DIR="rust-nightly"
RUST_BINARY_DIR="$RUST_BUILD_DIR-x86_64-unknown-linux-gnu"
RUST_BUILD_ARCHIVE="$RUST_BUILD_DIR.tar.gz"
RUST_BINARY_ARCHIVE="$RUST_BINARY_DIR.tar.gz"
RUST_BUILD_URL="$BASE_URL$RUST_BUILD_ARCHIVE"
RUST_BINARY_URL="$BASE_URL$RUST_BINARY_ARCHIVE"
if [[ ! -f "$RUST_BUILD_ARCHIVE" ]]; then
  echo "Downloading $RUST_BUILD_ARCHIVE from $BASE_URL"
  curl -O "$RUST_BUILD_URL" || exit 1
fi
if [[ ! -f "$RUST_BINARY_ARCHIVE" ]]; then
  echo "Downloading $RUST_BINARY_ARCHIVE from $BASE_URL"
  curl -O "$RUST_BINARY_URL" || exit 1
fi
if [[ ! -d rust-nightly ]]; then
  echo "Extracting $RUST_BUILD_DIR from $RUST_BUILD_ARCHIVE"
  tar -zxf "$RUST_BUILD_ARCHIVE" || exit 1
fi
if [[ ! -d rust ]]; then
  echo "Extracting $RUST_BINARY_DIR from $RUST_BINARY_ARCHIVE"
  tar -zxf "$RUST_BINARY_ARCHIVE" || exit 1
  echo "Moving $RUST_BINARY_DIR to rust"
  mv "$RUST_BINARY_DIR" rust
fi
mkdir -p install
if [[ "`ls install | grep libcore`" == "" || "`ls install | grep librlibc`" == "" ]]; then
  echo "Installing libcore and librlibc"
  mv rust/lib/rustlib/x86_64-unknown-linux-gnu/lib/rustlib/libcore*.rlib install/
  mv rust/lib/rustlib/x86_64-unknown-linux/gnu/lib/librlibc*.rlib install/
fi
if [[ ! -f install/bin/rustc ]]; then
  cd rust-nightly
  echo "Configure rust"
  #CC=clang ./configure --prefix=/ || exit 1
  CC=gcc CXX=g++ ./configure --prefix=/ || exit 1
  echo "Building rust"
  make -j32 || exit 1
  echo "Installing rust"
  make DESTDIR="$PWD"/../install install || exit 1
fi
