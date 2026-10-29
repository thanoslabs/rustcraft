# Rustcraft

A performance-first Minecraft-inspired voxel survival sandbox built in Rust, with a custom `wgpu` renderer.

## Goals

- Smooth 60+ FPS at 1080p and 12-chunk horizontal view distance on Intel Iris Xe, AMD Radeon iGPU, and Apple M1 (engineering target; not yet measured).
- Efficient chunk storage, meshing, streaming, and deterministic world generation.
- Single-player first, with authoritative simulation independent of rendering for future multiplayer.

## Getting started

Install the pinned Rust toolchain from `.tool-versions` using mise/asdf or rustup, then run:

```sh
cargo run -p rustcraft-client
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

The client is currently a foundation placeholder; graphics and gameplay are forthcoming.

## Documentation

- [Architecture](docs/architecture.md)
- [Performance goals](docs/performance.md)
- [Roadmap](docs/roadmap.md)

## License

MIT. Rustcraft is an independent project and does not distribute Minecraft assets or branding.
