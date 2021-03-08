FROM gcr.io/distroless/cc-debian10

COPY ./galos-sync /usr/local/bin/galos-sync

ENTRYPOINT [ "galos-sync" ]