#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
if [[ -z "${CARGO_TARGET_DIR:-}" && -d /mnt/backup/build-cache ]]; then
  export CARGO_TARGET_DIR="/mnt/backup/build-cache/openbim-okstra-target"
fi

step() { printf '\n==> %s\n' "$*"; "$@"; }

step cargo fmt --all -- --check
step cargo build --workspace --all-targets --locked
step cargo test --workspace --all-targets --locked
step cargo test --workspace --doc --locked
step cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" step cargo doc --workspace --no-deps --locked

# OKSTRA schema, model and documentation files (XSD, XMI, EA projects, PDF)
# have no established redistribution rights. They belong in the ignored
# references/ directory and must never be tracked or packaged.
standards_payload='\.(xsd|xmi|qea|eap|pdf|zip)$'
if git rev-parse --verify HEAD >/dev/null 2>&1 \
  && git ls-files | grep -Ei "$standards_payload"; then
  echo "gate: standards payload tracked in Git (see above)" >&2
  exit 1
fi

step cargo package -p openbim-okstra --allow-dirty --locked
listing="$(cargo package -p openbim-okstra --allow-dirty --locked --list)"
printf '%s\n' "$listing"
if printf '%s\n' "$listing" | grep -Ei "$standards_payload|^references/"; then
  echo "gate: standards payload in the crate package (see above)" >&2
  exit 1
fi

# Read the version rather than hardcoding it: a hardcoded path silently breaks
# the packaged-crate test on every version bump.
version="$(cargo metadata --format-version 1 --no-deps \
  | python3 -c 'import json,sys; print(next(p["version"] for p in json.load(sys.stdin)["packages"] if p["name"]=="openbim-okstra"))')"
package_root="${CARGO_TARGET_DIR:-target}/package/openbim-okstra-${version}"
test -d "$package_root" || {
  echo "packaged crate not found at $package_root" >&2
  exit 1
}
test -f "$package_root/README.md"
test -f "$package_root/LICENSE"
step cargo test --manifest-path "$package_root/Cargo.toml"

echo
echo "gate: PASS"
