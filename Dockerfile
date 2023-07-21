FROM node:alpine as web-builder
WORKDIR /usr/src
COPY client/package.json .
COPY client/package-lock.json .
RUN npm install
COPY client/ .
RUN npm run build

FROM rust:alpine as builder
WORKDIR /usr/src/api-service
RUN apk add --no-cache musl-dev

# Update this to whatever database provider you use
ENV RUSTFLAGS="-C target-feature=-crt-static" 

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml


COPY src/ src
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /usr/src/api-service/target/release/actix_svelte_surreal /app

COPY --from=web-builder /usr/src/build /app/static

ENV STATIC_FILE_PATH=/app/static PORT=8080
# Replace with your database connection string if not using sqlite
EXPOSE 8080
CMD ["/app/actix_svelte_surreal"]
