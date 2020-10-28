# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM rust:latest AS build

# Create a project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN cargo install cargo-build-dependencies
RUN cd /tmp && USER=root cargo new bhd-rss-bot
WORKDIR /tmp/bhd-rss-bot
COPY Cargo.toml Cargo.lock ./
# build dependencies in separate step, since they are less-likely to change
RUN cargo build-dependencies --release
# Copy the source and build the application.
COPY src /tmp/bhd-rss-bot/src
RUN cargo build --release
RUN strip target/release/bhd-rss-bot

# use a slim debian container
FROM debian:buster-slim as scratch
# make sure we're running the latest
RUN apt-get update
# and security updates
RUN apt-get -y upgrade
# need ssl and ca certs
RUN apt-get -y install --no-install-recommends libssl1.1 ca-certificates
# but clean up to keep things as slim as posible
RUN apt-get clean && rm -rf /var/lib/apt/lists/*
# copy in our (nearly) static executable (still needs libssl)
COPY --from=build /tmp/bhd-rss-bot/target/release/bhd-rss-bot .
USER 1000:1000
CMD ["/bhd-rss-bot"]
