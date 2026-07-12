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

