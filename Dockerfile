FROM rust:slim-buster as build
WORKDIR /usr/src/echo-server
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /usr/src/echo-server/target/release/echo-server /usr/bin/
CMD ["echo-server"]