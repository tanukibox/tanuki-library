#########################################
############## Build stage ##############
#########################################

FROM rust:1.80 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

##########################################
############## Runtime stage #############
##########################################

FROM rust:1.80-slim

RUN apt-get update
RUN apt-get install -y ca-certificates
RUN rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/tanukeys /app/

ENV PORT=3030 \
    RUST_LOG=main_http_api=trace,kernel=trace \
    DATABASE_URL=postgres://root:root@postgres:5432/tanukilibrary

CMD ["./tanukilibrary"]
