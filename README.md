# Suika Clone

This is a clone of "that game" that I am developing as a study, using the [bevy engine](https://bevyengine.org/).
Its physics engine is [`bevy_xpbd`](https://github.com/Jondolf/bevy_xpbd).


[Bevy](https://bevyengine.org/) (v0.13.2) を使用した習作として作成中のあのゲームのクローンゲームです。
物理エンジンは[`bevy_xpbd`](https://github.com/Jondolf/bevy_xpbd) を使用しています。

## Build

`cargo-make` should be installed for build. See `suika_clone/Makefile.toml`.

[cargo-make](https://github.com/sagiegurari/cargo-make) が必要です。
`suika_clone/Makefile.toml` を参照してください。

`cd suika_clone` した後に、`makers build-web` / `makers run-web` はWeb版を開発ビルド／実行します。`makers release-web` はリリースビルドを実行します。
`makers release-win-gnu` すると、Windows版のリリースビルドを実行します。要mingw。

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

It will create a direcotry (`out-web`) at the repository's root for the http-server.

### Windows

I have only confirmed that it can be build on Ubuntu 22.04 on WSL2.
Read [this](https://bevy-cheatbook.github.io/setup/cross/linux-windows.html#first-time-setup-msvc) for setting up a cross-compile environment.

#### Development (`x86_64-pc-windows-gnu`)

CAUTION: On Wsl2, it cannot run exe properly ...

Workaround: `makers build-win-gnu`, and then copy `suika_clone.exe` from `target/x86_64-pc-windows-gnu/debug/` and `assets` directory to the same location that you can access from windows explorer.

---

In `{repository root}/suika_clone` directory:
```sh
$ makers run-win-gnu
```

#### Release (`x86_64-pc-windows-gnu`)

In `{repository root}/suika_clone` directory:
```sh
$ makers release-win-gnu
```

It will create a direcotry (`out-win`) at the repository's root for the http-server.


## License

- Sources etc. not under `external/` are under the MIT License.
- For anything under `external/`, please check there.

- external/ 下以外のソースコード等は MIT License です。
- external/ 下は外部のリソースです。そちらを確認してください。
