FROM rust as builder
WORKDIR /usr/src/karuta-backend
COPY . .
RUN cargo install --path .

FROM debian
WORKDIR /usr/local/karuta-backend
COPY --from=builder /usr/local/cargo/bin/karuta_backend /usr/local/bin/karuta_backend
COPY ./decks ./decks
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["karuta_backend"]
