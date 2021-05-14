#!/usr/bin/env bash
docker run -v $PWD:/volume --rm -t clux/muslrust cargo build  --release