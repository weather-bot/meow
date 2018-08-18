build:
	cargo build

test:
	cargo run test.jpg '{"title":"今天他喵的會下雨！","time":"明天下午","temp":23,"humd":34,"overview":"雨天"}'

lint:
	rustfmt -f src/main.rs