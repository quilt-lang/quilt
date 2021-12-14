FROM rust:1-alpine AS builder

WORKDIR /usr/src/quilt
RUN apk add build-base
ENV RUSTFLAGS="-C link-arg=-s"
COPY . .
RUN cargo install --path .

FROM scratch
LABEL org.opencontainers.image.source https://github.com/quilt-lang/quilt
COPY --from=builder /usr/local/cargo/bin/quilt /usr/local/bin/quilt
ENTRYPOINT ["quilt"]
