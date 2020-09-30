FROM clux/muslrust:1.46.0-stable AS build

RUN mkdir -p /var/app
WORKDIR /var/app
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

RUN mkdir -p /var/app
WORKDIR /var/app
COPY --from=build /var/app/target/x86_64-unknown-linux-musl/release/short-url /var/app/short
RUN chmod +x /var/app/short

ENTRYPOINT ["/var/app/short"]
