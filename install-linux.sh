#!/usr/bin/env sh

set -e

cargo build --release
chown root:root target/release/groupls
chmod u=rwx,go=rx target/release/groupls

mv target/release/groupls /usr/local/bin

echo 'Installed successfully'
