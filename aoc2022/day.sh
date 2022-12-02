#!/bin/bash

set -eux

DAY="${1:-}"

if [[ -z "$DAY" ]]; then
    DAY=$(date +%d)
    echo "Using day $DAY by default..."
fi

cargo watch -x "test $DAY -- --nocapture" -x "run -- $DAY"