# Rust as the base image
FROM rust:1.72.1 as build

# create a new empty shell project
RUN USER=root cargo new --bin auth-api
WORKDIR /auth-api

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN apt update && apt install -y libssl-dev pkg-config

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/auth_api*

# install dependencies
RUN apt install -y build-essential 
RUN apt-get install -y libmariadb-dev

# build for release
RUN cargo build --release

# our final base
FROM rust:1.72.1

# copy the build artifact from the build stage
COPY --from=build /auth-api/target/release/auth-api .

# set the startup command to run your binary
CMD ["./auth-api"]