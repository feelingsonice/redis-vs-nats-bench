# Benchmarking Redis pubsub vs NATS pubsub

This is a benchmark comparing the performance of Redis pubsub vs NATS pubsub.

Benchmarked on `redis:7.4` vs `nats:2.10` docker images using [`redis-rs`](https://github.com/redis-rs/redis-rs) and [`async-nats`](https://github.com/nats-io/nats.rs) crates.

The setup is a single subscriber that's subscribed to a single channel. On the publishing side, the publisher is bombarded with `N` messages at the same time. It does not take into account w/e the underlying client implementations are doing.

`[10000, 20000, 40000, 80000, 160000, 320000, 640000, 1280000]` messages were published.

## Result

Date: `2024-09-05`

Machine:

```
OS: macOS 15.0.0
Memory: 96 GiB
Architecture: aarch64
```

Output:

```
Running benches/bench_pubsubs.rs (target/release/deps/bench_pubsubs-3dbb3970230cf3ea)
Gnuplot not found, using plotters backend
PubSub/NATS/10000       time:   [14.781 ms 15.074 ms 15.435 ms]
Found 5 outliers among 100 measurements (5.00%)
3 (3.00%) high mild
2 (2.00%) high severe
PubSub/Redis/10000      time:   [24.060 ms 24.228 ms 24.403 ms]
Found 4 outliers among 100 measurements (4.00%)
4 (4.00%) high mild
PubSub/NATS/20000       time:   [26.769 ms 27.149 ms 27.531 ms]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild
PubSub/Redis/20000      time:   [46.730 ms 47.051 ms 47.403 ms]
Found 6 outliers among 100 measurements (6.00%)
2 (2.00%) high mild
4 (4.00%) high severe
Benchmarking PubSub/NATS/40000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.4s, or reduce sample count to 90.
PubSub/NATS/40000       time:   [53.435 ms 54.671 ms 55.926 ms]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild
Benchmarking PubSub/Redis/40000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.2s, or reduce sample count to 50.
PubSub/Redis/40000      time:   [91.770 ms 92.096 ms 92.432 ms]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high severe
Benchmarking PubSub/NATS/80000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 12.0s, or reduce sample count to 40.
PubSub/NATS/80000       time:   [119.13 ms 120.92 ms 122.66 ms]
Benchmarking PubSub/Redis/80000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 18.5s, or reduce sample count to 20.
PubSub/Redis/80000      time:   [185.04 ms 186.09 ms 187.23 ms]
Found 14 outliers among 100 measurements (14.00%)
8 (8.00%) high mild
6 (6.00%) high severe
Benchmarking PubSub/NATS/160000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 25.0s, or reduce sample count to 20.
PubSub/NATS/160000      time:   [247.65 ms 250.28 ms 252.80 ms]
Found 14 outliers among 100 measurements (14.00%)
2 (2.00%) low severe
6 (6.00%) low mild
5 (5.00%) high mild
1 (1.00%) high severe
Benchmarking PubSub/Redis/160000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 38.7s, or reduce sample count to 10.
PubSub/Redis/160000     time:   [369.94 ms 372.33 ms 375.07 ms]
Found 8 outliers among 100 measurements (8.00%)
6 (6.00%) high mild
2 (2.00%) high severe
Benchmarking PubSub/NATS/320000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 48.8s, or reduce sample count to 10.
PubSub/NATS/320000      time:   [478.45 ms 482.20 ms 485.74 ms]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) low mild
Benchmarking PubSub/Redis/320000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 72.3s, or reduce sample count to 10.
PubSub/Redis/320000     time:   [733.10 ms 736.23 ms 739.88 ms]
Found 7 outliers among 100 measurements (7.00%)
3 (3.00%) high mild
4 (4.00%) high severe
Benchmarking PubSub/NATS/640000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 92.3s, or reduce sample count to 10.
PubSub/NATS/640000      time:   [942.20 ms 950.49 ms 959.19 ms]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild
Benchmarking PubSub/Redis/640000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 149.0s, or reduce sample count to 10.
PubSub/Redis/640000     time:   [1.4456 s 1.4519 s 1.4582 s]
Found 5 outliers among 100 measurements (5.00%)
1 (1.00%) low severe
4 (4.00%) high mild
Benchmarking PubSub/NATS/1280000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 181.9s, or reduce sample count to 10.
PubSub/NATS/1280000     time:   [1.8186 s 1.8281 s 1.8381 s]
Found 2 outliers among 100 measurements (2.00%)
2 (2.00%) high mild
Benchmarking PubSub/Redis/1280000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 288.9s, or reduce sample count to 10.
PubSub/Redis/1280000    time:   [2.8791 s 2.8878 s 2.8967 s]
Found 8 outliers among 100 measurements (8.00%)
1 (1.00%) low mild
6 (6.00%) high mild
1 (1.00%) high severe
```
