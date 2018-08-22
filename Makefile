build:
	cargo build

test:
	cargo build
	./target/debug/meow corner-mode test.jpg '{"title":"今天他喵的會下雨！","time":"明天下午","temp":29,"humd":34,"overview":"雨天"}'

help:
	./target/debug/meow -h

lint:
	find ./src -name "*.rs" -exec rustfmt -f {} \;