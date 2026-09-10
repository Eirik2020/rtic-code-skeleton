# RTIC2 Multi-Head App Composition Test Report

- Run: 2026-09-10 21:27:17 +02:00
- Target: `thumbv7em-none-eabihf`
- Manifest: `C:\ws\rtic-code-skeleton\rtic2-multi-head-test\Cargo.toml`

## PASS - f401 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.20s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.20s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:1177
  |
8 | ...ture = "f405")] async fn f405_software_task (_ : u32) { }
  |                             ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:48
  |
8 | ... , f405_shared : u32 } # [local] struct Local { buffer : [u8 ; 32] , f40
5_buffer : [u8 ; 4] } # [init] fn init (_ : init :: Context) -> (Shared , ...
  |       ^^^^^^^^^^^                                                          
                                                                 ------ field i
n this struct

warning: `rtic2-multi-head-test` (bin "rtic2-multi-head-test") generated 2 warn
ings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
```

## PASS - f411 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.20s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.20s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f401 generated body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,generated-body`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.20s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.20s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405 generated body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405,generated-body`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.21s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.21s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - syn reduction removes f405-only UART4 for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.22s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.22s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405-only UART4 retained
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:1177
  |
8 | ...ture = "f405")] async fn f405_software_task (_ : u32) { }
  |                             ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:48
  |
8 | ... , f405_shared : u32 } # [local] struct Local { buffer : [u8 ; 32] , f40
5_buffer : [u8 ; 4] } # [init] fn init (_ : init :: Context) -> (Shared , ...
  |       ^^^^^^^^^^^                                                          
                                                                 ------ field i
n this struct

warning: `rtic2-multi-head-test` (bin "rtic2-multi-head-test") generated 2 warn
ings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
```

## PASS - f405-only UART4 removed for f401
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.22s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.22s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405-only UART4 removed for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.19s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.19s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg-gated resources compile for f405
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:1177
  |
8 | ...ture = "f405")] async fn f405_software_task (_ : u32) { }
  |                             ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:48
  |
8 | ... , f405_shared : u32 } # [local] struct Local { buffer : [u8 ; 32] , f40
5_buffer : [u8 ; 4] } # [init] fn init (_ : init :: Context) -> (Shared , ...
  |       ^^^^^^^^^^^                                                          
                                                                 ------ field i
n this struct

warning: `rtic2-multi-head-test` (bin "rtic2-multi-head-test") generated 2 warn
ings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
```

## PASS - cfg-gated resources removed for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.19s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.19s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg whole task keeps f405 and removes f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:1177
  |
8 | ...ture = "f405")] async fn f405_software_task (_ : u32) { }
  |                             ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:48
  |
8 | ... , f405_shared : u32 } # [local] struct Local { buffer : [u8 ; 32] , f40
5_buffer : [u8 ; 4] } # [init] fn init (_ : init :: Context) -> (Shared , ...
  |       ^^^^^^^^^^^                                                          
                                                                 ------ field i
n this struct

warning: `rtic2-multi-head-test` (bin "rtic2-multi-head-test") generated 2 warn
ings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
```

## PASS - cfg whole task keeps f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.22s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.22s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg task header keeps f405 and removes f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:1177
  |
8 | ...ture = "f405")] async fn f405_software_task (_ : u32) { }
  |                             ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:48
  |
8 | ... , f405_shared : u32 } # [local] struct Local { buffer : [u8 ; 32] , f40
5_buffer : [u8 ; 4] } # [init] fn init (_ : init :: Context) -> (Shared , ...
  |       ^^^^^^^^^^^                                                          
                                                                 ------ field i
n this struct

warning: `rtic2-multi-head-test` (bin "rtic2-multi-head-test") generated 2 warn
ings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
```

## PASS - cfg task header keeps f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.19s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.19s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - cfg task resource keeps f405 and removes f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe : warning: function `f405_software_task` is never used
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: functi...` is never used 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:1177
  |
8 | ...ture = "f405")] async fn f405_software_task (_ : u32) { }
  |                             ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `f405_shared` is never read
 --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabih
f\debug\build\rtic2-multi-head-test-91ceb10465c7facc\out/generated_app_f405.rs:
8:48
  |
