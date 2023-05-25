#!/bin/bash
cargo build --release --bin openstream &&
cd front && npm run ci && npm run build && cd ..