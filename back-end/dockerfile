FROM rust:1.49

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/holodeck"]