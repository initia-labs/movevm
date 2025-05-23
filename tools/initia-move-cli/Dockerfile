FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends git ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
COPY initia-move-cli .
RUN chmod +x initia-move-cli

ENTRYPOINT ["initia-move-cli"]