FROM rust:1-bookworm AS development

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked
COPY src ./src

EXPOSE 8080
CMD ["cargo", "run", "--locked"]

FROM rust:1-bookworm AS build

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked
COPY src ./src
RUN cargo build --locked --release

FROM debian:bookworm-slim AS production

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home fitness

COPY --from=build /app/target/release/fitness-backend /usr/local/bin/fitness-backend

USER 10001:10001
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/fitness-backend"]
