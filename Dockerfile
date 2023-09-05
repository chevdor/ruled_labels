FROM docker.io/paritytech/ci-linux:production as builder

ARG PROFILE=release
WORKDIR /app

COPY . .

RUN cargo build --profile ${PROFILE} --bins

# MAIN IMAGE FOR PEOPLE TO PULL --- small one#
FROM docker.io/debian:buster-slim
LABEL maintainer="Chevdor"

ARG PROFILE=release
ARG USER=ci
WORKDIR /usr/local/bin

COPY --from=builder /app/target/$PROFILE/ruled-labels /usr/local/bin

RUN useradd -m -u 1000 -U $USER && \
    apt-get -y update && \
    apt-get -y install openssl && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/

USER $USER
ENTRYPOINT [ "ruled-labels" ]
