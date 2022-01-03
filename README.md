Bitmap Plasma with Rust
=======================

Android Code Sample / BitmapPlasma の C code を Rust code に置き換えたサンプルになります。

以下の手順で実装しています。

* Android Studio Arctic Fox 2020.3.1 の `Android Code Sample / BitmapPlasma` を取り込む
* update AGP 7.0.4/gradle 7.0.2
* add [Rust Android Gradle Plugin](https://github.com/mozilla/rust-android-gradle)
* android ndk 用の ffi は [複数登録](https://crates.io/search?q=ndk) されているが使用せずに、[最低限の実装のモジュール](rust/src/android_bitmap.rs) と [build.rs](rust/build.rs) を追加
* plasma.c を Rust へ移植

`rust-android-gradle` を使ったが、[Native API](https://developer.android.com/ndk/guides/stable_apis) の取り込み方法が分からず、buld.rs で適当に補うようにしたが、そのうち改善されるだろうか。。。

### 表示結果

| C code | Rust code |
|--------|-----------|
|![sc-c-20220103-01](sc-c-20220103-01.gif) | ![sc-c-20220103-02](sc-c-20220103-02.gif) |

C版は 60 frame/s 程度は出ているが、Rust 版は 30 frame/s 程度になっている。。。Rust の記載に熟れていないからだろうか。。。

参考
===

* https://github.com/android/ndk-samples/tree/main/bitmap-plasma
* https://github.com/mozilla/rust-android-gradle
* https://www.rust-lang.org
* https://crates.io
* https://developer.android.com/ndk
* https://developer.android.com/ndk/guides/stable_apis
