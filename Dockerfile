# Intermediate builder image
FROM debian:10 as builder

RUN apt-get update -y
RUN apt-get install libzmq3-dev -y

# Release image
FROM gcr.io/distroless/cc-debian10

COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /lib/ /lib/
COPY ./target/release/galos-sync /usr/local/bin/galos-sync

ENTRYPOINT [ "galos-sync", "eddn"]
