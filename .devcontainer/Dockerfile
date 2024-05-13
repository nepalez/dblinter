# These args won't outlive FROM instruction
ARG RUST_VERSION
ARG DISTRO_NAME
FROM rust:$RUST_VERSION-$DISTRO_NAME
LABEL maintainer="Andréi Kózin aka nepalez"

# Install system dependencies
RUN rustup component add rustfmt clippy && \
    apt-get update -qq && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yq --no-install-recommends \
      build-essential \
      ca-certificates \
      curl \
      git \
      gnupg \
      libssl-dev \
      pkg-config \
      sudo \
      unzip \
      wget

# Install PostgreSQL client
ARG DISTRO_NAME
ARG PG_VERSION
RUN curl -sSL https://www.postgresql.org/media/keys/ACCC4CF8.asc | gpg --dearmor -o /usr/share/keyrings/postgres-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/postgres-archive-keyring.gpg] https://apt.postgresql.org/pub/repos/apt/" \
      $DISTRO_NAME-pgdg main $(echo $PG_VERSION | grep -Eo ^[0-9]+) | tee /etc/apt/sources.list.d/postgres.list > /dev/null
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    --mount=type=tmpfs,target=/var/log \
    apt-get update -qq && \
    DEBIAN_FRONTEND=noninteractive apt-get -yq dist-upgrade && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yq --no-install-recommends \
      libpq-dev \
      postgresql-client-$(echo $PG_VERSION | grep -Eo ^[0-9]+)

# Create a directory for the project volume
RUN mkdir -p /app
WORKDIR /app

# Configure git aliases
COPY ./.gitconfig /root/
RUN git config --global --add safe.directory /app

CMD ["/bin/bash"]
