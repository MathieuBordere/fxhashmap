Produces different FxHashMaps on Linux and macOS.

```
mathieu@sophia:fxhashmap $ uname -a
Linux sophia 6.19.13-200.fc43.x86_64 #1 SMP PREEMPT_DYNAMIC Sat Apr 18 20:20:44 UTC 2026 x86_64 GNU/Linux
mathieu@sophia:fxhashmap $ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
mathieu@sophia:fxhashmap $ cargo run --release
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/fxiter`
ins=322 rm=161 final_len=25 cap=28
iter_order: [22, 44, 14, 69, 39, 50, 9, 20, 42, 12, 72, 67, 37, 59, 7, 29, 54, 65, 24, 35, 57, 5, 27, 52, 74]


mathieu@frida:fxhashmap $ uname -a
Darwin frida.local 25.3.0 Darwin Kernel Version 25.3.0: Wed Jan 28 20:56:35 PST 2026; root:xnu-12377.91.3~2/RELEASE_ARM64_T6030 arm64
mathieu@frida:fxhashmap $ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
mathieu@frida:fxhashmap $ cargo run --release
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/fxiter`
ins=322 rm=161 final_len=25 cap=56
iter_order: [22, 44, 69, 50, 72, 9, 12, 37, 59, 65, 24, 5, 27, 52, 74, 14, 39, 20, 42, 67, 7, 29, 54, 35, 57]
```
