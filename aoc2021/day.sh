#!/bin/bash

set -eux

cargo watch -x "test day$1 -- --nocapture" -x "run -- day$1"
