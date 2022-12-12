FROM rust:1.58.1-slim-bullseye AS build
WORKDIR /build

RUN apt-get update && apt-get -y install sudo wget lsb-release gnupg2 libssl-dev pkg-config

COPY . .
RUN ci/install-build-deps.sh

RUN ci/cargo-build-test.sh
RUN ls target

# FROM scratch AS export
# COPY --from=build /build/target/release/libholaplex_indexer_rabbitmq_geyser.so /