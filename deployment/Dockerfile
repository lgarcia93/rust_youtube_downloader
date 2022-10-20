FROM rust:latest AS builder

WORKDIR /youtube_downloader

COPY ./Cargo.toml ./
COPY ./src/ ./src/
COPY ./static/ ./static/

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc-debian10

# Import from builder.

WORKDIR /youtube_downloader

# Copy our build
COPY --from=builder /youtube_downloader/target/release/server ./
COPY --from=builder /youtube_downloader/static/ ./static/

EXPOSE 5000

CMD ["/youtube_downloader/server"]