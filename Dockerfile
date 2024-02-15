# syntax=docker/dockerfile:1.6

ARG BUILDER_BASE=rust:bullseye
ARG RUNTIME_BASE=debian:bullseye

FROM --platform=${TARGETPLATFORM} ${BUILDER_BASE} AS builder

RUN apt-get update \
  && apt-get install -y \
  clang \
  unzip \
  && rm -rf /var/lib/apt/lists/*

ARG PROTOC_VERSION="21.6"
RUN mkdir /protobuf \
  && cd /protobuf \
  && export PROTOC_ARCH="$(python3 -c 'import platform;m=platform.machine();m="aarch_64" if m=="aarch64" else m;print(m)')" \
  && curl -sSL -o protoc.zip "https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-${PROTOC_ARCH}.zip" \
  && unzip protoc.zip \
  && cp bin/protoc /usr/local/bin/protoc \
  && cp -r include/* /usr/local/include \
  && cd .. \
  && rm -rf /protobuf \
  && protoc --version

FROM --platform=${TARGETPLATFORM} ${RUNTIME_BASE} AS runtime

RUN apt-get update \
  && apt-get install -y \
  libssl1.1 \
  ca-certificates \
  jq \
  curl \
  && rm -rf /var/lib/apt/lists/*

FROM --platform=${TARGETPLATFORM} builder AS build

WORKDIR /build

# Build the binaries.
RUN \
  --mount=type=bind,target=.,readwrite \
  --mount=type=cache,target=/usr/local/rustup,id=${TARGETPLATFORM} \
  --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} \
  --mount=type=cache,target=target,id=${TARGETPLATFORM} \
  RUST_BACKTRACE=1 \
  cargo build --release --workspace

# Copy the binaries out.
RUN --mount=type=cache,target=target,id=${TARGETPLATFORM} \
  mkdir -p /artifacts \
  && cd target/release \
  && cp -t /artifacts \
  humanode-peer \
  robonode-server \
  robonode-keygen \
  && ls -la /artifacts

FROM --platform=${TARGETPLATFORM} runtime AS humanode-peer
COPY --from=build /artifacts/humanode-peer /usr/local/bin
RUN ldd /usr/local/bin/humanode-peer
CMD ["humanode-peer"]

FROM --platform=${TARGETPLATFORM} runtime AS robonode-server
COPY --from=build /artifacts/robonode-server /usr/local/bin
RUN ldd /usr/local/bin/robonode-server
CMD ["robonode-server"]

FROM --platform=${TARGETPLATFORM} runtime AS robonode-keygen
COPY --from=build /artifacts/robonode-keygen /usr/local/bin
RUN ldd /usr/local/bin/robonode-keygen
CMD ["robonode-keygen"]

FROM --platform=${TARGETPLATFORM} runtime AS aio
COPY --from=build /artifacts/humanode-peer /usr/local/bin
COPY --from=build /artifacts/robonode-server /usr/local/bin
COPY --from=build /artifacts/robonode-keygen /usr/local/bin
RUN ldd /usr/local/bin/humanode-peer \
  && ldd /usr/local/bin/robonode-server \
  && ldd /usr/local/bin/robonode-keygen

FROM --platform=${TARGETPLATFORM} scratch
# We put the dummy image last to force users to
# use the `docker build . --target <build stage>` invocation.
