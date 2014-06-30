# Thrust #

A small 64-bit, Multiboot 1.6 compliant kernel written in Rust and assembly.

## Dependencies ##
#### Build ####
* `clang` (capable of cross-compiling for x86-64)
* `nasm`
* `rust` (from git)
* `ninja`
* `xorriso`

#### Runtime ####
* A Multiboot 1.6 compliant bootloader (GRUB 2)

#### Emulation ####
* `qemu`
* `ninja`

## Build Instructions ##

#### Linux ####
Some distributions have compilers that will work just fine out-of-the-box, while some don't.

```bash
$ cd path/to/Thrust
$ ./configure
$ ninja
```

#### OSX ####
You should use Homebrew for as much as possible.  The configure script
autogenerates `binutils` for you because OSX has a broken version by default.
Please note that this assumes you already have `clang` installed via Xcode.

```bash
$ brew install nasm qemu ninja xorriso
$ brew install --HEAD rust
$ cd path/to/Thrust
$ ./configure
$ ninja
```

## Emulation Instructions ##
```bash
$ ninja run
```

## Legal ##
Copyright (C) 2014 Dominator008.
All rights reserved.

See LICENSE.txt for description of this project's license.
