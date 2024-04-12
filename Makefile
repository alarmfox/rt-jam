## up: executes the application using docker-compose
up:
	docker compose  up
## down: deletes docker containers
down:
	docker compose down 

## build-images: build docker images
build-images:
	docker compose build

## build: statically build frontend and backend
build:
	cargo build --bin backend --release
	RUSTFLAGS=--cfg=web_sys_unstable_apis trunk build --release

## dev: creates nats and postgres container; executes backend and frontend locally
dev:
	cargo watch -wq backend/ -w common -x "cargo run --bin backend" &
	RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve

.PHONY: help
## help: prints this help message
help:
	@echo "Usage: "
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'
