# Intermediate builder image
FROM debian:10 as builder

ENV SQLX_OFFLINE true

RUN apt-get update -y
RUN apt-get install libzmq3-dev -y

# Release image
FROM gcr.io/distroless/cc-debian10

COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /lib/ /lib/
COPY ./target/release/galos-sync /usr/local/bin/galos-sync

ENV DATABASE_URL "postgresql://bgsadmin:ELt!j3220%9nm32lt@newpbgs.cwg4n3ita3jl.eu-central-1.rds.amazonaws.com/elite_development"
ENTRYPOINT [ "galos-sync", "eddn"]
