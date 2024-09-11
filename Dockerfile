FROM rust:latest
ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /app
COPY ./ /app
RUN cargo build --release
RUN strip target/release/bike-stations

FROM rust:latest
COPY --from=0 /app/target/release/bike-stations .
EXPOSE 3000
CMD ["/bike-stations"]
