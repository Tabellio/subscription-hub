ARM_VERSION="0.12.13"
INTEL_VERSION="0.12.13"

function optimize_arm() {
  docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:$ARM_VERSION
}

function optimize_intel() {
  docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:$INTEL_VERSION
}

if [[ $(uname -m) =~ "arm64" ]]; then \
  optimize_arm; \
else \
  optimize_intel; \
fi