8 | ... , f405_shared : u32 } # [local] struct Local { buffer : [u8 ; 32] , f40
5_buffer : [u8 ; 4] } # [init] fn init (_ : init :: Context) -> (Shared , ...
  |       ^^^^^^^^^^^                                                          
                                                                 ------ field i
n this struct

warning: `rtic2-multi-head-test` (bin "rtic2-multi-head-test") generated 2 warn
ings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
```

## PASS - cfg task resource keeps f411 control
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.23s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.23s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - multiple heads rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,f405`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4xx-hal v0.23.0
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling stm32f4xx-hal v0.23. 
   0:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling rtic2-multi-head-test v0.1.0 (C:\ws\rtic-code-skeleton\rtic2-multi
-head-test)
error: failed to run custom build command for `stm32f4xx-hal v0.23.0`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\rtic2-multi-head-
test\target\debug\build\stm32f4xx-hal-f4106b5613392783\build-script-build` (exi
t code: 101)
  --- stderr

  thread 'main' (17812) panicked at C:\Users\xoonz\.cargo\registry\src\index.cr
ates.io-1949cf8c6b5b557f\stm32f4xx-hal-0.23.0\build.rs:31:39:
  Multiple stm32xx Cargo features enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
```

## PASS - invalid interrupt rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,invalid-interrupt`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :     Checking rtic2-multi-head-test v0.1.0 (C:\ws\rtic-code-skeleton
\rtic2-multi-head-test)
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking rt...ulti-head-test) 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error[E0599]: no variant or associated item named `UART8` found for enum `Inter
rupt` in the current scope
  --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabi
hf\debug\build\rtic2-multi-head-test-3387d51def075bbc\out/generated_app_f401.rs
:10:20
   |
 3 | / #[rtic::app(
 4 | |     device = stm32f4xx_hal::pac,
 5 | |     dispatchers = [TIM2]
 6 | | )]
...  |
10 | |     #[task(binds = UART8)]
   | |                   -^^^^^ variant or associated item not found in `Interr
upt`
   | |___________________|
   |

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rtic2-multi-head-test` (bin "rtic2-multi-head-test") 
due to 1 previous error
```

## PASS - no chip rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4 v0.16.0
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\tools\run-test.ps1:50 char:19
+         $output = & cargo @cargoArgs 2>&1 | Out-String
+                   ~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling stm32f4 v0.16.0:Stri 
   ng) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling stm32f4xx-hal v0.23.0
   Compiling rtic2-multi-head-test v0.1.0 (C:\ws\rtic-code-skeleton\rtic2-multi
-head-test)
error: failed to run custom build command for `stm32f4 v0.16.0`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\rtic2-multi-head-
test\target\debug\build\stm32f4-b3557ee3c8a15aba\build-script-build` (exit code
: 101)
  --- stdout
  cargo:rustc-link-search=C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target
\thumbv7em-none-eabihf\debug\build\stm32f4-dc0b7bb315eaef6e\out

  --- stderr

  thread 'main' (1708) panicked at C:\Users\xoonz\.cargo\registry\src\index.cra
tes.io-1949cf8c6b5b557f\stm32f4-0.16.0\build.rs:20:13:
  No device features selected. Avaliable device features are: ["stm32f401", "st
m32f405", "stm32f407", "stm32f410", "stm32f411", "stm32f412", "stm32f413", "stm
32f427", "stm32f429", "stm32f446", "stm32f469"]
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `stm32f4xx-hal v0.23.0`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\rtic2-multi-head-
test\target\debug\build\stm32f4xx-hal-4bc1b1ecd95cd6e9\build-script-build` (exi
t code: 101)
  --- stderr

  thread 'main' (22752) panicked at C:\Users\xoonz\.cargo\registry\src\index.cr
ates.io-1949cf8c6b5b557f\stm32f4xx-hal-0.23.0\build.rs:30:35:
  No stm32xx Cargo feature enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: failed to run custom build command for `rtic2-multi-head-test v0.1.0 (C:
\ws\rtic-code-skeleton\rtic2-multi-head-test)`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\rtic2-multi-head-
test\target\debug\build\rtic2-multi-head-test-c50cbea5efc43914\build-script-bui
ld` (exit code: 101)
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

  thread 'main' (25436) panicked at build.rs:158:28:
  one chip feature must be selected
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Result: PASS
