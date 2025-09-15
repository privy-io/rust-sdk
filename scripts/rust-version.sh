awk -F'"' '/channel/ {print $2}' < rust-toolchain.toml
