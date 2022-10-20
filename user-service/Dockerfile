FROM rust:1.64-bullseye

COPY . /app

RUN apt-get update && apt-get -y upgrade && apt-get install -y musl-dev make cmake libpq-dev build-essential\
    curl wget vim less tmux

WORKDIR /app

RUN RUST_BACKTRACE=full cargo build -vv --release

CMD ["./target/release/user-service"]
