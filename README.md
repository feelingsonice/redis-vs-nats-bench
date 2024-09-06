# Result

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
PubSub/NATS/10000       time:   [14.644 ms 14.877 ms 15.122 ms]
                   change: [+2.6226% +4.6496% +6.7751%] (p = 0.00 < 0.05)
                   Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild
PubSub/Redis/10000      time:   [23.700 ms 23.780 ms 23.864 ms]
                   change: [-0.6224% -0.1605% +0.3480%] (p = 0.53 > 0.05)
                   No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild
PubSub/NATS/20000       time:   [28.350 ms 29.199 ms 30.145 ms]
                   change: [+0.4194% +3.5884% +7.0454%] (p = 0.05 < 0.05)
                   Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
5 (5.00%) high mild
2 (2.00%) high severe
PubSub/Redis/20000      time:   [46.006 ms 46.195 ms 46.402 ms]
                   change: [-62.494% -36.190% -1.5830%] (p = 0.22 > 0.05)
                   No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
1 (1.00%) high mild
2 (2.00%) high severe
Benchmarking PubSub/NATS/40000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.4s, or reduce sample count to 90.
PubSub/NATS/40000       time:   [57.722 ms 59.219 ms 60.732 ms]
                   change: [-1.2574% +1.8824% +5.1286%] (p = 0.25 > 0.05)
                   No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild
Benchmarking PubSub/Redis/40000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 10.0s, or reduce sample count to 50.
PubSub/Redis/40000      time:   [95.212 ms 96.253 ms 97.346 ms]
                   change: [+3.5573% +4.7225% +5.9274%] (p = 0.00 < 0.05)
                   Performance has regressed.
Benchmarking PubSub/NATS/80000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 12.6s, or reduce sample count to 30.
PubSub/NATS/80000       time:   [118.96 ms 121.24 ms 123.52 ms]
                   change: [-2.8650% -0.3665% +2.1245%] (p = 0.77 > 0.05)
                   No change in performance detected.
Benchmarking PubSub/Redis/80000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 18.4s, or reduce sample count to 20.
PubSub/Redis/80000      time:   [183.41 ms 184.30 ms 185.26 ms]
                   change: [-2.4886% -1.8251% -1.2053%] (p = 0.00 < 0.05)
                   Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
4 (4.00%) high mild
3 (3.00%) high severe
Benchmarking PubSub/NATS/160000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 24.8s, or reduce sample count to 20.
PubSub/NATS/160000      time:   [251.08 ms 254.34 ms 257.60 ms]
Found 14 outliers among 100 measurements (14.00%)
1 (1.00%) low severe
6 (6.00%) low mild
5 (5.00%) high mild
2 (2.00%) high severe
Benchmarking PubSub/Redis/160000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 36.8s, or reduce sample count to 10.
PubSub/Redis/160000     time:   [371.26 ms 373.88 ms 376.81 ms]
Found 9 outliers among 100 measurements (9.00%)
4 (4.00%) high mild
5 (5.00%) high severe
```
