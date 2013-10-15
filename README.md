# kRnel #

A small 64-bit, Multiboot kernel written in Rust.

## Dependencies ##
#### Build ####
* a cross compiler for x86-64
* `nasm`
* `rust` (0.8)
* `ninja`

#### Runtime ####
* a Multiboot-compliant bootloader

#### Emulation ####
* `qemu`
* `ninja`

## Build Instructions ##

#### Linux ####
Most distributions have compilers that will work just fine out-of-the-box, so all you need is `qemu`, `nasm`, `rust`, and `ninja`.

```bash
$ sudo yaourt -S nasm qemu rust ninja
$ cd path/to/rustboot
$ ./configure
$ ninja
```

#### OSX ####
You should use Homebrew for as much as possible.

```bash
$ brew install nasm qemu ninja
$ wget 'ftp://sourceware.org/pub/binutils/snapshots/binutils-2.23.52.tar.bz2'
$ ./configure --target=x86-64-elf --prefix=/somewhere/in/the/path
$ make && make install
$ git clone git://github.com/mozilla/rust
$ cd rust
$ ./configure --prefix=/somewhere/in/the/path
$ make && make install
$ cd path/to/rustboot
$ ./configure
$ ninja
```

## Emulation Instructions ##
```bash
$ ninja run
```

## Legal ##
Copyright (C) 2013 Arcterus.
All rights reserved.

See License.txt for description of this project's license.
