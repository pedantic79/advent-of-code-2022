#!/bin/bash

SOURCE=target/aoc/aoc-autobench/benches/aoc_benchmark.rs

if [ -e "$SOURCE" ]; then
    gsed -i 's/^criterion_group.*$/criterion_group!\(name = benches; config = Criterion::default\(\).sample_size\(10\); targets = aoc_benchmark\);/g' "$SOURCE"
    (cd target/aoc/aoc-autobench && cargo bench)
fi
