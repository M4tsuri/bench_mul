# Multiplication Instruction Benchmark

Usage

- `mul ecx` v.s. `mul rcx`: `./bench96bit mul`
- `imul edx, ecx` v.s. `imul rdx, rcx`: `./bench96bit mul`
- `rbx` steps 44 bytes each time, 5x mul64 + 1x mul32
  ```
   48 f7 23                mulq   (%rbx)
   48 f7 63 08             mulq   0x8(%rbx)
   48 f7 63 10             mulq   0x10(%rbx)
   48 f7 63 18             mulq   0x18(%rbx)
   48 f7 63 20             mulq   0x20(%rbx)
   f7 63 28                mull   0x28(%rbx)
  ```

  v.s.

  `rbx` steps 48 bytes each time, 6x mul64

  ```
  48 f7 23                mulq   (%rbx)
  48 f7 63 08             mulq   0x8(%rbx)
  48 f7 63 10             mulq   0x10(%rbx)
  48 f7 63 18             mulq   0x18(%rbx)
  48 f7 63 20             mulq   0x20(%rbx)
  48 f7 63 28             mulq   0x28(%rbx)
  ```

  : `./bench96bit mulcache`
- `r10` steps 44 bytes each time, 5x mul64 + 1x mul32
  ```
   49 0f af 12             imul   (%r10),%rdx
   49 0f af 52 08          imul   0x8(%r10),%rdx
   49 0f af 52 10          imul   0x10(%r10),%rdx
   49 0f af 52 18          imul   0x18(%r10),%rdx
   49 0f af 52 20          imul   0x20(%r10),%rdx
   41 0f af 52 28          imul   0x28(%r10),%edx
  ```

  v.s.

  `r10` steps 48 bytes each time, 6x mul64

  ```
   49 0f af 12             imul   (%r10),%rdx
   49 0f af 52 08          imul   0x8(%r10),%rdx
   49 0f af 52 10          imul   0x10(%r10),%rdx
   49 0f af 52 18          imul   0x18(%r10),%rdx
   49 0f af 52 20          imul   0x20(%r10),%rdx
   49 0f af 52 28          imul   0x28(%r10),%rdx
  ```

  : `./bench96bit imulcache`

