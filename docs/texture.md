 * https://www.youtube.com/watch?v=Lv6WEFGzqNQ
  * Texture 
```
texture_layer_index(BlockType::Grass, TextureMeta::Bottom)
```
https://discord.com/channels/691052431525675048/1003449239780720754/1003666359818932244

```
struct Vertex {
    [[builtin(vertex_index)]] vertex_index: u32;
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tile_map_index: i32;
};
```

 * https://github.com/bevyengine/bevy/issues/399
  * Tiled/repeating textures


 * https://github.com/bevyengine/bevy/blob/main/crates/bevy_render/src/mesh/shape/mod.rs
 * https://github.com/bevyengine/bevy/blob/main/examples/3d/skybox.rs

 * https://github.com/bevyengine/bevy/issues/399
 * https://stackoverflow.com/questions/72172260/how-do-i-assign-textures-to-2d-meshes-in-bevy
 * https://www.reddit.com/r/bevy/comments/u9kzch/tiling_a_texture/

 * https://discord.com/channels/691052431525675048/866787577687310356/1047528301113114706



 * https://stackoverflow.com/questions/74465897/why-does-this-not-render-a-cube-with-a-texture-on-all-sides-bevy
 * https://www.reddit.com/r/bevy/comments/vps2v7/render_textures_from_a_texture_atlas_on_a_cube/
 * https://github.com/bevyengine/bevy/blob/f8e0fc190a7e8e085e07e1fbab190093c29e8916/crates/bevy_render/src/mesh/shape/mod.rs#L62-L93
 * https://github.com/google/bevy_skybox_cubemap
 * https://www.youtube.com/watch?v=ARDnaA4xi_s
 * 
 * https://docs.rs/bevy_atmosphere/0.5.0/bevy_atmosphere/model/index.html
 * https://bevy-cheatbook.github.io/pitfalls/uv-coordinates.html
 * 
 * https://whoisryosuke.com/blog/2022/primitive-geometry-in-wgpu-and-rust/
 * 
 * https://docs.rs/bevy_pixel_camera/latest/bevy_pixel_camera/
 * https://www.rustadventure.dev/snake-with-bevy-ecs/a-new-coat-of-paint-using-texture-atlas-and-bevy-asset-loader
 * 
 * 
 * 


https://discord.com/channels/691052431525675048/691052431974465548/924405378115047435
```
// `ImageSample` is a bit complicated compared to the rest of the IR.
            //
            // First there are three variations depending wether the sample level is explicitly set,
            // if it's automatic or it it's bias:
            // `texture(image, coordinate)` - Automatic sample level
            // `texture(image, coordinate, bias)` - Bias sample level
            // `textureLod(image, coordinate, level)` - Zero or Exact sample level
```
https://discord.com/channels/691052431525675048/743663924229963868/830939254270984192

https://discord.com/channels/691052431525675048/692648638823923732/798328670053466123


 * https://discord.com/channels/691052431525675048/1011875529479176256/1012028542890033244
 ```
 let world_material = StandardMaterial {
        base_color_texture: Some(assets.load("world_color.png")),
        emissive: Color::WHITE,
        emissive_texture: Some(assets.load("world_emissive.png")),
        metallic: 1.0,
        perceptual_roughness: 1.0,
        // Metallic and roughness are stored in the blue and green channels of this texture respectively.
        metallic_roughness_texture: Some(assets.load("world_metallic_roughness.png")),
        ..default()
    };
 ```




