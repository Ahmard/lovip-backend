# Build
FROM rust:1.70 as planner
RUN cargo install cargo-chef

# Set work directory
WORKDIR /usr/src/lovip-backend
COPY . .

# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.70 as build
RUN cargo install cargo-chef

# Install dependencies
RUN apt-get update && apt-get install -y clang ffmpeg libavcodec-dev libavformat-dev libavutil-dev pkg-config \
    && apt-get install -y libavfilter-dev libavdevice-dev

# Copy the build plan from the previous Docker stage
COPY --from=planner /usr/src/lovip-backend/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --recipe-path recipe.json

# Build the whole project
COPY . .

# Setup working directory
WORKDIR /lovip

# Build application
RUN cargo build --release

# BUILD
FROM rust:1.70 AS runtime

# Install dependencies
RUN apt-get update && apt-get install -y clang ffmpeg libavcodec-dev libavformat-dev libavutil-dev pkg-config \
    && apt-get install -y libavfilter-dev libavdevice-dev

COPY .env .env
COPY static static

# Copy our built binary
COPY --from=build /target/release/lovip-backend /usr/local/bin/lovip

CMD ["lovip"]