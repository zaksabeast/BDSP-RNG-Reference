use crate::xorshift::Xorshift;
use std::fmt;
use std::str::FromStr;

type Seconds = f64;

type Frames = u32;

fn u32_to_seconds(seconds: u32) -> Seconds {
    Seconds::from(seconds)
}

fn seconds_to_frames(seconds: Seconds) -> Frames {
    // Not amazing, but casting should have the intended effect
    (seconds * 30.0) as Frames
}

fn frames_to_seconds(frames: Frames) -> Seconds {
    Seconds::from(frames) / 30.0
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fidget {
    Idle,
    LookAround,
    TapFoot,
    RaiseArms,
}

impl Fidget {
    fn get_frames(&self) -> u32 {
        match self {
            Fidget::Idle => 150,       // 5 seconds
            Fidget::LookAround => 109, // 3.83333349 - 0.2 seconds
            Fidget::TapFoot => 96,     // 3.4000001 - 0.2 seconds
            Fidget::RaiseArms => 119,  // 4.16666698 - 0.2 seconds
        }
    }
}

// This will allow NoBlink to exist,
// which is a better UX than "None"
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Blink {
    Single,
    Double,
    NoBlink,
}

impl Blink {
    fn get_frames() -> Frames {
        30
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Animation {
    Fidget(Fidget),
    Blink(Blink), // Handled outside of the others
}

impl FromStr for Animation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Idle" => Ok(Animation::Fidget(Fidget::Idle)),
            "LookAround" => Ok(Animation::Fidget(Fidget::LookAround)),
            "TapFoot" => Ok(Animation::Fidget(Fidget::TapFoot)),
            "RaiseArms" => Ok(Animation::Fidget(Fidget::RaiseArms)),
            "Single" => Ok(Animation::Blink(Blink::Single)),
            "Double" => Ok(Animation::Blink(Blink::Double)),
            "NoBlink" => Ok(Animation::Blink(Blink::NoBlink)),
            _ => Err(format!("{} is not a valid Animation", s)),
        }
    }
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
            "Seconds: {:.2}, Animation: {}, RNG state: [{:08x}, {:08x}, {:08x}, {:08x}]",
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

impl Timeline {
    pub fn get_animations(&self) -> Vec<Animation> {
        self.0
            .iter()
            .map(|animation_time| animation_time.animation)
            .collect()
    }
}

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

fn handle_fidget(rng: &mut Xorshift, previous_fidget: Fidget) -> Fidget {
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

pub fn create_timeline(
    rng: &mut Xorshift,
    duration: u32,
    offset: u32,
    include_all_rng_calls: bool,
) -> Timeline {
    let mut result = vec![];
    let last_frame = seconds_to_frames(u32_to_seconds(duration));
    let mut current_fidget = Fidget::Idle;
    let mut next_fidget_frame = current_fidget.get_frames();
    let mut next_blink_frame = Blink::get_frames();

    for current_frame in offset..=last_frame {
        // A fidget rng call is made in 5 seconds
        // if the character is idling and 0.2 seconds
        if current_frame == next_fidget_frame {
            current_fidget = handle_fidget(rng, current_fidget);
            next_fidget_frame += current_fidget.get_frames();

            if current_fidget != Fidget::Idle || include_all_rng_calls {
                result.push(AnimationTime {
                    animation: Animation::Fidget(current_fidget),
                    seconds: frames_to_seconds(current_frame),
                    rng_state: rng.get_state(),
                });
            }
        }

        if current_frame == next_blink_frame {
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
                    seconds: frames_to_seconds(current_frame),
                    animation: Animation::Blink(blink),
                    rng_state: rng.get_state(),
                });
            }

            next_blink_frame += Blink::get_frames();
        }
    }

    Timeline(result)
}
