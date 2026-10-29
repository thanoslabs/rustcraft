# ADR 0001: Custom voxel engine

**Status:** Accepted

Use Rust and a custom `wgpu` renderer rather than a general-purpose game engine. Own the world representation, chunk meshing, streaming, and simulation boundaries. This enables focused data-oriented optimization, portable GPU backends, and explicit performance measurements. The cost is additional implementation and maintenance responsibility.
