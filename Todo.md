# ToDo Items

## Game

- (ALWAYS) Refactoring!
  - [ ] Split `src/game_screen.rs` into player/ball/bottle/view related and others.
  - [ ] Re-design States
    - See: https://github.com/MiniaczQ/bevy-design-patterns/tree/main/patterns/
- [x] Remove Max Level Balls Combined.
- [x] Scoring:
  - [x] Combine Scores.
  - [x] Drop Scores.
- [x] Player position:
  - [x] y-position should be higher than all of balls.
  - [x] x-position should be limited x positon to the inside of the bottle.
- [x] Use random generator.
- [x] GameOver.
- [x] Reset game.
- [x] Embedded an external file (.ron or others) as settings
  for ball size, texture infos.
- [x] Sound.
  - [x] BGM.
  - [x] SE.
- [x] Title Screen.
  - Use embedded assets(title image)
- [x] Config Screen. (or Popup on title screen)
  - [x] List and Load a .ron file
- [x] Separate Asset Loading Logic ~~into Plugin~~ ~~or Use `bevy_asset_loader`~~
- [x] Refine config screen & title screen.
  - [x] Loading state needed to read 'list.ron' and selected game ron
  - [x] Use `bevy_egui_kbgp`
    - But `bevy_egui_kbgp` (or my code) has some strange focus movement on slider.
      And `A`, `D` on a slider are not work as to change value.
      So I don't use slider now.
- [x] Change some GameRon/Images/Audios to Embedded assets.
- [x] Change timing of spawing fake ball to after previous ball is touched to other
- [x] Player texture.
- [x] Create ~~PlayerBundle~~ /Asset.
- [x] Guide for dropping a ball
- More Player Actions.
  - [x] Holding a ball.
  - [x] Shaking the bottle.
- Extend .ron
  - [x] player settings
    - [x] Guide color
  - [x] bottle settings
  - [x] Ui on game screen
  - [x] background image
  - [x] popup image
  - [ ] messages
- [x] Pause in playing game
  - [x] Return to Title
  - [x] Restart
- [ ] Adjust game parameter/physics behavior
  - [ ] Spawned ball has too much impluse to bounce off the others.
    - [x] Add methods to adjust Friction and Restitution of balls/bottle.
    - [ ] Add methods to limit velocity that is too large for a ball
      - Idea: Air friction
    - [ ] Adjust parameters
  - [x] Player should be able to get more higher y-position.
  - [x] The radius of the puppetter's shape-cast should be equal to the radius of the next ball.
- [x] Gamepad supports
- [x] Move camera to a ball that is over the area at game over.
- [x] Fix issue: It can make "totem" by dropping balls in the same position.
- [x] Set config from program args( `ron_name` for web build )
- [x] Show application version on screen
- [x] Record/Show high score
- [ ] Ball Samples View
- [ ] Refine the game over popup to show how the player played.
  - show the numbers of balls combined
- [ ] Refine title screen
  - [x] Show credits (on config popup?). Then link to their websites using [Hyperlink](https://docs.rs/egui/latest/egui/widgets/struct.Hyperlink.html).
- [ ] Refine loading screen
- Effect
  - [x] Combine balls (at each levels)
  - [ ] ball protruded
- [x] Save config.
- [x] Separate game states to 
      application state (pre-load/title/config/loading/in-game) and
      in-game state (playing/pausing/gameover)
- [ ] New game mode: ex) Mode where the objective is to flood a lot of balls.



## Development Environments

- Build Environments
  - Release Build
    - [x] Wasm run environment
      - [x] Fix an issue caused by browser caches.
         - There are 2 issues
           - cash of .wasm
             - It will be solved by adding the build number to the wasm names.
           - cashes of assets/**
             - It will be solved by:
               - Changing `assets` directory to `assets_XXX` (XXX: assets version)
               - Or create [custom AssertReader](https://bevyengine.org/examples/Assets/custom-asset-reader/) that appends `?v=xxx` to any pathes.
      - [ ] Use [AssetMetaCheck::Never](https://github.com/bevyengine/bevy/pull/10623) to remove requests for `.meta` files of assets
    - [ ] Windows
    - [ ] Linux
    - [ ] macOS ?
  - Make (App Bundle)[https://github.com/burtonageo/cargo-bundle]
- [x] Github Actions
  - check: https://github.com/davidB/rust-cargo-make
- [ ] Refactoring Makefile.toml
- ...

