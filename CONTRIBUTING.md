# Contributing

Contributions are welcome. Follow the repository context files and run the
verification gate before opening a pull request.

1. Encode only facts that a public OKSTRA source states; cite it in rustdoc.
2. Never add OKSTRA schema files, XMI/EA exports, or documentation (PDF or
   HTML). Keep local copies in the ignored `references/` directory.
3. Add or update a failing test before production behavior changes.
4. Run `bash scripts/gate.sh` and add an `[Unreleased]` changelog entry for
   user-visible changes.

## Licensing contributions

Unless an explicitly signed agreement says otherwise, every contribution
submitted to this repository is licensed under `AGPL-3.0-or-later`. Submit only
work that you have the right to license. Identify third-party material and
preserve its license, attribution, and provenance.

## Releasing

Releases publish from CI through crates.io trusted publishing
(`.github/workflows/release.yml`); no one needs a crates.io token.

### First version

crates.io accepts a trusted publisher only for a crate that already exists, so
the first version (`0.1.0`) is published by hand by a maintainer, from a clean
checkout of the reviewed `main` commit, after `bash scripts/gate.sh` and
`cargo publish -p openbim-okstra --dry-run` pass. Afterwards, add a trusted
publisher in the crate's Settings -> Trusted Publishing on crates.io:
repository `openbimrs/okstra`, workflow `release.yml`, environment
`crates.io`. Then push the annotated tag `v0.1.0`: the workflow sees the
version is already live, skips publishing, and creates the GitHub release.

### Later versions

1. Bump `version` in the root `Cargo.toml`, run
   `cargo update -p openbim-okstra`, and date the `## [Unreleased]` changelog
   section as `## [x.y.z] - date` (with its compare link).
2. Run `bash scripts/gate.sh`, and `cargo semver-checks` against the last
   release to confirm the bump matches the change.
3. Merge to `main`, then push an annotated tag `vx.y.z` on that commit.

The workflow refuses a tag that is not on `main`, does not match the manifest
version, or has no changelog section; it gates the tagged commit, publishes,
and creates the GitHub release from the changelog. Re-running a partly failed
release skips what is already live. To rehearse, run the workflow by hand with
an existing tag: it gates and packages, and publishes nothing.

crates.io trusts this repository, the file name `release.yml` and the
`crates.io` environment; renaming either needs the same change there.
