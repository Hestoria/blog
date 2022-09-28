FROM rust:latest as planner
WORKDIR /usr/src/api
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres
COPY . .

FROM planner as build
RUN cargo build --release

FROM rust:slim-buster
COPY --from=build /usr/src/api/target/release/api .
EXPOSE 8080
CMD ["/usr/local/bin/api"]