# Build
FROM rust:1.70 as build
WORKDIR /usr/src/devops-api
COPY . .
RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/devops-api/target/release/devops-api /usr/local/bin/devops-api
EXPOSE 8000
CMD ["devops-api"]
