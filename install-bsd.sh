#!/usr/bin/env sh

set -e

cargo build --release
chown root:wheel target/release/groupls
chmod u=rwx,go=rx target/release/groupls

mv target/release/groupls /usr/local/bin

echo 'Installed successfully'
