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
- [`cargo-insta`](https://aur.archlinux.org/packages/cargo-insta) - Cargo plugin for the [`insta`](https://github.com/mitsuhiko/insta) snapshot testing library
- [`inlyne`](https::/aur.archlinux.org/packages/inlyne) - A GPU powered, browserless, markdown viewer
- [`to-html`](https://github.com/Aloso/to-html) - Convert terminal commands to formatted HTML

## Previously Maintained

- [`cargo-auditable`](https://archlinux.org/packages/community/x86_64/cargo-auditable/) - **Merged into community 🎉**
