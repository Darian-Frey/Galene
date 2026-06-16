# Build

## Supported platforms

- Linux (primary development target).
- Windows and macOS are shipping targets (Phase 5); not yet validated.

## Dependencies

Current scaffold (Phase 0):

- **Rust 1.95+** (stable), with `cargo`. No other toolchain required yet.
- Third-party crates: `serde`, `ron` (fetched by cargo on first build).

Deferred dependencies, added at their phase (see [DECISIONS.md](DECISIONS.md) D-005):

| Dependency | Crate | Phase / trigger |
|---|---|---|
| GPU rendering | `wgpu` | Phase 0, once render-doc §12 questions are resolved |
| Desktop shell | `tauri` + Node toolchain | Phase 0 app scaffold |
| Session storage | `rusqlite` | Phase 2 |
| Ambient synthesis | `nyx` (external) | availability TBC |

When the Tauri shell lands, this file gains the Node/`tauri-cli` setup and the
per-platform packaging commands.

## Build commands

```bash
cargo build                  # debug build, whole workspace
cargo build --release        # optimised build
cargo test                   # run all tests
cargo clippy --workspace     # lints
cargo run -p flowstate-app   # run the (placeholder) app binary
```

## Troubleshooting

- Nothing platform-specific yet. The workspace builds offline once the `serde`
  and `ron` crates are cached.
