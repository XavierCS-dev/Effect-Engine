## Next Steps (Rewrite)
- [x] Remove TexturePool entirely.
- [ ] Implement one time texture init for layer2D and atlas2D
- [ ] Cleanup entity2D
- [x] Reimplement TextureAtlas2D.
- [ ] Increase use of ECS paradigm.
- [ ] Increase simplicity of Layer2D to be used directly by a user.
- [ ] Increase overall simplicity and reduce abstraction.
- [ ] Make complex high performance paths optional
- [ ] Make layers and their atlases not allow dynamic addition and removal of textures
- [x] Fix incorrect buffer allocation / entity copying
- [ ] Fix (shaders maybe?) to use correct texture atlas coordinates
  - store total width and height in layer atlas then use to calc exact tex position
- [x] Fix tex aspect ratio
- [ ] Remove * 10.0 in vertex group when texture atlas complete
- [ ] Fix issues where entity buffers aren't updated correctly
- [x] Fix entity positioning being incorrect
~~¬- [ ] Implement 2D transformations first~~
- [x] Fix broken buffers
- [ ] Finish rewrite and clean everything up
  - [ ] Switch from f32 to f64 if a good idea
  - [ ] Remove unnecessary params
  - [ ] Remove unecessary functions
  - [ ] Add functions so user doesn't need to access queue and device
  - [ ] Any other issues
- [ ] Merge with main

- [ ] Further goals (0.2.0 release blockers)
  - [ ] Implement Transformation2D
  - [ ] Implement Camera2D (using 3D proj matrix)

- [ ] Possible 0,3.0 release
  - [ ] Sound system
  - [ ] User input system

- [ ] Possible 0.4.0 release
  - [ ] Physics system including Collision2D suvat etc

- [ ] Possible 0.5.0 release
  - [ ] GUI using framework eg iced egui

- [ ] 1.0 release when all basic components needed to make a game present and code is in good shape + performant
