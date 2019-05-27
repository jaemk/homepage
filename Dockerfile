FROM rust:1.35

WORKDIR /usr/src/homepage
COPY . .

RUN cargo install --path .

CMD ["homepage", "serve", "--port", "8000", "--public"]
