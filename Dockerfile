FROM rust:1.69.0-bullseye as chef
RUN cargo install cargo-chef --locked

WORKDIR /app

RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
    cmake \
    g++ \
    libsasl2-dev \
    libssl-dev \
    libudev-dev \
    pkg-config \
    wget \
  && \
  rm -rf /var/lib/apt/lists/*

COPY ci/get-protoc.sh ./
RUN chmod +x get-protoc.sh
RUN /app/get-protoc.sh

FROM chef AS planner
COPY Cargo.* ./
COPY app app
COPY migration migration
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.* ./
COPY app app
COPY migration migration

FROM builder AS builder-hub-webhooks
RUN cargo build --release --bin holaplex-hub-webhooks

FROM builder AS builder-migration
RUN cargo build --release --bin migration

FROM debian:bullseye-slim as base
WORKDIR /app
RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    libssl1.1 \
  && \
  rm -rf /var/lib/apt/lists/*

FROM base AS hub-webhooks
ENV TZ=Etc/UTC
ENV APP_USER=runner

RUN groupadd $APP_USER \
    && useradd --uid 10000 -g $APP_USER $APP_USER \
    && mkdir -p bin

RUN chown -R $APP_USER:$APP_USER bin

USER 10000
COPY --from=builder-hub-webhooks /app/target/release/holaplex-hub-webhooks /usr/local/bin
CMD ["/usr/local/bin/holaplex-hub-webhooks"]

FROM base AS migrator
COPY --from=builder-migration /app/target/release/migration /usr/local/bin
CMD ["/usr/local/bin/migration"]
