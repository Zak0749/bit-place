FROM lukemathwalker/cargo-chef AS chef
WORKDIR /app

FROM chef AS planner
COPY ["backend/Cargo.toml", "backend/Cargo.lock", "./"]
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS backend 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY /backend/src src
RUN cargo build --release --bin backend

FROM node as frontend
WORKDIR /app
COPY ["frontend/package.json","frontend/package-lock.json", "./"]
RUN npm install
COPY frontend/public public
COPY frontend/src src
COPY frontend/rollup.config.js rollup.config.js
RUN npm run build

FROM debian:buster-slim
WORKDIR /app
COPY --from=backend /app/target/release/backend /usr/local/bin
COPY --from=frontend /app/public /usr/local/public
EXPOSE 8080
ENV REDIS_URL = "db:6379"
ENTRYPOINT ["/usr/local/bin/backend"]
