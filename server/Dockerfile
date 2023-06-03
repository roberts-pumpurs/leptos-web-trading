FROM debian:bookworm-slim AS runner

# copy app from builder
WORKDIR /app
COPY ./target/server/release/server .
COPY ./target/site ./target/site

# Start the application
ENV LEPTOS_OUTPUT_NAME=frontend
ENTRYPOINT ["./server"]
