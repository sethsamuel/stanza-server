FROM debian:jessie

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    libc6-dev \
    libssl-dev \
  && rm -rf /var/lib/apt/lists/*


COPY target/release/stanza-server /stanza-server

CMD /stanza-server
