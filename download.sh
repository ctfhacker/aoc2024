#!/usr/bin/env bash

if [ -z ${SESSION+x} ]; then
  echo "SESSION env var is not set."
  exit 1
fi

# Create the repo is it doesn't already exist
cargo init "day$1" || true

# Download the input for this day
curl --cookie session="$SESSION" "https://adventofcode.com/2024/day/$1/input" > "day$1/input"
