# Notes

Just pro-cons on tech choices and little things I don't want to forget whil implementing `renderling`.

# rust-gpu

## pros

* sharing code on CPU and GPU
* it's Rust
  - using cargo and Rust module system
  - expressions!

## cons / limititions

* can't use enums (but you can't in glsl or hlsl or msl or wgsl either)
* struct layout size/alignment errors can be really tricky
* rust code must be no-std
* don't use `while let` or `while` loops
* for loops are hit or miss, sometimes they work and sometimes they don't
  - see [this rust-gpu issue](https://github.com/EmbarkStudios/rust-gpu/issues/739)
  - see [conversation with edyyb on discord](https://discord.com/channels/750717012564770887/750717499737243679/threads/1092283362217046066)
* meh, but no support for dynamically sized arrays (how would that work in no-std?)
  - see [conversation on discord](https://discord.com/channels/750717012564770887/750717499737243679/1091813590400516106)

# wgpu

## pros

* works on all platforms with the same API
* much more configurable than OpenGL
* much better error messages than OpenGL
* much less verbose than Vulkan
* the team is very responsive

## cons

* no support for arrays of textures on web, yet
* not yet 1.0 (on by default in chrome beta)
* what happens if WebGPU the standard fails? (everyone doubts it will)
* atomics are not supported in the Naga SPIRV frontend, which limits the capabilities of compute
  - see [the related Naga issue](https://github.com/gfx-rs/naga/issues/2301)

# more things to figure out

* bindless - wth exactly is it

# tips and gotchas

* `location[...] is provided by the previous stage output but is not consumed as input by this stage.`
  - rust-gpu has optimized away the shader input, you must use the input parameter in your downstream shader
  - sometimes the optimization is pretty agressive, so you really gotta _use_ the input

# links

- [Forward+ shading (as opposed to deferred)](https://takahiroharada.files.wordpress.com/2015/04/forward_plus.pdf)
  **tl;dr**
  In a compute shader before the vertex pass:
  * break up the frame into tiles
  * for each tile compute which lights contribute to the pixels in the tile
  * during shading, iterate over the lights for each pixel according to its tile