FROM debian:stretch-slim

# This dockerfile assumes that the node has already been built locally.
# To ensure that run the command `cargo build --release -p kitchen-node`

# Copy the node into the image
COPY ./target/release/kitchen-node .

# Open default ports. User is responsible for re-mapping these, using
# host networking, or otherwise resolving their port-related needs.
EXPOSE 30333 9933 9944

ENTRYPOINT ["./kitchen-node"]
