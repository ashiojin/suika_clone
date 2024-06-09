# Suika Clone

習作として作成中のあのゲームのクローンゲーム。
- ゲームエンジンは[Bevy](https://bevyengine.org/) を使用
- 物理エンジンは[bevy_xpbd](https://github.com/Jondolf/bevy_xpbd) を使用


## Build

開発環境は、WSL2上のubuntu 22.04 でのみ確認してます。
ビルド＆動作確認には、なんかいろいろ依存してたはずなのです・・・が、メモってない。いつかちゃんと修正します。
いろいろインストールが必要なものがあるはず・・・。

[cargo-make](https://github.com/sagiegurari/cargo-make) が必要です。
`cd suika_clone` した後に、`makers build` / `makers run` することでWindows版を開発ビルド／実行します。
`makers build-web` / `makers run-web` はWeb版を開発ビルド／実行します。
Makefileに追加してないですが、linux版もビルドできたはず・・・。

## License

- external/ 下以外のソースコード等は MIT License です。
- external/ 下は外部のリソースです。そちらを確認してください。

- Sources etc. not under `external/` are under the MIT License.
- For anything under `external/`, please check there.
