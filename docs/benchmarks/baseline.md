# ezpdf Benchmark Baseline

**Date:** 2026-03-12
**Machine:** Apple M3 MacBook Pro (darwin 25.3.0)
**Profile:** release (`cargo bench`)
**Criterion:** warm-up 3s, measurement 5s, 100 samples

## Results

| Benchmark | Mean | Lower CI | Upper CI |
|-----------|------|----------|----------|
| merge 5×10-page PDFs (parallel) | 9.76 ms | 9.43 ms | 10.11 ms |
| split_each 50-page PDF | 333 ms | 327 ms | 340 ms |
| remove half of 50-page PDF | 8.08 ms | 7.75 ms | 8.44 ms |
| rotate all pages of 50-page PDF | 7.96 ms | 7.73 ms | 8.23 ms |

## Notes

- Merge uses `rayon::par_iter` for parallel document loading — speedup is most
  pronounced with 5+ large inputs on multi-core machines.
- `split_each` is I/O-bound (writes 50 separate files); the 333 ms is dominated
  by file creation overhead.
- All operations are lossless: no re-encoding, only structural object manipulation.
