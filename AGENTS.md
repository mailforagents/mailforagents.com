# Mail for Agents marketing site

This is the public marketing and content repository for Mail for Agents.

## Repository boundary

- Public product: `https://github.com/mailforagents/mailforagents`
- Public marketing site: this repository, `https://github.com/mailforagents/mailforagents.com`
- Private hosted operations: `https://github.com/mailforagents/mailforagents-cloud`
- Product requirements live in the public product repository and outrank marketing shorthand.
- Never copy private cloud code, secrets, customer identifiers, production topology, internal metrics, or incident data here.
- This site may link to and explain public product capabilities. It must not imply that planned features already ship.

## Stack

- Rust 2024, Axum, Askama, and Tower HTTP.
- Server-render the first useful response.
- Add HTMX only for a bounded interaction with a normal form or link fallback.
- Keep content in Git under `content/`; do not introduce a database or CMS without an explicit decision. If site persistence becomes necessary, SQLite is the only approved v1 database.
- Keep JavaScript optional and small. Do not add a SPA framework.

## Design direction

The site should feel like precise field instrumentation for lifecycle systems: editorial, dark, calm, and technically credible. Use warm signal orange only for meaningful emphasis. Avoid generic AI gradients, robot imagery, decorative sparkles, fake dashboards, and unsupported social proof.

- Preserve visible keyboard focus, semantic HTML, 44px touch targets, and readable contrast.
- Honor reduced motion, reduced transparency, and increased contrast.
- Motion must explain hierarchy or response; never loop merely for atmosphere.
- Use original assets and copy. Do not reproduce another product's interface or trade dress.

## Content and claims

- Distinguish shipped, preview, and planned capabilities.
- Do not invent customers, revenue, benchmarks, testimonials, integrations, citations, or deliverability claims.
- Prefer specific product language: playbook, workflow, event, message, broadcast, topic, agent.
- Explain that AWS SES is customer-owned by default and that the operator remains responsible for compliance and deliverability.

## Change workflow

1. Read this file, `README.md`, and the relevant product PRD sections.
2. Make one coherent change and update affected routes, templates, content, metadata, and tests together.
3. Run `cargo fmt --check`, `cargo check --all-targets`, and `cargo test`.
4. For visual changes, inspect desktop and narrow layouts plus keyboard and reduced-motion behavior.
5. Keep commits scoped and explain any cross-repository dependency with a shared change ID.
