set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
    @just --list

clean:
    cargo clean
    rm -rf flameglaph-*.svg packed.bin perf.data*

generate:
    cargo run --release -- create

time:
    cargo run --release -- time-generated
    cargo run --release -- time-fixed

flamegraph:
    cargo flamegraph --release --output flamegraph-generated.svg -- time-generated
    cargo flamegraph --release --output flamegraph-fixed.svg -- time-fixed
