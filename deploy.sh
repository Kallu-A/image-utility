#!/bin/bash

# use this script to automatically deploy the new binary in the install (need a lot of dependencies to work)

# windows
echo windows deploy: 
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/image-utility.exe install/

# binary file
echo binary deploy: 
cargo build --release
cp target/release/image-utility install/


# debian
echo debian deploy:
cargo deb
cp target/debian/image-utility_0.1.0_amd64.deb install/

