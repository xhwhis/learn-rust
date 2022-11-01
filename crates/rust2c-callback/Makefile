all: clean build

clean:
	cargo clean
	rm -f a.out

build:
	cargo build -r
	cc main.c -o a.out -lrust2c_callback -L../../target/release

.PHONY: clean

run:
	./a.out
