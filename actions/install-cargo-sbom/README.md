# cargo-sbom-actions/install-cargo-sbom

Installs [cargo-sbom](https://crates.io/crates/cargo-sbom) on GitHub Actions for the supported platforms: Linux and macOS (x86_64).

## Usage

Create or update a Github workflow (`.github/workflows/<workflow>.yml`) in your repo with the following contents:

```yaml
jobs:
  sbom:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: psastras/sbom-rs/actions/install-cargo-sbom@cargo-sbom-latest
    - name: Run cargo-sbom
      run: cargo-sbom
```

## Inputs (specify using `with:`)

- `version`: version of `cargo-sbom` to use (default: `latest`)

- `cargo-sbom architecture`: which binary architecture to download from Github Releases (default: `x86_64`)

- `platform`: which os platform to download from Github Releases (default: `unknown-linux-gnu`)

License: MIT
