# Attempt at 2D tiling textures

Tiling works fine in 3D but fails in 2D

- `cargo run --example 3d_tiling` - ✅ works fine shows a 2x2 tiled texture
- `cargo run --example 2d_tiling` - ❌ camera changed to 2D and it no longer works. There is a blank white quad above from where the tiling should appear, the difference being the material.
- `cargo run --example 2d_tiling_addressmode` - ✅ works and uses addressmode applied as default. I don't know the benefits or trade-offs

Please fork and make a pull request if you know how to make this.

Solution for 2D is all I care about, if it breaks on 3D it's fine. It doesn't need to use shaders, if it doesn't have repeat count explicitly stated it should support both odd and even multiples of image tiles either automatically or through explicitly stating.
