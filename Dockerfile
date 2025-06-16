FROM ubuntu:25.04 AS builder

RUN apt-get update
RUN apt-get install -y --no-install-recommends curl ca-certificates build-essential pkg-config libssl-dev git bash

ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
ENV SQLX_OFFLINE=true

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release || true

RUN rm -rf src

COPY . .

RUN cargo build --release

FROM ubuntu:25.04 AS style
RUN apt-get update
RUN apt-get install -y --no-install-recommends curl tar ca-certificates

RUN apt-get clean
RUN rm -rf /var/lib/apt/lists/*

RUN curl "https://github.com/sass/dart-sass/releases/download/1.89.0/dart-sass-1.89.0-linux-x64.tar.gz" -Lo "dart-sass-1.89.0-linux-x64.tar.gz"
RUN tar -xvf "dart-sass-1.89.0-linux-x64.tar.gz" -C /opt
RUN ln -s /opt/dart-sass/sass /usr/bin/sass

WORKDIR /app

COPY ./style ./style

RUN sass "style/style.scss" "dist/style.css" --style compressed

FROM ubuntu:25.04

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  curl \
  ca-certificates \
  libdbus-1-3 \
  libglib2.0-0 \
  libssl3 \
  libatk1.0-0 \
  libatk-bridge2.0-0 \
  libexpat1 \
  libxcomposite1 \
  libxdamage1 \
  libxfixes3 \
  libxrandr2 \
  libgbm1 \
  libxkbcommon-x11-0 \
  libasound2t64 \
  libgcc-s1 \
  libstdc++6 \
  libx11-6 \
  bash \
  fonts-liberation \
  unzip \
  libnss3
RUN apt-get clean
RUN rm -rf /var/lib/apt/lists/*

ENV CHROME_VERSION=136.0.7103.94
ENV ARCH=linux64

RUN curl -Lo chrome.zip "https://storage.googleapis.com/chrome-for-testing-public/${CHROME_VERSION}/${ARCH}/chrome-headless-shell-${ARCH}.zip"
RUN unzip chrome.zip
RUN rm chrome.zip
RUN mv chrome-headless-shell-${ARCH} /opt/chrome-headless-shell
RUN ln -s /opt/chrome-headless-shell/chrome-headless-shell /usr/bin/chrome

RUN curl -Lo chromedriver.zip "https://storage.googleapis.com/chrome-for-testing-public/${CHROME_VERSION}/${ARCH}/chromedriver-${ARCH}.zip"
RUN unzip chromedriver.zip
RUN rm chromedriver.zip
RUN mv chromedriver-${ARCH} /opt/chromedriver
RUN chmod u+x /opt/chromedriver/chromedriver
RUN ln -s /opt/chromedriver/chromedriver /usr/bin/chromedriver

WORKDIR /app

COPY --from=builder /app/target/release/khanza-klaim-server /app/khanza-klaim-server
COPY --from=style /app/dist /app/dist

RUN mkdir /app/temp /app/exported /app/assets

LABEL org.opencontainers.image.source=https://github.com/ilhamfu/khanza-claim-server

ENTRYPOINT ["/app/khanza-klaim-server"]
