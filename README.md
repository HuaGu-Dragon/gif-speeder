# A Cli Tool that allows you to speed or slow down GIFs

## Usage

```bash
gif-speeder --input input.gif --output output.gif --speed 2.0 --min 5 --skip 10 --repeat 3
```

# Quick Start

```bash
git clone git@github.com:HuaGu-Dragon/gif-speeder.git
cd gif-speeder
cargo run --release -- -i input.gif -o output.gif -s 2.0 -m 5 -k 10 -r 3
```

## Notice

* In wechat, the minimum duration of a frame is 10ms.
* In QQ, the minimum duration of a frame is 20ms.
