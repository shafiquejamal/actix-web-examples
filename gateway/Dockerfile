FROM rust:1.64-bullseye

COPY . /app
WORKDIR /app/src/front-end 

RUN apt-get update && apt-get -y upgrade && apt-get install -y musl-dev make cmake libpq-dev build-essential\
    curl wget vim less tmux && curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash &&\
    export NVM_DIR="$HOME/.nvm" && [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" &&\
    [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion" && nvm install 18.10.0 && nvm use 18.10.0 &&\
    node --version && npm install --global yarn && yarn install && yarn build

WORKDIR /app
RUN RUST_BACKTRACE=full cargo build -vv --release
EXPOSE 8080

CMD ["./target/release/gateway"]
