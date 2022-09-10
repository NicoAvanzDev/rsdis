# 🚀 rsdis 🚀
Minimal rust redis clone

### Benchmark

I'm using memtier_benchmark (from Redis) to perform benchmark tests

`memtier_benchmark –s $SERVER_IP -t 8 -c 16 --test-time=30 --distinct-client-seed -d 256 --pipeline=30`

#### Machine specs

```
OS: Debian GNU/Linux 11 (bullseye) on Windows 10 x86_64
Kernel: 5.10.102.1-microsoft-standard-WSL2
CPU: AMD Ryzen 5 3600 (12) @ 3.6GHz
GPU: GeForce RTX 2060
Memory: 16GiB
```

#### Run command

`cargo run --release`

#### Results

```
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec
----------------------------------------------------------------------------------------------------------------------------
Sets       152278.91          ---          ---         2.29101         2.06300         5.63100         7.99900     45191.24
Gets      1522767.88    300734.45   1222033.43         2.29076         2.06300         5.63100         7.96700    135382.62
Waits           0.00          ---          ---             ---             ---             ---             ---          ---
Totals    1675046.79    300734.45   1222033.43         2.29079         2.06300         5.63100         7.96700    180573.86
```
