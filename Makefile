clean:
	rm -f ./exp 2>/dev/null;
	rm -f ./*.csv 2>/dev/null;

build: clean
	cargo build
	mv ./target/debug/exp ./exp

run: build
	./exp

run_exp1: build
	./exp random bench 10000 > exp1.csv

run_exp2: build
	echo qwq

test:
	cargo test