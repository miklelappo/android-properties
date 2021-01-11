set -e
adb shell reboot

cargo check --target=aarch64-linux-android
cargo check --target=aarch64-linux-android
cargo check --target=aarch64-linux-android --features "bionic-deprecated"
cargo check --target=aarch64-linux-android --examples --features "bionic-deprecated"
cargo build --target=aarch64-linux-android --examples
cargo doc
cargo test

adb wait-for-device

echo "##########################"
adb push target/aarch64-linux-android/debug/examples/property_foreach /data/
adb shell chmod a+x /data/property_foreach
adb shell /data/property_foreach

echo "##########################"
adb push target/aarch64-linux-android/debug/examples/property_get /data/
adb shell chmod a+x /data/property_get
adb shell /data/property_get

echo "##########################"
adb push target/aarch64-linux-android/debug/examples/property_set /data/
adb shell chmod a+x /data/property_set
adb shell /data/property_set

echo "##########################"
adb push target/aarch64-linux-android/debug/examples/property_get /data/
adb shell chmod a+x /data/property_get
adb shell /data/property_get

echo "##########################"
adb push target/aarch64-linux-android/debug/examples/property_refresh /data/
adb shell chmod a+x /data/property_refresh
adb shell /data/property_refresh
