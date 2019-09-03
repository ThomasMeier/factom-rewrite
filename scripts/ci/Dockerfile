FROM 'rust/stretch'

# test dependencies for the system
RUN apt-get update; \
    apt-get install -y clang; \
    rustup component add rustfmt; \
    rustup component add clippy;

# gitlab-runner complains for this
ENV TERM xterm-256color
