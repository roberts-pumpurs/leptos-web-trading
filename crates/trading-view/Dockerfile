# Using a very tiny image from google to run the app!
FROM debian:bookworm-slim AS runner

# copy app from builder
WORKDIR /app
COPY ./target/server/release/trading-view .
COPY ./target/site site

# Start the application
ENTRYPOINT ["./trading-view"]
