FROM rustlang/rust:nightly AS builder
WORKDIR /app
ADD Cargo.toml Cargo.lock /app/
RUN mkdir -p /app/src && echo "fn main() {}" > /app/src/main.rs &&\
    touch -d 1980-01-01 /app/src/main.rs && \
    cargo build --release

ADD . /app
RUN rm /app/target/*/merlin ; cargo build --release

FROM buildpack-deps:stretch
WORKDIR /app
COPY run.sh Rocket.toml /app/
COPY templates /app/templates/
COPY --from=builder /app/target/release/merlin /app/merlin

CMD ["/app/run.sh"]