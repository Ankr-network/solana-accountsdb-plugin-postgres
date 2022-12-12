FROM rust:1.60.0-bullseye AS build
WORKDIR /build

RUN apt-get update && apt-get -y install sudo wget lsb-release gnupg2 libssl-dev pkg-config libclang-dev

COPY . .
RUN ci/install-build-deps.sh

RUN ci/cargo-build-release.sh
RUN ls target/release

# FROM scratch AS export
# COPY --from=build /build/target/release/libholaplex_indexer_rabbitmq_geyser.so /