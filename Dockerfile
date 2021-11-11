FROM rust:1.50.0-slim-buster as build


# create a new empty shell project
RUN USER=root cargo new --bin web-server
WORKDIR /web-server

# copy over your manifests
COPY Cargo.toml Cargo.lock ./

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/web_server*
RUN cargo build --release

# our final base
FROM rust:1.50.0-slim-buster

# copy the build artifact from the build stage
COPY --from=build /web-server/target/release/web-server .

COPY ./hello.html .

EXPOSE 7878

# set the startup command to run your binary
CMD ["./web-server"]