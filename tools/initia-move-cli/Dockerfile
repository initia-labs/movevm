FROM debian:bullseye-slim

WORKDIR /usr/local/bin
COPY initia-move-cli .
RUN chmod +x initia-move-cli

ENTRYPOINT ["initia-move-cli"]