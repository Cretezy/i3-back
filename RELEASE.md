# How to release

- Bump version in `Cargo.toml` and `cargo build`
- Commit new version
- Create tag for version
- Push commit and tag
- Run [`cargo aur`](https://crates.io/crates/cargo-aur)
- Create GitHub release
- Add `target/release/i3-back` and `i3-back-*.tar.gz` to GitHub release
- Go to repository for [i3-back-bin (AUR)](https://aur.archlinux.org/packages/i3-back-bin)
- Copy `PKGBUILD`
- Generate `.SRCINFO`: `makepkg --printsrcinfo > .SRCINFO`
- Commit and push AUR repository
