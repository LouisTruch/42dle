FROM rust

WORKDIR /app

COPY ./ ./

RUN cargo install cargo-watch

ENV ROCKET_ADDRESS=0.0.0.0

CMD ["cargo", "watch", "-x", "run"]
