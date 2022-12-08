all: clean build test

clean:
	cargo clean
	rm -f a.out

build:
	cargo build -r

test:
	cc main.c -o a.out -lrhai_test -L./target/release
	./a.out
