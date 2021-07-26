# namahage

## Prerequisites

You must have `rust` and `cargo` installed if you build from source manually.

- [rust](https://www.rust-lang.org) (see [Getting started](https://www.rust-lang.org/learn/get-started) for details)
- cargo (automatically installed by `rustup`)


## Installation

### Use pre-built binary

Download the latest version from [releases page](https://github.com/ddbj/namahage/releases).

### Use docker

```shell
$ docker pull ***/***
$ docker run --rm ***/*** namahage
```

### Use singularity

```shell
$ singularity build namahage.sif docker://***/***
$ singularity exec namahage.sif namahage
```

### Build manually

```shell
$ git clone https://github.com/ddbj/namahage.git
$ cd namahage
$ cargo build --release
$ ./target/release/namahage
```

### Build manually with singularity

```shell
$ singularity build rust_slim-buster.sif docker://rust:slim-buster
$ git clone https://github.com/ddbj/namahage.git
$ cd namahage
$ singularity exec <path to rust_slim-buster.sif> cargo build --release
$ singularity exec <path to rust_slim-buster.sif> ./target/release/namahage
```

### For NIG supercomputer users

```shell
$ git clone https://github.com/ddbj/namahage.git
$ cd namahage
$ singularity exec /lustre6/public/app/namahage/rust_slim-buster.sif cargo build --release
$ singularity exec /lustre6/public/app/namahage/rust_slim-buster.sif ./target/release/namahage
```
