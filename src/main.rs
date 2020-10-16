#![no_std]
#![no_main]

mod game;

use crate::game::Game;
use cortex_m_rt::entry;
use hal::adc::Adc;
use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32;
use hal::time::MegaHertz;
use hal::timer::Timer;
use panic_reset as _;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use smart_leds::{self, colors, SmartLedsWrite, RGB8};
use stm32f1xx_hal as hal;
use ws2812_timer_delay::Ws2812;

const ALIVE: RGB8 = colors::RED;
const DEAD: RGB8 = colors::BLACK;
const BRIGHTNESS: u8 = 10;
const DELAY: u16 = 500;
const WIDTH: usize = 16;
const PIXELS: usize = WIDTH * WIDTH;

fn update_screen(game: &Game, mut screen: [RGB8; PIXELS]) -> [RGB8; PIXELS] {
    let mut index = 0;
    for j in 0..WIDTH {
        let mut step: isize = 1;
        let mut i = 0;

        // the panel I have snakes around at the edge
        if j % 2 != 0 {
            step = -1;
            i = WIDTH - 1;
        }

        while i < WIDTH {
            if game.is_alive(i, j) {
                screen[index] = ALIVE;
            } else {
                screen[index] = DEAD;
            }

            index += 1;
            i = (i as isize + step) as usize;
        }
    }

    screen
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.adcclk(2.mhz()).freeze(&mut flash.acr);

    let mut adc1 = Adc::adc1(dp.ADC1, &mut rcc.apb2, clocks);
    let timer = Timer::tim1(dp.TIM1, &clocks, &mut rcc.apb2).start_count_down(MegaHertz(3));
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // poor excuse for entropy but it works
    let mut ch0 = gpiob.pb0.into_analog(&mut gpiob.crl);
    let seed: u16 = adc1.read(&mut ch0).unwrap();
    let mut rng = SmallRng::seed_from_u64(seed as u64);

    let data_pin = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut ws = Ws2812::new(timer, data_pin);

    let mut game = Game::random(&mut rng);
    let mut screen: [RGB8; PIXELS] = [RGB8::default(); PIXELS];

    loop {
        screen = update_screen(&game, screen);

        let data = smart_leds::brightness(screen.iter().cloned(), BRIGHTNESS);
        ws.write(data).unwrap();

        let next = game.next().unwrap();
        if game == next {
            game = Game::random(&mut rng);
        } else {
            game = next;
        }

        delay.delay_ms(DELAY);
    }
}
