# Result

Machine:

```
OS: macOS 15.0.0
Memory: 96 GiB
Architecture: aarch64
```

Output:

```
SimplexPubsub/NATS/10000
                        time:   [13.495 ms 13.621 ms 13.746 ms]
SimplexPubsub/Redis/10000
                        time:   [23.390 ms 23.602 ms 23.849 ms]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
SimplexPubsub/NATS/20000
                        time:   [26.315 ms 26.764 ms 27.206 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
SimplexPubsub/Redis/20000
                        time:   [45.852 ms 46.061 ms 46.286 ms]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Benchmarking SimplexPubsub/NATS/40000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.2s, or reduce sample count to 90.
SimplexPubsub/NATS/40000
                        time:   [51.308 ms 52.548 ms 53.775 ms]
Benchmarking SimplexPubsub/Redis/40000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.1s, or reduce sample count to 50.
SimplexPubsub/Redis/40000
                        time:   [91.830 ms 92.679 ms 93.617 ms]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
Benchmarking SimplexPubsub/NATS/80000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 11.7s, or reduce sample count to 40.
SimplexPubsub/NATS/80000
                        time:   [111.49 ms 113.51 ms 115.49 ms]
Benchmarking SimplexPubsub/Redis/80000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 18.1s, or reduce sample count to 20.
SimplexPubsub/Redis/80000
                        time:   [180.07 ms 180.63 ms 181.25 ms]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
```
