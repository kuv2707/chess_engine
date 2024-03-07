# Use an official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to optimize caching
COPY Cargo.toml Cargo.lock ./

# Build dependencies (this step is separate to cache dependencies)
RUN mkdir src && echo "fn main() {println!(\"dummy\");}" > src/main.rs
RUN cargo build --release
RUN rm -r src

# Copy the entire project to the container
COPY . .

# Build the application
RUN cargo build --release

# Create a new image with only the compiled application
FROM ubuntu:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled application from the builder stage
COPY --from=builder /app/target/release/chess_engine /app
COPY --from=builder /app/ubuntu /app

# Expose the port your Actix Web app listens on
EXPOSE 8080

# Set the environment variable to configure Actix Web
ENV ROCKET_ENV production

# Command to run your Actix Web application
CMD ["./chess_engine"]
