## Next Steps (Phase 2 - Sound and Input)
- [ ] Add mouse information to Context2D
- [ ] Restructure project into workspace
  - [ ] Have set of examples
- [ ] Look for obsolete code and sections where performance can be optimised
- [ ] Use more ECS where possible to user can update their data in parallel more easily
- [ ] Introduce the use of multiple threads

- [ ] Move event loop into lib or engine
  - [x] Decided against doing this, managing input becomes very complex and is likely to hurt performance.
  - [x] Let user handle their inputs, it isn't so bad
  - [x] Nevermind it really isn't user friendly.
  - [ ] Write a thin wrapper for winit events in a context struct
  - [x] Pass this context struct to a closure supplied by a user

- [ ] 0.3.0 release blockers
  - [ ] Sound system
  - [ ] User input system

- [ ] Possible 0.4.0 release
  - [ ] GUI using framework eg iced egui

- [ ] Possible 0.5.0 release
  - [ ] Physics system including Collision2D suvat etc
  - [ ] Better coordinate system

- [ ] Possible 0.6.0 release
  - [ ] Tilemap (stores entities and a Tile2D type maybe)
  - [ ] Parallax backgrounds

- [ ] 1.0 release:
  - [ ] Could create Touhou or Stardew Valley
