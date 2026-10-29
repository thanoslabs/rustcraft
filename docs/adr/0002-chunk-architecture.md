# ADR 0002: Chunk sections

**Status:** Accepted as initial design

Use 16³ independently meshed chunk sections and signed 64-bit world coordinates. Begin with contiguous dense block arrays, measure, then introduce palette compression where beneficial. Use visible-face culling first and benchmark greedy meshing. Neighbor changes must invalidate relevant section boundaries. Camera-relative coordinates avoid large-world GPU precision issues.
