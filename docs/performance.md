# Performance targets

## Baseline hardware

Intel Iris Xe, AMD Radeon integrated graphics, and Apple M1. These are targets to validate, not measured claims.

- 1080p, default graphics, 12-chunk horizontal view distance: sustained 60 FPS or better during a repeatable exploration scenario.
- 20 TPS fixed-rate simulation under baseline single-player workload.
- Steady-state P99 frame time at or below 20 ms; measure chunk-streaming stalls separately.
- Initial planning budgets: main-thread CPU 8 ms, GPU 10 ms. These overlap and must not be summed as sequential frame costs.
- Memory should stabilize during extended exploration at a fixed view distance.

## Measurement policy

Record machine model, GPU, driver/OS, resolution, render distance, scene seed, run duration, build profile, and thermal conditions. Capture average FPS, P95/P99 frame time, CPU/GPU timing, memory, visible chunks, draw calls, chunk queue depths, and upload bytes.

Microbenchmarks will cover block access, generation, meshing, raycasting, and serialization. Standard scenes: flat, forest, mountains, caves, high-speed flight, dense builds, and survival simulation. CI benchmark deltas are advisory until hardware-specific variance is established. GPU performance gates require repeatable runs on dedicated physical hardware.
