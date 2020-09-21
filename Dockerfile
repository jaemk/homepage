FROM rust:1.46

# create a new empty shell
RUN USER=root cargo new --bin homepage
WORKDIR /homepage

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy all source/static/resource files
COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates

# build for release
RUN rm ./target/release/deps/homepage*
RUN cargo build --release

# set the startup command to run your binary
CMD ["./target/release/homepage"]
