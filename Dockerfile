FROM rustlang/rust:nightly AS builder
WORKDIR /app
ADD Cargo.toml Cargo.lock /app/
RUN mkdir -p /app/src && echo "fn main() {}" > /app/src/main.rs &&\
    touch -d 1980-01-01 /app/src/main.rs && \
    cargo build --release

ADD . /app
RUN rm /app/target/*/merlin ; cargo build --release

FROM rustlang/rust:nightly
WORKDIR /app
COPY run.sh templates Rocket.toml /app/
COPY --from=builder /app/target/release/merlin /app/merlin

CMD ["/app/run.sh"]