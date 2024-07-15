FROM rust:1.79.0-slim-buster as build

# DEPENDECIES
RUN apt-get update
RUN apt-get install --assume-yes pkg-config
RUN apt-get install --assume-yes libssl-dev
RUN USER=root cargo new --bin pxollyrs

WORKDIR /pxollyrs

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/pxollyrs*
RUN cargo build --release

# START
FROM rust:1.79.0-slim-buster

COPY --from=build /pxollyrs/target/release/pxollyrs .
COPY ./conf ./conf

CMD ["./pxollyrs"]