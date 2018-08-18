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

You can create an image immediately.

Chinese Demo:

```sh
cargo run test.jpg '{"title":"今天他喵的會下雨！","time":"明天下午","temp":23,"humd":34,"overview":"雨天"}'
```

English Demo:

```sh
cargo run test.jpg '{"title":"Fucking meow hot!","time":"This PM","temp":23,"humd":34,"overview":"Rainy"}'
```
