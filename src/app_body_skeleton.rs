// Keep this inline module in the source graph. The reducer must run before RTIC
// and retain token spans so completion and diagnostics map to this file.
// rustfmt::skip: these selector lines are kept one-per-branch by hand; the
// default 100-column width would wrap them back into multi-line form.
#[rustfmt::skip]
#[cfg_attr(all(feature = "system-nucleo-f401re", feature = "sw-report"), rtic_app_cfg::for_chip(f401, "system-nucleo-f401re", software = ["sw-report"]))]
#[cfg_attr(all(feature = "system-nucleo-f401re", not(feature = "sw-report")), rtic_app_cfg::for_chip(f401, "system-nucleo-f401re"))]
#[cfg_attr(all(feature = "f401", not(any(feature = "system-nucleo-f401re", feature = "system-default-f401"))), rtic_app_cfg::for_chip(f401))]
#[cfg_attr(all(feature = "f405", not(feature = "system-default-f405")), rtic_app_cfg::for_chip(f405))]
#[cfg_attr(all(feature = "f411", not(feature = "system-default-f411")), rtic_app_cfg::for_chip(f411))]
#[cfg_attr(feature = "system-default-f401", rtic_app_cfg::for_chip(f401, "system-default-f401"))]
#[cfg_attr(feature = "system-default-f405", rtic_app_cfg::for_chip(f405, "system-default-f405"))]
#[cfg_attr(feature = "system-default-f411", rtic_app_cfg::for_chip(f411, "system-default-f411"))]
#[cfg_attr(feature = "system-nucleo-f401re", rtic::app(device = stm32f4xx_hal::pac, dispatchers = [TIM2]))]
#[cfg_attr(not(feature = "system-nucleo-f401re"), rtic::app(device = stm32f4xx_hal::pac, dispatchers = [TIM2, TIM3]))]
mod app {
    use crate::app_prelude::*;

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
        #[cfg(board_pin = "led1")]
        led1: crate::hardware::gpio::OutputPin,
        #[cfg(board_pin = "led2")]
        led2: crate::hardware::gpio::OutputPin,
        #[cfg(board_pin = "led3")]
        led3: crate::hardware::gpio::OutputPin,
        #[cfg(board_peripheral = "usart1")]
        serial1: crate::hardware::serial::Usart1,
        #[cfg(board_peripheral = "usart2")]
        serial2: crate::hardware::serial::Usart2,
        #[cfg(board_peripheral = "usart6")]
        serial3: crate::hardware::serial::Usart6,
        #[cfg(board_peripheral = "tim3")]
        timer1: crate::hardware::timer::Tim3,
        #[cfg(board_peripheral = "tim3")]
        report_route_enabled: bool,
        #[cfg(board_peripheral = "tim5")]
        timer2: crate::hardware::timer::Tim5,
        #[cfg(board_peripheral = "tim4")]
        pwm_timer: crate::hardware::pwm::Tim4,
        #[cfg(board_pin = "pwm1")]
        pwm_output1: crate::hardware::pwm::Tim4Ch1,
        #[cfg(board_pin = "pwm2")]
        pwm_output2: crate::hardware::pwm::Tim4Ch2,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Clock setup belongs to the unified initializer. The Config value
        // comes from the active system's [clock] selection, generated and
        // validated in build.rs, including any board-specific adjustment
        // (e.g. HSE bypass mode) baked in ahead of time. A board with real
        // hardware freezes RCC itself, inside `Board::new`, because it takes
        // the whole PAC peripheral bundle rather than one parameter per
        // peripheral — see `docs/architecture/decisions.md`. Either way this
        // is gated on board *capability* (`board_pin`/`board_peripheral`),
        // never a system's own feature name, so this file's shape doesn't
        // change as boards are added.
        let counter = 0;
        #[cfg(feature = "f405")]
        let f405_shared = 0;
        let buffer = [0; 32];
        #[cfg(feature = "f405")]
        let f405_buffer = [0; 4];

        #[cfg(any(board_pin = "led1", board_peripheral = "tim3"))]
        let initialized = crate::boards::active::Board::new(
            cx.device,
            crate::clock::config(),
            crate::boards::active::Settings {
                // Each port's settings come from its default routed protocol
                // (see crate::routing, generated from the active system's
                // default_serial_routes); an unrouted port falls back to a
                // generic default (crate::software::serial_protocol::resolve).
                // The route decides *settings* only here — no protocol
                // decoding logic is wired in yet.
                serial1: crate::software::serial_protocol::resolve(crate::routing::serial1_protocol()),
                serial2: crate::software::serial_protocol::resolve(crate::routing::serial2_protocol()),
                serial3: crate::software::serial_protocol::resolve(crate::routing::serial3_protocol()),
                timer1_frequency: 1.Hz(),
                timer2_frequency: 10.Hz(),
                pwm_frequency: 400.Hz(),
            },
        );
        #[cfg(any(board_pin = "led1", board_peripheral = "tim3"))]
        let sysclk_hz = initialized.sysclk_hz;

        #[cfg(not(any(board_pin = "led1", board_peripheral = "tim3")))]
        let rcc = cx.device.RCC.freeze(crate::clock::config());
        #[cfg(not(any(board_pin = "led1", board_peripheral = "tim3")))]
        let sysclk_hz = rcc.clocks.sysclk().raw();

