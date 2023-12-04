# Acid Rain

A game for [Bevy Jam #4](https://itch.io/jam/bevy-jam-4). Builds on top of
[bevy_game_template](https://github.com/NiklasEi/bevy_game_template).

Has builds for Windows, Linux, macOS, and Web (Wasm).

- run the native version with `cargo run`
- run the web build with `trunk serve`
  - requires [trunk](https://trunkrs.dev/): `cargo install --locked trunk`
  - requires `wasm32-unknown-unknown` target: `rustup target add
wasm32-unknown-unknown`
  - this will serve your app on `8080` and automatically rebuild + reload it
    after code changes
- push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger
  the release workflow
  - the release workflow automatically includes the `credits` directory in every
    build

### Updating the icons

1.  Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel
    png icon and run `create_icns.sh` (make sure to run the script inside the
    `build/macos` directory) - _Note: this requires a mac_
2.  Replace `build/windows/icon.ico` (used for windows executable and as favicon
    for the web-builds) - You can create an `.ico` file for windows by following
    these steps: 1. Open `macos/AppIcon.iconset/icon_256x256.png` in
    [Gimp](https://www.gimp.org/downloads/) 2. Select the `File > Export As` menu
    item. 3. Change the file extension to `.ico` (or click `Select File Type (By
Extension)` and select `Microsoft Windows Icon`) 4. Save as
    `build/windows/icon.ico`
3.  Replace `build/android/res/mipmap-mdpi/icon.png` with
    `macos/AppIcon.iconset/icon_256x256.png`, but rename it to `icon.png`

### Deploy web build to GitHub pages

1.  Trigger the `deploy-github-page` workflow
2.  After a few minutes the game is live at `http://username.github.io/repository`

To deploy newer versions, just run the `deploy-github-page` workflow again.

# License

This work is dual-licensed under MIT ([LICENSE-MIT](LICENSE-MIT) or
[http://opensource.org/licenses/MIT]) and Apache 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0). You can choose between one of them
if you use this work.
