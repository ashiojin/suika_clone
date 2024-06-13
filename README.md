# Suika Clone

This is a clon of "that game" that I am developing as a study, using the [bevy engine](https://bevyengine.org/).
Its physics engine is [`bevy_xpbd`](https://github.com/Jondolf/bevy_xpbd).


[Bevy](https://bevyengine.org/) (v0.13.2) を使用した習作として作成中のあのゲームのクローンゲームです。
物理エンジンは[`bevy_xpbd`](https://github.com/Jondolf/bevy_xpbd) を使用しています。

## Build

`cargo-make` should be installed for build.

[cargo-make](https://github.com/sagiegurari/cargo-make) が必要です。
`cd suika_clone` した後に、`makers build` / `makers run` することでWindows版を開発ビルド／実行します。
`makers build-web` / `makers run-web` はWeb版を開発ビルド／実行します。
Makefileに追加してないですが、linux版もビルドできたはず・・・。

### Web(`wasm32-unknown-unknown`)

#### Development

In `{repository root}/suika_clone` directory:
```sh
$ makers run-web
```

#### Release

In `{repository root}/suika_clone` directory:
```sh
$ makers relase-web
```

It will create a direcotry (`out`) at the repository's root for the http-server.

### Windows(`x86_64-pc-windows-msvc`)

I have only confirmed that it can be build on Ubuntu 22.04 on WSL2.
Read [this](https://bevy-cheatbook.github.io/setup/cross/linux-windows.html#first-time-setup-msvc) for setting up a cross-compile environment.

#### Development

In `{repository root}/suika_clone` directory:
```sh
$ makers run
```


## License

- Sources etc. not under `external/` are under the MIT License.
- For anything under `external/`, please check there.

- external/ 下以外のソースコード等は MIT License です。
- external/ 下は外部のリソースです。そちらを確認してください。
