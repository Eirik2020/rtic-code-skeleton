# RTIC2 Multi-Head App Composition Test Report

- Run: 2026-09-12 18:36:52 +02:00
- Target: `thumbv7em-none-eabihf`
- Manifest: `C:\ws\rtic-code-skeleton\Cargo.toml`

## PASS - default F401 system
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.32s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - default F405 system
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:201:14
    |
201 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:51:9
   |
51 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
68 |     fn init(cx: init::Context) -> (Shared, Local) {
   |                                    ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
```

## PASS - default F411 system
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.20s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - two systems sharing F401 rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f401,system-nucleo-f401re`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-2a25cf86467474ac\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_NUCLEO_F401RE
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SW_REPORT
  cargo:rerun-if-env-changed=CARGO_FEATURE_GENERATED_BODY
  cargo:rerun-if-env-changed=CARGO_FEATURE_INVALID_INTERRUPT

  --- stderr

  thread 'main' (20900) panicked at build.rs:174:5:
  select at most one system feature
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## PASS - systems using different chips rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-default-f401,system-default-f405`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4xx-hal v0.23.0
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
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

  thread 'main' (13976) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4xx-hal-0
.23.0\build.rs:31:39:
  Multiple stm32xx Cargo features enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-69f18a9557abbd39\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_NUCLEO_F401RE
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SW_REPORT
  cargo:rerun-if-env-changed=CARGO_FEATURE_GENERATED_BODY
  cargo:rerun-if-env-changed=CARGO_FEATURE_INVALID_INTERRUPT

  --- stderr

  thread 'main' (13272) panicked at build.rs:359:5:
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
cargo.exe :     Checking rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
```

## PASS - Nucleo prototype enables report software
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-nucleo-f401re,sw-report`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Checking rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
```

## PASS - report software without prototype board rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,sw-report`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-0c030b7aa3bba136\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_NUCLEO_F401RE
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SW_REPORT
  cargo:rerun-if-env-changed=CARGO_FEATURE_GENERATED_BODY
  cargo:rerun-if-env-changed=CARGO_FEATURE_INVALID_INTERRUPT
  cargo:rustc-check-cfg=cfg(board_peripheral, values("gpioa", "tim3"))
  cargo:rustc-check-cfg=cfg(board_pin, values("status_led"))

  --- stderr

  thread 'main' (24904) panicked at build.rs:315:32:
  feature 'sw-report' requires the Nucleo board prototype
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## PASS - Nucleo system accepts explicit matching chip
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-nucleo-f401re,f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - Nucleo system rejects incompatible chip
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-nucleo-f401re,f405`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4xx-hal v0.23.0
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
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

  thread 'main' (34200) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4xx-hal-0
.23.0\build.rs:31:39:
  Multiple stm32xx Cargo features enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-2285382f05f52f6e\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_NUCLEO_F401RE
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SW_REPORT
  cargo:rerun-if-env-changed=CARGO_FEATURE_GENERATED_BODY
  cargo:rerun-if-env-changed=CARGO_FEATURE_INVALID_INTERRUPT

  --- stderr

  thread 'main' (35608) panicked at build.rs:359:5:
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
cargo.exe :    Compiling rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-e904b689a2ef5e71\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_NUCLEO_F401RE
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SW_REPORT
  cargo:rerun-if-env-changed=CARGO_FEATURE_GENERATED_BODY
  cargo:rerun-if-env-changed=CARGO_FEATURE_INVALID_INTERRUPT

  --- stderr

  thread 'main' (29452) panicked at build.rs:187:9:
  system features cannot be combined with the generated-body fixture
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## PASS - Nucleo system rejects invalid interrupt
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features system-nucleo-f401re,invalid-interrupt`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :     Checking rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `blink`
  --> C:\ws\rtic-code-skeleton\target\thumbv7em-none-eabihf\debug\build\rtic-code-skeleton-c34febde79b2fa41\out/generat
ed_app_f401.rs:44:17
   |
44 |         let _ = blink::spawn();
   |                 ^^^^^ use of unresolved module or unlinked crate `blink`
   |
   = help: if you wanted to use a crate named `blink`, use `cargo add blink` to add it to your `Cargo.toml`

error[E0599]: no variant or associated item named `UART8` found for enum `Interrupt` in the current scope
  --> C:\ws\rtic-code-skeleton\target\thumbv7em-none-eabihf\debug\build\rtic-code-skeleton-c34febde79b2fa41\out/generat
ed_app_f401.rs:62:20
   |
 3 | / #[rtic::app(
 4 | |     device = stm32f4xx_hal::pac,
 5 | |     dispatchers = [TIM2]
 6 | | )]