        Mono::start(cx.core.SYST, sysclk_hz);

        #[cfg(any(board_pin = "led1", board_peripheral = "tim3"))]
        let crate::boards::active::Board {
            led1,
            led2,
            led3,
            serial1,
            serial2,
            serial3,
            timer1,
            timer2,
            pwm:
                crate::boards::active::PwmOutputs {
                    timer: pwm_timer,
                    output1: pwm_output1,
                    output2: pwm_output2,
                },
        } = initialized.hardware;
        // This is the prototype boot route. A persistent/runtime configuration
        // can replace the boolean without changing TIM3's RTIC ownership.
        #[cfg(all(board_peripheral = "tim3", feature = "sw-report"))]
        let report_route_enabled = true;
        #[cfg(all(board_peripheral = "tim3", not(feature = "sw-report")))]
        let report_route_enabled = false;

        #[cfg(board_pin = "led1")]
        let _ = blink::spawn();
        #[cfg(board_peripheral = "tim4")]
        let _ = start_pwm::spawn();

        let shared = Shared {
            counter,
            #[cfg(feature = "f405")]
            f405_shared,
        };
        let local = Local {
            buffer,
            #[cfg(feature = "f405")]
            f405_buffer,
            #[cfg(board_pin = "led1")]
            led1,
            #[cfg(board_pin = "led2")]
            led2,
            #[cfg(board_pin = "led3")]
            led3,
            #[cfg(board_peripheral = "usart1")]
            serial1,
            #[cfg(board_peripheral = "usart2")]
            serial2,
            #[cfg(board_peripheral = "usart6")]
            serial3,
            #[cfg(board_peripheral = "tim3")]
            timer1,
            #[cfg(board_peripheral = "tim3")]
            report_route_enabled,
            #[cfg(board_peripheral = "tim5")]
            timer2,
            #[cfg(board_peripheral = "tim4")]
            pwm_timer,
            #[cfg(board_pin = "pwm1")]
            pwm_output1,
            #[cfg(board_pin = "pwm2")]
            pwm_output2,
        };
        (shared, local)
    }

    #[cfg(board_pin = "led1")]
    #[task(local = [led1, led2, led3])]
    async fn blink(cx: blink::Context) {
        loop {
            cx.local.led1.on();
            cx.local.led2.on();
            cx.local.led3.on();
            Mono::delay(500.millis()).await;
            cx.local.led1.off();
            cx.local.led2.off();
            cx.local.led3.off();
            Mono::delay(500.millis()).await;
        }
    }

    #[cfg(board_peripheral = "tim3")]
    #[task(binds = TIM3, local = [timer1, report_route_enabled])]
    fn report_timer_interrupt(cx: report_timer_interrupt::Context) {
        cx.local.timer1.clear_interrupt();
        if *cx.local.report_route_enabled {
            #[cfg(feature = "sw-report")]
            let _ = report::spawn();
        }
    }

    #[cfg(feature = "sw-report")]
    #[task]
    async fn report(_: report::Context) {
        defmt::info!("Hello World!");
    }

    #[cfg(board_peripheral = "usart1")]
    #[task(binds = USART1, shared = [counter], local = [buffer, serial1])]
    fn usart1(mut cx: usart1::Context) {
        let _ = cx.local.serial1;
        cx.shared.counter.lock(|counter| {
            *counter += 1;
        });
        cx.local.buffer[0] = 1;
    }

    #[cfg(not(board_peripheral = "usart1"))]
    #[task(binds = USART1, shared = [counter], local = [buffer])]
    fn usart1(mut cx: usart1::Context) {
        cx.shared.counter.lock(|counter| {
            *counter += 1;
        });
        cx.local.buffer[0] = 1;
    }

    #[cfg(board_peripheral = "usart2")]
    #[task(binds = USART2, local = [serial2])]
    fn serial2_interrupt(cx: serial2_interrupt::Context) {
        let _ = cx.local.serial2;
    }

    #[cfg(board_peripheral = "usart6")]
    #[task(binds = USART6, local = [serial3])]
    fn serial3_interrupt(cx: serial3_interrupt::Context) {
        let _ = cx.local.serial3;
    }

    #[cfg(board_peripheral = "tim5")]
    #[task(binds = TIM5, shared = [counter], local = [timer2])]
    fn timer2_interrupt(mut cx: timer2_interrupt::Context) {
        cx.local.timer2.clear_interrupt();
        cx.shared.counter.lock(|counter| {
            *counter = counter.wrapping_add(1);
        });
    }

    #[cfg(board_peripheral = "tim4")]
    #[task(local = [pwm_timer, pwm_output1, pwm_output2])]
    async fn start_pwm(cx: start_pwm::Context) {
        let _ = cx.local.pwm_timer;
        cx.local.pwm_output1.set_duty(0);
        cx.local.pwm_output2.set_duty(0);
        cx.local.pwm_output1.enable();
        cx.local.pwm_output2.enable();
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

    #[cfg(any(
        feature = "f411",
        all(feature = "f401", not(feature = "system-nucleo-f401re"))
    ))]
    #[task(binds = TIM5)]
    fn non_f405_compound_control(_: non_f405_compound_control::Context) {}

    #[cfg(feature = "f405")]
    async fn f405_software_task(_: u32) {}
}
