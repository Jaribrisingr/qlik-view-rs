# Read Qlik Sense .qvd files 🛠

This is a fork of the python library to read qvd's in python but modified to be a binary executable.

Original repo: https://github.com/SBentley/qvd-utils


# Install
## The binary route (MacOS)
I didn't upload a binary to the repo, thus having rust installed is required for this.
```bash
# Rust can be installed with the following command
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
```

```bash
git clone <this repo>
cd qlik-view-rs
cargo build --release
mv ./target/release/vqvd /usr/local/bin # or another dir but then you have to add that dir in your /etc/paths which requires admin permissions
```
Restart your terminal and check if it works

# Usage
Optionally provide a limit (default is 20) of rows read & returned from the dataframe.
```
vqvd [OPTIONS] <PATH>

Arguments:
  <PATH>

Options:
  -r, --max-rows <MAX_ROWS>        [default: 20]
  -w, --cell-width <CELL_WIDTH>    [default: 30]
  -c, --max-columns <MAX_COLUMNS>  [default: 10]
  -m, --metadata
  -h, --help                       Print help
```

# Notes
Feel free to improve and extend 
