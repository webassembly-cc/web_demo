Rust web demo with sdl2
=======================

## Pre-built compiler
To get a `rustc` with Emscripten support, you can install a nigthly compiler using [rustup](https://rustup.rs/). Follow these steps below (please refer to the official documentation for requirements and installation on Windows):
```
$ wget https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz
$ tar -xvf emsdk-portable.tar.gz
$ cd emsdk_portable
$ ./emsdk update
$ ./emsdk install sdk-incoming-64bit
```
Once everything is set up, you can compile with Emscripten:
```
$ cargo build --target asmjs-unknown-emscripten
```
Then, copy the output to the static directory:
```
$ cp target/asmjs-unknown-emscripten/debug/web_demo.js static/
```
The last step, open static/index.html with chrome(I only test it on chrome, maybe other browser is also ok).

[Online demo](http://115.28.32.115/web_demo/index.html)