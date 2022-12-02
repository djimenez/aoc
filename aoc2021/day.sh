#!/bin/bash

set -eux

cargo watch -x "test day$1 -- --nocapture --test-threads=1" -x "run -- day$1"
