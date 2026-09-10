#[rtic::app(
    device = stm32f4xx_hal::pac,
    dispatchers = [TIM2]
)]
mod app {
    /* APP_BODY */

    #[task(binds = UART8)]
    fn uart8(_: uart8::Context) {}
}
