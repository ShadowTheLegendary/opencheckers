# OpenCheckers #
Simple checkers implementation, faithful to the rules determined by [wikipedia](https://en.wikipedia.org/wiki/English_draughts).

## Building from source ##
You will need cargo, the rust package manager. I recommend installing it via [rustup](https://rust-lang.org/tools/install/).
```bash
git clone https://github.com/ShadowTheLegendary/opencheckers.git
cd opencheckers
cargo build --release
./target/release/checkers
```

## macOS
If you see a warning about the app being unverified, run this in Terminal:
```bash
xattr -d com.apple.quarantine opencheckers-macos
```
Then double-click the binary as normal.
