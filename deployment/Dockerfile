FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /youtube_downloader

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.

WORKDIR /youtube_downloader

# Copy our build
COPY --from=builder /myip/target/x86_64-unknown-linux-musl/release/youtube_downloader ./


CMD ["/youtube_downloader/youtube_downloader"]