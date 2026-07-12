FROM rust:1.93-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --locked --release

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 --create-home app
WORKDIR /app
COPY --from=builder /app/target/release/mailforagents-site /usr/local/bin/mailforagents-site
COPY --from=builder /app/static ./static
USER app
ENV MFA_SITE_ADDR=0.0.0.0:3000
EXPOSE 3000
ENTRYPOINT ["mailforagents-site"]

