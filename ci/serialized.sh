#!/bin/sh

set -eux

# mkdir bin-output
# cargo run -p util --release -- generate-tests bin-output

du -h bin-output

# tarballing is fast; do it separately so `xz` gives us a real percentage
# (rather than piping).
tar cvf bin-output.tar bin-output

# Use `xz` standalone rather than `tar -J` so we get parallelism
# I also tested with zstd, but anything less than level 18 compresses worse
# than xz (and _significantly_ slower).
xz -v bin-output.tar
