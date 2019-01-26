# Meow

Meow is a tool to create a kitty image with weather information.

## Quick Start

Before you start, you need to have Rust and Cargo.

```sh
curl https://sh.rustup.rs -sSf | sh
```

Then we could get the source code.

```sh
git clone https://github.com/weather-bot/meow.git
cd meow
```

You can create a demo image immediately.

```sh
make test        # Create in corner-mode
make test-bottom # Create in bottom-mode
```

To see the full usage:

```sh
make help
```

## Demo

The original image:

<img height="400" border="0" alt="input" src="https://raw.githubusercontent.com/weather-bot/meow/master/test.jpg">

The output kitty image with information

corner-mode:

<img height="400" border="0" alt="corner_output" src="https://raw.githubusercontent.com/weather-bot/meow/master/sample/corner_out.jpg">

bottom-mode:

<img height="400" border="0" alt="bottom_output" src="https://raw.githubusercontent.com/weather-bot/meow/master/sample/bottom_out.jpg">

chinese-mode:

<img height="400" border="0" alt="bottom_output" src="https://raw.githubusercontent.com/weather-bot/meow/master/sample/chinese_out.jpg">
