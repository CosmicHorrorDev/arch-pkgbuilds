# Arch Packages

My collection of Arch packages hosted in the [AUR](https://aur.archlinux.org)

## Maintaining

Packages are currently checked in a manual process with `makepkg` and `namcap`,
checksums are updated using `updpkgsums`, and generating `.SRCINFO` along with
some final checks and publishing are handled by `aurpublish`.

In the future it would be nice to automate most of this manual work and have
some higher level automated testing that verifies some of the packages
functionality beyond just running the test suite.

## Packages

- [`cargo-chef`](https://aur.archlinux.org/packages/cargo-chef) - Cargo plugin for managing docker layer caching with Rust projects
- [`cargo-deadlinks`](https://aur.archlinux.org/packages/cargo-deadlinks) - Cargo plugin for checking your documentation for broken links
- [`cargo-pgo`](https://aur.archlinux.org/packages/cargo-pgo) - Cargo subcommand for optimizing Rust binaries with PGO and BOLT
- [`gimoji`](https://aur.archlinux.org/packages/gimoji) - Easily add emojis to your git commit messages 🎉
- [`inlyne`](https://aur.archlinux.org/packages/inlyne) and [`inlyne-bin`](https://aur.archlinux.org/packages/inlyne-bin) - A GPU powered, browserless, markdown viewer
- [`metrics-observer`](https://aur.archlinux.org/packages/metrics-observer) - A text-based UI for observing metrics exported by the [`metrics-exporter-tcp`](https://docs.rs/metrics-exporter-tcp/0.9.0/metrics_exporter_tcp/) crate
- [`ra-multiplex`](https://aur.archlinux.org/packages/ra-multiplex) - A client/server style program that can multiplex multiple `rust-analyzer` sessions together (think sharing the same r-a instance when you have two files in the same project open in two different vim editors)
- [`regex-cli`](https://aur.archlinux.org/packages/regex-cli) - A command line tool debugging, ad hoc benchmarking and generating regular expressions for Rust's regex crate
- [`to-html`](https://aur.archlinux.org/packages/to-html) - Convert terminal commands to formatted HTML
- [`typeshare`](https://aur.archlinux.org/packages/regex-cli) - Command line tool for generating files with typeshare

## Previously Maintained

- [`cargo-auditable`](https://archlinux.org/packages/community/x86_64/cargo-auditable/) - **Merged into community 🎉**
- [`cargo-insta`](https://archlinux.org/packages/extra/x86_64/cargo-insta/) - **Merged into extra 🎉**
- [`samply`](https://archlinux.org/packages/extra/x86_64/samply/) - **Merged into extra 🎉**
