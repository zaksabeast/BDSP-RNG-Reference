use crate::xorshift::Xorshift;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fidget {
    Idle,
    LookAround,
    TapFoot,
    RaiseArms,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Blink {
    Single,
    Double,
    NoBlink,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Animation {
    Fidget(Fidget),
    Blink(Blink), // Handled outside of the others
}

impl fmt::Display for Animation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Animation::Fidget(fidget) => {
                write!(f, "Fidget::{:?}", fidget)
            }
            Animation::Blink(blink) => {
                write!(f, "Blink::{:?}", blink)
            }
        }
    }
}

struct FidgetDuration {
    duration: u32,
    fidget: Fidget,
}

#[derive(Clone, Copy, Debug)]
pub struct AnimationTime {
    seconds: f64,
    animation: Animation,
    rng_state: [u32; 4],
}

impl fmt::Display for AnimationTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Seconds: {}, Animation: {}, RNG state: [{:08x}, {:08x}, {:08x}, {:08x}]",
            self.seconds,
            self.animation,
            self.rng_state[0],
            self.rng_state[1],
            self.rng_state[2],
            self.rng_state[3],
        )
    }
}

pub struct Timeline(Vec<AnimationTime>);

impl fmt::Display for Timeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_timeline = self
            .0
            .iter()
            .map(|animation_time| animation_time.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", formatted_timeline)
    }
}

fn get_next_fidget_cycle(rng: &mut Xorshift, previous_fidget: Fidget) -> Fidget {
    match (previous_fidget, rng.next_max(3)) {
        (Fidget::Idle, 0) => Fidget::LookAround,
        (Fidget::Idle, 1) => Fidget::TapFoot,
        (Fidget::Idle, _) => Fidget::RaiseArms,
        (Fidget::LookAround, 0) => Fidget::TapFoot,
        (Fidget::LookAround, 1) => Fidget::RaiseArms,
        (Fidget::LookAround, _) => Fidget::Idle,
        (Fidget::TapFoot, 0) => Fidget::LookAround,
        (Fidget::TapFoot, 1) => Fidget::RaiseArms,
        (Fidget::TapFoot, _) => Fidget::Idle,
        (Fidget::RaiseArms, 0) => Fidget::LookAround,
        (Fidget::RaiseArms, 1) => Fidget::TapFoot,
        (Fidget::RaiseArms, _) => Fidget::Idle,
    }
}

fn handle_fidget(rng: &mut Xorshift, previous_fidget: Fidget) -> FidgetDuration {
    let next_fidget = get_next_fidget_cycle(rng, previous_fidget);
    let duration = if next_fidget == Fidget::Idle { 25 } else { 20 };

    FidgetDuration {
        duration,
        fidget: next_fidget,
    }
}

pub fn create_timeline(rng: &mut Xorshift, duration: u32, include_all_rng_calls: bool) -> Timeline {
    let mut result = vec![];
    // Each cycle is 0.2 seconds (1/5 second)
    let last_cycle = duration * 5;
    let mut next_fidget_cycle = 25;
    let mut current_fidget = Fidget::Idle;

    for current_cycle in 1..=last_cycle {
        // A fidget rng call is made in 5 seconds (25 cycles)
        // if the character is idling and 0.2 seconds (1 cycle) if not
        if current_cycle == next_fidget_cycle {
            let fidget_result = handle_fidget(rng, current_fidget);
            current_fidget = fidget_result.fidget;
            next_fidget_cycle += fidget_result.duration;

            if fidget_result.fidget != Fidget::Idle || include_all_rng_calls {
                result.push(AnimationTime {
                    animation: Animation::Fidget(fidget_result.fidget),
                    seconds: f64::from(current_cycle) / 5.0,
                    rng_state: rng.get_state(),
                });
            }
        }

        // Each blink is 1 second or 5 cycles
        if current_cycle % 5 == 0 {
            // Lucas (and probably other npcs) have 16 blink patterns
            // Only patterns 0 and 1 have animations
            // 2-15 do not hav animations
            let blink_pattern = rng.next_max(16);
            let blink = match blink_pattern {
                0 => Blink::Single,
                1 => Blink::Double,
                _ => Blink::NoBlink,
            };

            if blink != Blink::NoBlink || include_all_rng_calls {
                result.push(AnimationTime {
                    seconds: f64::from(current_cycle) / 5.0,
                    animation: Animation::Blink(blink),
                    rng_state: rng.get_state(),
                });
            }
        }
    }

    Timeline(result)
}
