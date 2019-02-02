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
make test-light # Create in light-mode
```

To see the full usage:

```sh
make help
```

## Demo

The original image:

<img height="400" border="0" alt="input" src="https://raw.githubusercontent.com/weather-bot/meow/master/test.jpg">

output with information in light-mode:

<img height="400" border="0" alt="light_output" src="https://raw.githubusercontent.com/weather-bot/meow/master/sample/light_out.jpg">
