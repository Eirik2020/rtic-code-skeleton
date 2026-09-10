//! Board B role table: TIM2 -> RC capture, TIM3 -> motor PWM (swapped vs. board A).

pub type Tim2Role = crate::roles::rc_capture::RcCapture;
pub type Tim3Role = crate::roles::motor_pwm::MotorPwm;
