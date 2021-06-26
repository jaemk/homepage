FROM rust:1.53 as builder

# create a new empty shell
RUN USER=root cargo new --bin homepage
WORKDIR /homepage

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy all source files
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/homepage*
RUN cargo build --release

COPY ./.git .git
RUN git rev-parse HEAD | head -c 7 | awk '{ printf "%s", $0 >"commit_hash.txt" }'
RUN rm -rf .git

FROM debian:buster-slim
RUN mkdir /homepage
WORKDIR /homepage

RUN mkdir ./bin
COPY --from=builder /homepage/target/release/homepage ./homepage
COPY --from=builder /homepage/commit_hash.txt ./commit_hash.txt

# copy all static files
COPY ./static ./static
COPY ./templates ./templates

# set the startup command to run your binary
CMD ["./homepage"]
