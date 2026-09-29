FROM rust:1-bookworm AS rust-base

RUN apt-get update \
    && apt-get install -y --no-install-recommends curl pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# Cargo requires a target to be present even when only fetching dependencies.
RUN mkdir src \
    && printf 'fn main() {}\n' > src/main.rs \
    && cargo fetch --locked \
    && rm -rf src
COPY build.rs ./
COPY migrations ./migrations

FROM rust-base AS development

RUN rustup component add rustfmt clippy
COPY src ./src
EXPOSE 8080
CMD ["cargo", "run", "--locked"]

FROM rust-base AS build

COPY src ./src
RUN cargo build --locked --release

FROM debian:bookworm-slim AS production

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home fitness

COPY --from=build /app/target/release/fitness-backend /usr/local/bin/fitness-backend

USER 10001:10001
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/fitness-backend"]
