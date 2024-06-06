# Suika Clone

習作として作成中のあのゲームのクローンゲーム。
- ゲームエンジンは[Bevy](https://bevyengine.org/) を使用
- 物理エンジンは[bevy_xpbd](https://github.com/Jondolf/bevy_xpbd) を使用


## Build

開発環境は、WSL2上のubuntu 22.04 でのみ確認してます。
ビルド＆動作確認には、なんかいろいろ依存してたはずなのです・・・が、メモってない。いつかちゃんと修正します。
いろいろインストールが必要なものがあるはず・・・。

[cargo-make](https://github.com/sagiegurari/cargo-make) が必要です。
`makers build` / `makers run` はWindows版を開発ビルド／実行します。
`makers build-web` / `makers run-web` はWeb版を開発ビルド／実行します。
Makefileに追加してないですが、linux版もビルドできたはず・・・。


## ToDo Items (Development)

- (ALWAYS) Refactoring!
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
    But `bevy_egui_kbgp` (or my code) has some strange focus movement on slider.
    And `A`, `D` on a slider are not work as to change value.
    So I don't use slider now.
- [x] Change some GameRon/Images/Audios to Embedded assets.
- [x] Change timing of spawing fake ball to after previous ball is touched to other
- [x] Player texture.
- [x] Create ~~PlayerBundle~~ /Asset.
- [x] Guide for dropping a ball
- More Player Actions.
  - [x] Holding a ball.
  - [ ] Shaking the bottle.
- [ ] Extend .ron
  - [ ] player settings
    - [ ] Guide color
  - [ ] bottle settings
  - [ ] background image
  - [ ] popup/messages
- [x] Pause in playing game
  - [x] Return to Title
  - [ ] Restart
- [ ] Adjust game parameter/physics behavior
  - [ ] Spawned ball has too much impluse to bounce off the others.
  - [ ] Player should be able to get more higher y-position.
- [ ] Set config from program args.
- [ ] Save config.
- [ ] New game mode: ex) Mode where the objective is to flood a lot of balls.
- [x] Separate game states to 
      application state (pre-load/title/config/loading/in-game) and
      in-game state (playing/pausing/gameover)
- [ ] Re-design States
      See: https://github.com/MiniaczQ/bevy-design-patterns/tree/main/patterns/



## ToDo Items (Development Environments)

- [ ] Build Environments
  - [ ] Release Build
    - [ ] Wasm run environment
- [ ] Github Actions
- ...
