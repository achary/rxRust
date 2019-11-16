FROM rustlang/rust:nightly-buster-slim

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# Configure apt and install packages
RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils dialog 2>&1 \
    #
    # Verify git, needed tools installed
    && apt-get -y install git iproute2 procps lsb-release \
    #
    # Install other dependencies
    && apt-get install -y lldb \
    # Bash autocomplet
    && apt-get install bash-completion

# Install Rust components
# to conform to what CI on rxrust is doing
RUN rustup override set nightly-2019-09-01
RUN rustup update
RUN rustup component add rls rust-analysis
RUN rustup component add rust-src
RUN rustup component add rustfmt clippy