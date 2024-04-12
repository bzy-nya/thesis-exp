clean:
	rm -f ./exp 2>/dev/null;
	rm -f ./*.csv 2>/dev/null;

build:
	cargo build
	rm -f ./exp 2>/dev/null;
	mv -f ./target/debug/exp ./exp

run: build
	./exp

run_exp1: build
	./exp random bench 10000 > exp1.csv

run_exp2: build
	./exp satc2023 bench --filter=lll 100 > exp2.csv

run_exp3: build
	./exp enum enum 3 > exp3.csv

test:
	cargo test