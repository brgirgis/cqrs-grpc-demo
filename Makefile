
all:
	cargo build

up:
	docker-compose up -d

down:
	docker-compose down

test:
	cargo test

run:
	cargo run
