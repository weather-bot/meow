build:
	cargo build

test-light:
	cargo build
	./target/debug/meow -o light_out.jpg light-mode test.jpg '{"title":"多雲將持續一整天！","location":"台北","time":"02/01","temp":29,"humd":34,"overview":"空氣良好","overview2":"悶熱"}'

help:
	./target/debug/meow -h

lint:
	find ./src -name "*.rs" -exec rustfmt {} \;

check:
	find ./src -name "*.rs" -exec rustfmt --check {} \;