...  |
62 | |     #[task(binds = UART8)]
   | |                   -^^^^^ variant or associated item not found in `Interrupt`
   | |___________________|
   |

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `rtic-code-skeleton` (bin "rtic-code-skeleton") due to 2 previous errors
```

## PASS - f401 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:201:14
    |
201 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:51:9
   |
51 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
68 |     fn init(cx: init::Context) -> (Shared, Local) {
   |                                    ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
```

## PASS - f411 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.21s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f401 generated body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,generated-body`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405 generated body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405,generated-body`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - syn reduction removes f405-only UART4 for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405-only UART4 retained
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:201:14
    |
201 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:51:9
   |
51 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
68 |     fn init(cx: init::Context) -> (Shared, Local) {
   |                                    ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
```

## PASS - f405-only UART4 removed for f401
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405-only UART4 removed for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg-gated resources compile for f405
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:201:14
    |
201 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:51:9
   |
51 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
68 |     fn init(cx: init::Context) -> (Shared, Local) {
   |                                    ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
```

## PASS - cfg-gated resources removed for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg whole task keeps f405 and removes f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:201:14
    |
201 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:51:9
   |
51 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
68 |     fn init(cx: init::Context) -> (Shared, Local) {
   |                                    ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
```

## PASS - cfg whole task keeps f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg task header keeps f405 and removes f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:201:14
    |
201 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:51:9
   |
51 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
68 |     fn init(cx: init::Context) -> (Shared, Local) {
   |                                    ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
```

## PASS - cfg task header keeps f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg task resource keeps f405 and removes f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\app_body_skeleton.rs:201:14
    |
201 |     async fn f405_software_task(_: u32) {}
    |              ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
  --> src\app_body_skeleton.rs:51:9
   |
51 |         f405_shared: u32,
   |         ^^^^^^^^^^^
...
68 |     fn init(cx: init::Context) -> (Shared, Local) {
   |                                    ------ field in this struct

warning: `rtic-code-skeleton` (bin "rtic-code-skeleton") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
```

## PASS - cfg task resource keeps f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - multiple heads rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,f405`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4xx-hal v0.23.0
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
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

  thread 'main' (25008) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4xx-hal-0
.23.0\build.rs:31:39:
  Multiple stm32xx Cargo features enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-b38abf61f28e125c\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_NUCLEO_F401RE
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SW_REPORT
  cargo:rerun-if-env-changed=CARGO_FEATURE_GENERATED_BODY
  cargo:rerun-if-env-changed=CARGO_FEATURE_INVALID_INTERRUPT

  --- stderr

  thread 'main' (8548) panicked at build.rs:359:5:
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
cargo.exe :     Checking rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking rt...-code-skeleton):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error[E0599]: no variant or associated item named `UART8` found for enum `Interrupt` in the current scope
  --> C:\ws\rtic-code-skeleton\target\thumbv7em-none-eabihf\debug\build\rtic-code-skeleton-3bd0286d55c64e92\out/generat
ed_app_f401.rs:62:20
   |
 3 | / #[rtic::app(
 4 | |     device = stm32f4xx_hal::pac,
 5 | |     dispatchers = [TIM2]
 6 | | )]
...  |
62 | |     #[task(binds = UART8)]
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
At C:\ws\rtic-code-skeleton\tools\run-test.ps1:62 char:19
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

  thread 'main' (34992) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4-0.16.0\
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

  thread 'main' (25452) panicked at C:\Users\xoonz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\stm32f4xx-hal-0
.23.0\build.rs:30:35:
  No stm32xx Cargo feature enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: failed to run custom build command for `rtic-code-skeleton v0.1.0 (C:\ws\rtic-code-skeleton)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\target\debug\build\rtic-code-skeleton-22ab30fb7b001edf\bu
ild-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=src/app_body_skeleton.rs
  cargo:rerun-if-changed=src/chips/f401/app_head.rs
  cargo:rerun-if-changed=src/chips/f401/app_body.rs
  cargo:rerun-if-changed=src/chips/f405/app_head.rs
  cargo:rerun-if-changed=src/chips/f405/app_body.rs
  cargo:rerun-if-changed=src/chips/f411/app_head.rs
  cargo:rerun-if-changed=src/chips/f411/app_body.rs
  cargo:rerun-if-changed=src/invalid_interrupt.rs
  cargo:rerun-if-env-changed=CARGO_FEATURE_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_NUCLEO_F401RE
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F401
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F405
  cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM_DEFAULT_F411
  cargo:rerun-if-env-changed=CARGO_FEATURE_SW_REPORT
  cargo:rerun-if-env-changed=CARGO_FEATURE_GENERATED_BODY
  cargo:rerun-if-env-changed=CARGO_FEATURE_INVALID_INTERRUPT

  --- stderr

  thread 'main' (32612) panicked at build.rs:359:5:
  assertion `left == right` failed: exactly one chip feature must be selected
    left: 0
   right: 1
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Result: PASS
