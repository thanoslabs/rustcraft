# Architecture

Rustcraft is a performance-first voxel survival sandbox in Rust. The client uses a custom `wgpu` renderer; the simulation and world model must not depend on GPU or windowing APIs.

## Initial crates

- `core`: shared IDs, coordinates, and mathematical primitives.
- `world`: block states, chunk sections, and world access.
- `render`: GPU resources, chunk meshing, and visibility.
- `client`: window, input, presentation, and the application entrypoint.

The workspace starts intentionally small. Generation, simulation, physics, persistence, and protocol crates will be introduced with their first functional systems, rather than as empty scaffolding.

## Design constraints

- Chunk sections are 16 × 16 × 16 blocks; world coordinates are signed 64-bit.
- Block definitions are registry-backed; runtime block states must support properties such as orientation.
- Simulation runs at a fixed 20 TPS independently from the render loop.
- World generation, meshing, and storage I/O use bounded background queues; GPU uploads have per-frame budgets.
- Single-player runs an authoritative local simulation with explicit commands and events. Dedicated-server transport is deferred.
- Chunk generation is seeded and deterministic, with generator versions tracked for persistence.

See [performance](performance.md) and [roadmap](roadmap.md).
