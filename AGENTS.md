# Rustcraft agent guidelines

## Mission

Build a performance-first voxel survival sandbox in Rust with a custom wgpu renderer. Single-player first, with simulation independent of rendering for future authoritative multiplayer.

## Repository conventions

- Follow pinned Rust version in `.tool-versions` (mise/asdf compatible).
- Workspace crates: `core`, `world`, `render`, and `apps/client`. Add new crates only with actual functionality.
- No direct commits to main; use focused branches and draft PRs for work in progress.
- SemVer release tags are bare X.Y.Z; preserve existing nebula-md documentation workflow.
- Keep core and simulation crates independent from graphics/windowing dependencies.
- Prefer safe Rust and explicit ownership; workspace forbids unsafe code by default.

## Validation

Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` before proposing a merge.

Performance-sensitive changes must include a reproducible benchmark or profile evidence, workload description, and memory/allocation considerations. Avoid unbounded queues, blocking I/O on the render thread, and unnecessary allocations in hot paths. Do not claim FPS targets are achieved without real hardware measurements.

See `docs/architecture.md`, `docs/performance.md`, and `docs/roadmap.md`.
