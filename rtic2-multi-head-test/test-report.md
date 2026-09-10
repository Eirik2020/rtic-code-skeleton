# RTIC2 Multi-Head App Composition Test Report

- Run: 2026-09-10 20:38:52 +02:00
- Target: `thumbv7em-none-eabihf`
- Manifest: `C:\ws\rtic-code-skeleton\rtic2-multi-head-test\Cargo.toml`

## PASS - f401 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.18s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.16s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.16s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f411 shared body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.17s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f401 generated body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,generated-body`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.19s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.19s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405 generated body
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405,generated-body`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.16s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.16s 
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
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.22s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405-only UART4 retained
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f405`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.17s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.17s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405-only UART4 removed for f401
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.19s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.19s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - f405-only UART4 removed for f411
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f411`
- Expected exit code: 0
- Actual exit code: 0

```text
cargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0
.18s
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
```

## PASS - multiple heads rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,f405`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :    Compiling stm32f4xx-hal v0.23.0
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling stm32f4xx-hal v0.23. 
   0:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error: failed to run custom build command for `stm32f4xx-hal v0.23.0`

Caused by:
  process didn't exit successfully: `C:\ws\rtic-code-skeleton\rtic2-multi-head-
test\target\debug\build\stm32f4xx-hal-f4106b5613392783\build-script-build` (exi
t code: 101)
  --- stderr

  thread 'main' (16808) panicked at C:\Users\xoonz\.cargo\registry\src\index.cr
ates.io-1949cf8c6b5b557f\stm32f4xx-hal-0.23.0\build.rs:31:39:
  Multiple stm32xx Cargo features enabled
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## PASS - invalid interrupt rejected
`cargo check --offline --target thumbv7em-none-eabihf --no-default-features --features f401,invalid-interrupt`
- Expected exit code: 101
- Actual exit code: 101

```text
cargo.exe :     Checking rtic2-multi-head-test v0.1.0 (C:\ws\rtic-code-skeleton
\rtic2-multi-head-test)
At C:\ws\rtic-code-skeleton\rtic2-multi-head-test\run-test.ps1:36 char:19
+ ...   $output = & cargo check --offline --manifest-path $manifest --targe ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking rt...ulti-head-test) 
   :String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error[E0599]: no variant or associated item named `UART8` found for enum `Inter
rupt` in the current scope
  --> C:\ws\rtic-code-skeleton\rtic2-multi-head-test\target\thumbv7em-none-eabi
hf\debug\build\rtic2-multi-head-test-1830f2e8d65a8433\out/generated_app_f401.rs
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

Result: PASS
