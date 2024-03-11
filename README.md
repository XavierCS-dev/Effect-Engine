## Next Steps (Phase 1 - Renderer)
- [x] Remove EntityGroup2D - don't want it to take entity owernship
- [x] Find different way to pass refs to entities grouped by layer to render function
- [x] Render function - iterate through layers to draw, call methods to get buffers
- [x] Layers - Create and write to buffers when needed.
- [x] TextureAtlas2D - Complete atlas, to expand across width and height, and limit self to 8096x8096
- [x] Create descriptors for entity
- [x] Create Transform 2D maths stuff for entity (later)
- [x] Create buffers in shader
- [x] Add buffer layouts to pipeline
- [x] Add bind group layouts to pipeline (Mainly for entity)
- [x] Implement Entity2D::new()
- [x] Switch HashMap to BTreeMap where it makes sense to do so
- [ ] Add unit tests
- [x] Fix buffer locations
- [x] Modify Texture to be added to a layer upon creation
- [x] Refactor
  - [x] Reduce black box state
  - [ ] Increase unit test friendliness
  - [ ] Hide main loop from user, or at least, make it easier to use
  - [ ] Event System
- [ ] Provide a method to set a background image
- [ ] Fix camera stretching

- [x] Further goals (0.2.0 release blockers)
  - [x] Implement Transformation2D
  - [x] Implement Camera2D (using 3D proj matrix)

- [ ] Possible 0.3.0 release
  - [ ] Sound system
  - [ ] User input system

- [ ] Possible 0.4.0 release
  - [ ] Physics system including Collision2D suvat etc

- [ ] Possible 0.5.0 release
  - [ ] GUI using framework eg iced egui

- [ ] 1.0 release when all basic components needed to make a game present and code is in good shape + performant
