#[rtic::app(
    device = stm32f4xx_hal::pac,
    dispatchers = [TIM2]
)]
mod app {
    /* APP_BODY */
}
