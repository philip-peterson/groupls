#!/usr/bin/env sh

set -e

cargo build --release

sudo chown root:wheel target/release/groupls
sudo chmod u=rwx,go=rx target/release/groupls
sudo mv target/release/groupls /usr/local/bin

echo 'Installed successfully'
