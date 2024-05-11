clean:
	rm -f ./exp 2>/dev/null;
	rm -f ./*.csv 2>/dev/null;

build:
	cargo build --release
	rm -f ./exp 2>/dev/null;
	mv -f ./target/release/exp ./exp

run: build
	./exp

run_exp1: build
	./exp random bench 10000 --filter=bf > exp1.csv

run_exp2: build
	./exp enum enum 4 --filter=bf > exp2.csv

run_exp3: build
	./exp satc2023 run

test:
	cargo test

gen:
	./dataset/gen.sh