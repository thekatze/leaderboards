# Step 1 build tailwindcss
FROM node:20-slim AS tailwind-builder
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /app
WORKDIR /app

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build:styles

# Step 2 build rust
FROM rust:slim as builder

# Cache rust dependencies
RUN USER=root cargo new --bin leaderboards
WORKDIR ./leaderboards
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# Build rust app
ADD . ./
COPY --from=tailwind-builder /app/src/routes/styles_generated.css src/routes/styles_generated.css

RUN rm target/release/deps/leaderboards*
RUN cargo build --release

# Copy application to runner image
FROM debian:12-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /leaderboards/target/release/leaderboards ${APP}/leaderboards

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./leaderboards"]

