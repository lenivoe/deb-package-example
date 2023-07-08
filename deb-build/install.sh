#!/bin/bash
set -e

mkdir -p "$DESTDIR/usr/bin/"

cp build-rust/release/duck "$DESTDIR/usr/bin/hydrogen-rugby-july-floor"
