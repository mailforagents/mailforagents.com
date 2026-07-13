# mailforagents.com

Public marketing and content site for [Mail for Agents](https://github.com/mailforagents/mailforagents), built with Axum and Askama.

## Run locally

```bash
cargo run
```

The server listens on `127.0.0.1:3000` by default. Override it with `MFA_SITE_ADDR`.

```bash
MFA_SITE_ADDR=0.0.0.0:8080 cargo run
```

## Verify

```bash
cargo fmt --check
cargo check --all-targets
cargo test
```

Content collections live in `content/`. They are placeholders for reviewed playbook, integration, comparison, and article pages; the homepage does not claim those pages already exist.

## Production deployment

Pushes to `main` run `.github/workflows/deploy.yml`. The workflow verifies the Rust project, builds the Docker image on GitHub Actions, publishes immutable `sha-<commit>` and moving `main` tags to `ghcr.io/mailforagents/mailforagents.com`, updates the Coolify resource to the immutable tag, and asks Coolify to pull and run it.

The GitHub `production` environment requires these settings:

- secrets: `COOLIFY_URL`, `COOLIFY_TOKEN`
- variable: `COOLIFY_RESOURCE_UUID`

Coolify is a runtime and deployment target only; it does not compile this project.
