# Effect Engine 2

## Next Steps (Phase 2 - Sound and Input)
- [ ] Introduce the use of multiple threads
  - [ ] Do this on a PER system basis, ie system functions are muktithreaded themselves.
  - [ ] Add parallel operations to app ie settings entities for multiple layers at a time.
        to take burden from user.
  - [ ] Add worker task system for internal loop, only use when number of tasks becomes high
- [ ] Re expose device and queue to allow for massive parallel operations should the user want it
  - [ ] Create own parallel functions instead
- [ ] Controller support

- [ ] 0.3.0 alpha blockers
  - [x] Sound system
    - [x] Allow tracks to be replayed..somehow
    - [x] Separate system for spacial audio
  - [x] User input system
  - [ ] Renderer optimisation
  - [ ] Project restructure
    - [x] Restructure into Cargo workspace
    - [ ] Increase abstractions
    - [ ] Examples
    - [ ] Internalise WebLayer2D
      - Provide option for parallel updates
    - [ ] Remove unnecessary dependencies
      - [ ] Possibly use features to minimise deps
  - [ ] Replace texture atlas with texture arrays
  - [ ] Generate 2D sprite vertices on the fly in shader
  - [ ] Simplify Texture2D to component struct used to by user to easily reinitialise a texture
    - Technical aspects will be covered by TextureData
    - Size will be stored in the layer, as it will be uniform with all other textures in the layer.
      - [ ] Remove size, index fields from Texture2D
    - [ ] Update Texture2D to implement Copy
  - [ ] Add pixel art option to TextureData builder, so it is done on a per texture basis

- [ ] Possible 0.4.0 alpha
  - [ ] Debug GUI using egui
  - [ ] Font rendering and basic GUI framework
    - [ ] WGPU renderer for GUI

- [ ] Possible 0.5.0 alpha
  - [ ] Physics system including Collision2D suvat etc
  - [ ] Improved coordinate system

- [ ] Possible 0.6.0 alpha
  - [ ] Parallax backgrounds
  - [ ] 2D Shadows and lighting, fog etc
  - [ ] Normal maps
  - [ ] Custom shaders and pipeline, mainly for lighting and 3D tech

This may or may not need to come before shadows, lighting and fog etc
- [ ] Possible 0.7.0 alpha
  - [ ] 2D particle system

- [ ] Possible 0.8.0 alpha
  - [ ] Tilemap (stores entities and a Tile2D type maybe)

- [ ] 1.0 release:
  - [ ] Could create Touhou or Stardew Valley

- [ ] 2.0 release (Effect Engine 3 rebrand)
  - [ ] Full 3D support

