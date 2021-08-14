
all:
	make clean
	make build

clean:
	rm -rf target

build:
	cargo build

test:
	cargo test

up:
	docker-compose up -d

down:
	docker-compose down

run:
	cargo run --bin server
