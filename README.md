## Next Steps (Rewrite)
- [x] Remove TexturePool entirely.
- [x] Implement one time texture init for layer2D and atlas2D
- [ ] Cleanup entity2D
- [x] Reimplement TextureAtlas2D.
- [ ] Increase use of ECS paradigm.
- [ ] Increase simplicity of Layer2D to be used directly by a user.
- [ ] Increase overall simplicity and reduce abstraction.
- [x] Make complex high performance paths optional
- [x] Make layers and their atlases not allow dynamic addition and removal of textures
- [x] Fix incorrect buffer allocation / entity copying
- [x] Fix (shaders maybe?) to use correct texture atlas coordinates
  - store total width and height in layer atlas then use to calc exact tex position
- [x] Fix tex aspect ratio
- [x] Remove * 10.0 in vertex group when texture atlas complete
- [x] Fix issues where entity buffers aren't updated correctly
- [x] Fix entity positioning being incorrect
~~¬- [ ] Implement 2D transformations first~~
- [x] Fix broken buffers
- [x] Fix bug which causes different entities to use the texture of the first one in the buffer
- [ ] Finish rewrite and clean everything up
  - [ ] Very bad idea, find alternative to limit worldspace inaccuracies.
  - [ ] Remove unnecessary params
  - [ ] Remove unecessary functions
  - [ ] Add functions so user doesn't need to access queue and device
  - [ ] Any other issues
- [ ] Merge with main

- [ ] Further goals (0.2.0 release blockers)
  - [ ] Implement Transformation2D
  - [ ] Implement Camera2D (using 3D proj matrix)

- [ ] Possible 0.3.0 release
  - [ ] Sound system
  - [ ] User input system

- [ ] Possible 0.4.0 release
  - [ ] Physics system including Collision2D suvat etc

- [ ] Possible 0.5.0 release
  - [ ] GUI using framework eg iced egui

- [ ] 1.0 release when all basic components needed to make a game present and code is in good shape + performant
