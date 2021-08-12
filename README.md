# cqrs-grpc-demo

**A demo application using the [cqrs-es2](https://github.com/brgirgis/cqrs-es2 framework and postgres persistence.**

## Requirements

- rust stable
- docker & [docker-compose](https://docs.docker.com/compose/) for starting an instance of Postgres
- [postman](https://www.postman.com/) (or curl or your favorite Restful client)

Alternatively, if a a standard Postgres instance is running locally it can be utilized instead of the docker instance,
see [the init script](db/init.sql) for the expected table configuration.

## Installation

Clone this repository

    git clone https://github.com/brgirgis/cqrs-restful-demo

Enter the project folder and start postgress

    cd cqrs-restful-demo
    docker-compose up -d

Start the application

    cargo run --bin server

Call the API using the provided client.

### Documentation

[Documentation can be found here](https://doc.rust-cqrs.org/)
for the CQRS and event sourcing portions of this demo application.
