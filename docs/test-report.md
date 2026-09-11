# RTIC2 Multi-Head App Composition Test Report

- Run: 2026-09-11 01:53:09 +02:00
- Target: `thumbv7em-none-eabihf`
- Manifest: `C:\ws\rtic-code-skeleton\Cargo.toml`

## PASS - default F401 system
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: error finalizing incremental compilation session directory `\\?\C:\ws\rtic-code-skeleton\target\de
bug\incremental\build_script_build-2rccjof5f5l7j\s-hm6duon4b5-0mv3q71-working`: The process cannot access the file beca
use it is being used by another process. (os error 32)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: error .... (os error 32):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 

warning: `rtic-code-skeleton` (build script) generated 1 warning
   Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
```

## PASS - default F405 system
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
warning: function `f405_software_task` is never used
   --> src\app_body_skeleton.rs:129:14
    |
129 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:41:9
   |
41 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
54 |     fn init(_cx: init::Context) -> (Shared, Local) {
   |                                     ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
```

## PASS - default F411 system
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
```

## PASS - two systems sharing F401 rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f401,system-nucleo-f401re`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-895f9c08d0135d20\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/config.toml
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/config.toml
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/config.toml
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411
  cargo:rerun-if-changed=src/chips/f401/config.toml

  --- stderr

  thread 'main' (16360) panicked at build.rs:183:5:
  select at most one system feature
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## PASS - systems using different chips rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f401,system-default-f405`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4xx-hal v0.23.0
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling stm32f4xx-hal v0.23.0:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
error: failed to run custom build command for `stm32f4xx-hal v0.23.0`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\stm32f4xx-hal-f4106b5613392783\build-s
cript-build` (exit code: 101)
  --- stderr

  thread 'main' (6152) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4xx-hal-0.
23.0\build.rs:31:39:
  Multiple stm32xx Cargo features enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-78ed8fb3730e22fe\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/config.toml
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/config.toml
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/config.toml
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411

  --- stderr

  thread 'main' (24188) panicked at build.rs:271:5:
  assertion `left == right` failed: exactly one chip feature must be selected
    left: 2
   right: 1
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## PASS - Nucleo system selects F401 and its hardware
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-nucleo-f401re`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: error finalizing incremental compilation session directory `\\?\C:\ws\rtic-code-skeleton\target\de
bug\incremental\build_script_build-3prlcji6aej3v\s-hm6duw8t5w-1jprisn-working`: The process cannot access the file beca
use it is being used by another process. (os error 32)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: error .... (os error 32):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 

warning: `rtic-code-skeleton` (build script) generated 1 warning
   Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
```

## PASS - Nucleo system accepts explicit matching chip
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-nucleo-f401re,f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: error finalizing incremental compilation session directory `\\?\C:\ws\rtic-code-skeleton\target\de
bug\incremental\build_script_build-3prlcji6aej3v\s-hm6duw8t5w-1jprisn-working`: The process cannot access the file beca
use it is being used by another process. (os error 32)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: error .... (os error 32):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 

warning: `rtic-code-skeleton` (build script) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
```

## PASS - Nucleo system rejects incompatible chip
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-nucleo-f401re,f405`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4xx-hal v0.23.0
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling stm32f4xx-hal v0.23.0:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
error: failed to run custom build command for `stm32f4xx-hal v0.23.0`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\stm32f4xx-hal-f4106b5613392783\build-s
cript-build` (exit code: 101)
  --- stderr

  thread 'main' (29588) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4xx-hal-0
.23.0\build.rs:31:39:
  Multiple stm32xx Cargo features enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-3396cb9efcb9063d\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/config.toml
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/config.toml
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/config.toml
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411

  --- stderr

  thread 'main' (22108) panicked at build.rs:271:5:
  assertion `left == right` failed: exactly one chip feature must be selected
    left: 2
   right: 1
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## PASS - Nucleo system rejects generated-body fixture
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-nucleo-f401re,generated-body`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe : warning: error finalizing incremental compilation session directory `\\?\C:\ws\rtic-code-skeleton\target\de
bug\incremental\build_script_build-3tqm5rwg3qr10\s-hm6duyysro-1l5p73i-working`: The process cannot access the file beca
use it is being used by another process. (os error 32)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: error .... (os error 32):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 

warning: `rtic-code-skeleton` (build script) generated 1 warning
   Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-842317a2a0909eef\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/config.toml
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/config.toml
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/config.toml
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411
  cargo:rerun-if-changed=src/chips/f401/config.toml
  cargo:rerun-if-changed=src/systems/nucleo_f401re/config.toml

  --- stderr

  thread 'main' (27944) panicked at build.rs:201:9:
  system features cannot be combined with the generated-body fixture
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## PASS - Nucleo system rejects invalid interrupt
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-nucleo-f401re,invalid-interrupt`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error[E0599]: no variant or associated item named `UART8` found for enum `Interrupt` in the current scope
  --> C:\ws\rtic-code-skeleton\target\thumbv7em-none-eabihf\debug\build\rtic-code-skeleton-b74ed954563935d7\out/generat
ed_app_f401.rs:45:20
   |
 3 | / #[rtic::app(
 4 | |     device = stm32f4xx_hal::pac,
 5 | |     dispatchers = [TIM2]
 6 | | )]
