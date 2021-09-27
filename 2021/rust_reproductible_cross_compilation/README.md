#


## Windows

```shell
$ docker build . -t rust_cross_compile/windows -f Dockerfile.windows
$ docker run --rm -ti -v `pwd`:/app rust_cross_compile/windows
```


## Linux aarch64 (armv8)

```shell
$ docker build . -t rust_cross_compile/aarch64 -f Dockerfile.aarch64
$ docker run --rm -ti -v `pwd`:/app rust_cross_compile/aarch64
```

## Linux armv7

```shell
$ docker build . -t rust_cross_compile/armv7 -f Dockerfile.armv7
$ docker run --rm -ti -v `pwd`:/app rust_cross_compile/armv7
```
