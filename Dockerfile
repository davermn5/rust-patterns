FROM rust:1.71-slim-bullseye

RUN apt-get update && apt-get install -y nano iproute2 iputils-ping && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .