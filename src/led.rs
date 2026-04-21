use color::{AlphaColor, ColorSpace};
use embedded_hal::pwm::SetDutyCycle;
use stm32g4xx_hal::pwm::{ActiveHigh, C1, C2, C3, ComplementaryImpossible, Pwm};

type PwmPin<TIM, C> = Pwm<TIM, C, ComplementaryImpossible, ActiveHigh, ActiveHigh>;

pub struct LED<TIM> {
    c1: PwmPin<TIM, C1>,
    c2: PwmPin<TIM, C2>,
    c3: PwmPin<TIM, C3>,
}

impl<TIM> LED<TIM> {
    pub fn new(pins: (PwmPin<TIM, C1>, PwmPin<TIM, C2>, PwmPin<TIM, C3>)) -> Self {
        let (c1, c2, c3) = pins;
        Self { c1, c2, c3 }
    }
}

impl<TIM> LED<TIM>
where
    PwmPin<TIM, C1>: SetDutyCycle,
    PwmPin<TIM, C2>: SetDutyCycle,
    PwmPin<TIM, C3>: SetDutyCycle,
{
    pub fn set<CS: ColorSpace>(&mut self, color: AlphaColor<CS>) {
        let [r, g, b, _] = color.premultiply().components;
        // Every Pin I've looked at has Infallible as it's error type
        let _ = self.c1.set_duty_cycle_fraction(r as u16, 255);
        let _ = self.c2.set_duty_cycle_fraction(g as u16, 255);
        let _ = self.c3.set_duty_cycle_fraction(b as u16, 255);
    }
}
