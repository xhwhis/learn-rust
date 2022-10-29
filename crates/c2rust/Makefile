all: clean build

clean:
	cargo clean
	rm -f a.out

build:
	cargo build -r
	cc main.c -o a.out -lc2rust -L../../target/release

.PHONY: clean

run:
	./a.out
