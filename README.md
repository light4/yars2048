# Yars 2048 (Yet Another Rust 2048)

一个几乎只用 Rust 开发的 Android 2048

## 下载编译打包

```bash
git clone https://github.com/light4/yars2048
cd yars2048

# 注意修改 Android SDK 和 NDK 路径
env RUST_BACKTRACE=1 ANDROID_SDK_ROOT=/home/light4/Android/Sdk/ ANDROID_NDK_ROOT=/home/light4/Android/Sdk/ndk/24.0.8215888/ cargo apk build --release
```

依赖于修改版的 bevy

* [bevy#fix_android](https://github.com/light4/bevy/commits/fix_android)
* [bevy_easings#0.7.0-dev](https://github.com/light4/bevy_easings/commits/0.7.0-dev)
