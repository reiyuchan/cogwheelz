#!/bin/bash

# Copyright (c) 2024 reiyuchan <iwakura.rei.tk@gmail.com>
# Released under the MIT license
# http://opensource.org/licenses/mit-license.php

DIR=$(cd $(dirname $0); pwd)
DEST_DIR=bin

PWD=$(pwd)

cd "$DIR"

if type cargo &>/dev/null
then
	cargo build --target x86_64-unknown-linux-gnu --target-dir "$DEST_DIR" --release
	cargo build --target x86_64-pc-windows-gnu --target-dir "$DEST_DIR" --release
else
	echo ERROR:cargo not found...
fi

cd "$PWD"
