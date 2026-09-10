#[shared]
struct Shared {
    counter: u32,
    #[cfg(feature = "f405")]
    f405_shared: u32,
}

#[local]
struct Local {
    buffer: [u8; 32],
    #[cfg(feature = "f405")]
    f405_buffer: [u8; 4],
}

#[init]
fn init(_: init::Context) -> (Shared, Local) {
    (
        Shared {
            counter: 0,
            #[cfg(feature = "f405")]
            f405_shared: 0,
        },
        Local {
            buffer: [0; 32],
            #[cfg(feature = "f405")]
            f405_buffer: [0; 4],
        },
    )
}

#[task(binds = USART1, shared = [counter], local = [buffer])]
fn usart1(mut cx: usart1::Context) {
    cx.shared.counter.lock(|counter| {
        *counter += 1;
    });
    cx.local.buffer[0] = 1;
}

#[cfg(feature = "f405")]
#[task(binds = UART4)]
fn f405_uart4(_: f405_uart4::Context) {}

// Whole task gated: the F411 control task conflicts if both survive reduction.
#[cfg(feature = "f405")]
#[task(binds = USART6)]
fn f405_whole_task(_: f405_whole_task::Context) {}

#[cfg(feature = "f411")]
#[task(binds = USART6)]
fn f411_whole_control(_: f411_whole_control::Context) {}

// Task header gated: the cfg is after the RTIC task attribute.
#[task(binds = USART2)]
#[cfg(feature = "f405")]
fn f405_header_task(_: f405_header_task::Context) {}

#[task(binds = USART2)]
#[cfg(feature = "f411")]
fn f411_header_control(_: f411_header_control::Context) {}

// Task resource gated: F411's control task conflicts if this task survives.
#[cfg(feature = "f405")]
#[task(binds = TIM4, local = [f405_buffer])]
fn f405_resource_task(cx: f405_resource_task::Context) {
    cx.local.f405_buffer[0] = 1;
}

#[cfg(feature = "f411")]
#[task(binds = TIM4)]
fn f411_resource_control(_: f411_resource_control::Context) {}

// Compound cfg probe: all/not keep the F405 task, while any keeps the F411/F401 control.
#[cfg(all(feature = "f405", not(feature = "f401")))]
#[task(binds = TIM5)]
fn f405_compound_task(_: f405_compound_task::Context) {}

#[cfg(any(feature = "f411", feature = "f401"))]
#[task(binds = TIM5)]
fn non_f405_compound_control(_: non_f405_compound_control::Context) {}

#[cfg(feature = "f405")]
async fn f405_software_task(_: u32) {}
