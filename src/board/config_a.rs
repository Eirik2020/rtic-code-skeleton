//! Board A role table: TIM2 -> motor PWM, TIM3 -> RC capture.

pub type Tim2Role = crate::roles::motor_pwm::MotorPwm;
pub type Tim3Role = crate::roles::rc_capture::RcCapture;
