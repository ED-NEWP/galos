FROM gcr.io/distroless/cc-debian10

COPY ./target/release/galos-sync /usr/local/bin/galos-sync

ENTRYPOINT [ "galos-sync" ]