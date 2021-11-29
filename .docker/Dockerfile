FROM rust:1.31

# Install missing packages in ubuntu docker container
RUN apt-get update && apt-get install -y --no-install-recommends \
    vim \
    && apt-get clean all \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /rust
COPY . .

#RUN cargo install --path .

#CMD ["myapp"]
