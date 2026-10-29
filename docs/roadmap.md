# Roadmap

1. **M0 Engine foundation**: Rust workspace, CI, profiling, `wgpu` bootstrap, camera, textured cube.
2. **M1 Voxel rendering**: chunk sections, block states, face culling, greedy meshing, streaming.
3. **M2 Creative gameplay**: collision, movement, raycasting, block interaction, hotbar.
4. **M3 Persistent worlds**: deterministic terrain, biomes, caves, trees, seeds, save/load.
5. **M4 Lighting**: sunlight, block lighting, water, fog, day/night.
6. **M5 Survival**: inventory, crafting, tools, health, hunger, mobs, combat.
7. **M6 Multiplayer**: dedicated server, replication, prediction, reconciliation.
8. **M7 Advanced systems**: farming, fluids, automation, extensions.

## Initial PR sequence

1. Rust workspace and architecture (foundation).
2. CI and benchmarks (after PR 1).
3. `wgpu` bootstrap (after PR 1).
4. Voxel core (after PR 1).
5. Camera and diagnostics (after PR 3).
6. Chunk meshing (after PRs 3 and 4).
7. Parallel terrain generation (after PR 4).
8. Chunk streaming (after PRs 5–7).

Every rendering feature should be evaluated against the baseline integrated-GPU target; do not trade predictable frame times for average FPS.
