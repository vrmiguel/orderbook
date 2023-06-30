# orderbook

Example of an application with a backend in Rust (w/ `actix-web`) and a simplistic front-end in TypeScript with React.
Orders can be stored either in-memory, on a distributed sharded `HashMap` with low lock contention or in Redis, allowing for some level of persistence.

## Back-end

See the available endpoints, their accepted inputs and their responses in the backend's [README.md](backend/README.md).

A simple load test is available. On a six-core Ryzen 5 5500U, the results are:

```
Summary:
  Total:        21.4949 secs
  Slowest:      0.0392 secs
  Fastest:      0.0001 secs
  Average:      0.0043 secs
  Requests/sec: 69784.0314
  
  Total data:   70500000 bytes
  Size/request: 70 bytes

Latency distribution:
  10% in 0.0001 secs
  25% in 0.0002 secs
  50% in 0.0015 secs
  75% in 0.0039 secs
  90% in 0.0070 secs
  95% in 0.0094 secs
  99% in 0.0138 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0000 secs, 0.0001 secs, 0.0392 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0000 secs
  req write:    0.0000 secs, 0.0000 secs, 0.0032 secs
  resp wait:    0.0003 secs, 0.0000 secs, 0.0190 secs
  resp read:    0.0023 secs, 0.0000 secs, 0.0354 secs

Status code distribution:
  [200] 1000000 responses
```

The code used for the result above is in commit e608916fa92ca816ac52f0b173a00f19809f8f0b.

Using Redis, we got around 14k requests/sec.

## Front-end

See the corresponding [README](frontend/README.md);
