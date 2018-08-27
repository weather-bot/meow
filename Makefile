build:
	cargo build

test:
	cargo build
	./target/debug/meow corner-mode test.jpg '{"title":"今天他喵的會下雨！","location":"台北","time":"明天下午","temp":29,"humd":34,"overview":"短暫陣雨","overview2":"悶熱"}'

test-bottom:
	cargo build
	./target/debug/meow bottom-mode test.jpg '{"title":"今天他喵的會下雨！","location":"台北","time":"明天下午","temp":29,"humd":34,"overview":"短暫陣雨","overview2":"悶熱"}'

help:
	./target/debug/meow -h

lint:
	find ./src -name "*.rs" -exec rustfmt {} \;

check:
	find ./src -name "*.rs" -exec rustfmt --check {} \;
