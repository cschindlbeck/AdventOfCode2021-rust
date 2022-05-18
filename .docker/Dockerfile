FROM rust:1.57

RUN rustup component add rustfmt

# Install missing packages in ubuntu docker container
RUN apt-get update && apt-get install -y --no-install-recommends \
    vim \
    && apt-get clean all \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash chris
WORKDIR /home/chris/rust
COPY . .

#RUN cargo install --path .

#CMD ["myapp"]
