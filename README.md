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
- [ ] Separate Asset Loading Logic into Plugin ~~or Use `bevy_asset_loader`~~
- [ ] Change some GameRon/Images/Audios to Embedded assets.
- [ ] Refine config screen & title screen.
  - [ ] Loading state needed to read 'list.ron' and selected game ron
  - [ ] Use `bevy_egui_kbgp`
- [x] Change timing of spawing fake ball to after previous ball is touched to other
- [ ] Create PlayerBundle.
- [ ] Player texture.
- [ ] Player Actions.
  - [ ] Holding a ball.
  - [ ] Shaking the bottle.
- [ ] Extend .ron
  - [ ] player settings
  - [ ] bottle settings
  - [ ] background image
  - [ ] popup/messages
- [ ] Pause in playing game
  - [ ] Return to Title
  - [ ] Restart
- [ ] Set config from program args.
- [ ] Save config.
- [ ] New game mode: ex) Mode where the objective is to flood a lot of balls.
- [ ] Separate game states to 
      application state (pre-load/title/config/loading/in-game) and
      in-game state (playing/pausing/gameover)


## ToDo Items (Development Environments)

- [ ] Build Environments
  - [ ] Release Build
- [ ] Github Actions
- ...