...  |
45 | |     #[task(binds = UART8)]
   | |                   -^^^^^ variant or associated item not found in `Interrupt`
   | |___________________|
   |

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rtic-code-skeleton` (bin "rtic-code-skeleton") due to 1 previous error
```

## PASS - f401 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
```

## PASS - f405 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
warning: function `f405_software_task` is never used
   --> src\app_body_skeleton.rs:129:14
    |
129 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:41:9
   |
41 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
54 |     fn init(_cx: init::Context) -> (Shared, Local) {
   |                                     ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
```

## PASS - f411 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
```

## PASS - f401 generated body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,generated-body`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
```

## PASS - f405 generated body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405,generated-body`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
```

## PASS - syn reduction removes f405-only UART4 for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.14s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405-only UART4 retained
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:129:14
    |
129 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:41:9
   |
41 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
54 |     fn init(_cx: init::Context) -> (Shared, Local) {
   |                                     ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
```

## PASS - f405-only UART4 removed for f401
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.14s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405-only UART4 removed for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.15s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg-gated resources compile for f405
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:129:14
    |
129 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:41:9
   |
41 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
54 |     fn init(_cx: init::Context) -> (Shared, Local) {
   |                                     ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

## PASS - cfg-gated resources removed for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg whole task keeps f405 and removes f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:129:14
    |
129 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:41:9
   |
41 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
54 |     fn init(_cx: init::Context) -> (Shared, Local) {
   |                                     ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
```

## PASS - cfg whole task keeps f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.16s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg task header keeps f405 and removes f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:129:14
    |
129 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:41:9
   |
41 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
54 |     fn init(_cx: init::Context) -> (Shared, Local) {
   |                                     ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
```

## PASS - cfg task header keeps f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.13s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg task resource keeps f405 and removes f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:129:14
    |
129 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:41:9
   |
41 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
54 |     fn init(_cx: init::Context) -> (Shared, Local) {
   |                                     ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

## PASS - cfg task resource keeps f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.15s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - multiple heads rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,f405`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4xx-hal v0.23.0
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling stm32f4xx-hal v0.23.0:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
error: failed to run custom build command for `stm32f4xx-hal v0.23.0`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\stm32f4xx-hal-f4106b5613392783\build-s
cript-build` (exit code: 101)
  --- stderr

  thread 'main' (31308) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4xx-hal-0
.23.0\build.rs:31:39:
  Multiple stm32xx Cargo features enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-c34bc511e5d5762a\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/config.toml
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/config.toml
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/config.toml
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411

  --- stderr

  thread 'main' (7496) panicked at build.rs:271:5:
  assertion `left == right` failed: exactly one chip feature must be selected
    left: 2
   right: 1
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## PASS - invalid interrupt rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,invalid-interrupt`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe : warning: error finalizing incremental compilation session directory `\\?\C:\ws\rtic-code-skeleton\target\de
bug\incremental\build_script_build-2q3ejlxm4ph0e\s-hm6dvbxey8-1rfbfmh-working`: The process cannot access the file beca
use it is being used by another process. (os error 32)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: error .... (os error 32):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 

warning: `rtic-code-skeleton` (build script) generated 1 warning
   Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
error[E0599]: no variant or associated item named `UART8` found for enum `Interrupt` in the current scope
  --> C:\ws\rtic-code-skeleton\target\thumbv7em-none-eabihf\debug\build\rtic-code-skeleton-af67d71cd8e8cd4e\out/generat
ed_app_f401.rs:33:20
   |
 3 | / #[rtic::app(
 4 | |     device = stm32f4xx_hal::pac,
 5 | |     dispatchers = [TIM2]
 6 | | )]
...  |
33 | |     #[task(binds = UART8)]
   | |                   -^^^^^ variant or associated item not found in `Interrupt`
   | |___________________|
   |

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rtic-code-skeleton` (bin "rtic-code-skeleton") due to 1 previous error
```

## PASS - no chip rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4 v0.16.0
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:60 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling stm32f4 v0.16.0:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling stm32f4xx-hal v0.23.0
   Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
error: failed to run custom build command for `stm32f4 v0.16.0`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\stm32f4-b3557ee3c8a15aba\build-script-
build` (exit code: 101)
  --- stdout
  cargo:rustc-link-search=C:\ws\rtic-code-skeleton\target\thumbv7em-none-eabihf\debug\build\stm32f4-dc0b7bb315eaef6e\ou
t

  --- stderr

  thread 'main' (29124) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4-0.16.0\
build.rs:20:13:
  No device features selected. Avaliable device features are: ["stm32f401", "stm32f405", "stm32f407", "stm32f410", "stm
32f411", "stm32f412", "stm32f413", "stm32f427", "stm32f429", "stm32f446", "stm32f469"]
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `stm32f4xx-hal v0.23.0`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\stm32f4xx-hal-4bc1b1ecd95cd6e9\build-s
cript-build` (exit code: 101)
  --- stderr

  thread 'main' (9472) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4xx-hal-0.
23.0\build.rs:30:35:
  No stm32xx Cargo feature enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-f4c6be67eadf51d9\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/config.toml
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/config.toml
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/config.toml
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411

  --- stderr

  thread 'main' (24464) panicked at build.rs:271:5:
  assertion `left == right` failed: exactly one chip feature must be selected
    left: 0
   right: 1
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Result: PASS
