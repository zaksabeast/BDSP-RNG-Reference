mod shiny;
mod timeline;
mod utils;
mod xorshift;

use structopt::StructOpt;
use xorshift::Xorshift;

/// Basic BDSP rng cli
#[derive(StructOpt, Debug)]
#[structopt(name = "bdsp-rng")]
struct Opt {
    /// state[0] for the rng
    #[structopt(long = "s0", parse(try_from_str = utils::parse_hex))]
    state0: u32,

    /// state[1] for the rng
    #[structopt(long = "s1", parse(try_from_str = utils::parse_hex))]
    state1: u32,

    /// state[2] for the rng
    #[structopt(long = "s2", parse(try_from_str = utils::parse_hex))]
    state2: u32,

    /// state[3] for the rng
    #[structopt(long = "s3", parse(try_from_str = utils::parse_hex))]
    state3: u32,

    /// Number of rng advances before any operation
    #[structopt(short, long)]
    offset: Option<usize>,

    #[structopt(subcommand)]
    subcommands: Option<SubCommand>,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Advance the rng until a shiny Pokemon is found
    FindShiny {
        /// Number of rng advances between the time the user takes an action and a Pokemon is generated
        #[structopt(short, long, default_value = "0")]
        delay: usize,
    },
    /// Advance the rng until the provided states are found
    FindState {
        /// state[0] for the rng
        #[structopt(long = "s0", parse(try_from_str = utils::parse_hex))]
        new_state0: u32,

        /// state[1] for the rng
        #[structopt(long = "s1", parse(try_from_str = utils::parse_hex))]
        new_state1: u32,

        /// state[2] for the rng
        #[structopt(long = "s2", parse(try_from_str = utils::parse_hex))]
        new_state2: u32,

        /// state[3] for the rng
        #[structopt(long = "s3", parse(try_from_str = utils::parse_hex))]
        new_state3: u32,
    },
    /// Shows the character animation timeline, assuming no nearby npcs
    Timeline {
        /// Number of seconds the timeline should last
        #[structopt(short, long)]
        duration: u32,
        /// Frame offset
        #[structopt(short, long)]
        offset: u32,
        /// Include all rng calls, including no blinks and idle fidgets
        #[structopt(short, long)]
        include_all: bool,
    },
    /// Finds a timline, assuming no nearby npcs
    FindTimeline {
        /// Number of seconds the timeline should last
        #[structopt(short, long)]
        duration: u32,
        /// Frame offset
        #[structopt(short, long)]
        offset: u32,
        /// The lower advance limit to search
        #[structopt(long)]
        min: usize,
        /// The upper advance limit to search
        #[structopt(long)]
        max: usize,
        /// The timeline to find
        #[structopt(short, long)]
        timeline: Vec<timeline::Animation>,
    },
}

fn main() {
    let opt = Opt::from_args();

    let offset = opt.offset.unwrap_or_default();
    let mut rng = Xorshift::from_state([opt.state0, opt.state1, opt.state2, opt.state3]);

    rng.advance(offset);

    match opt.subcommands {
        Some(SubCommand::FindShiny { delay }) => {
            let advances = shiny::find_shiny_pokemon(&mut rng, delay);
            println!("Shiny in {} advances", advances + offset);
        }
        Some(SubCommand::FindState {
            new_state0,
            new_state1,
            new_state2,
            new_state3,
        }) => match rng.advance_to_state([new_state0, new_state1, new_state2, new_state3]) {
            Some(advances) => println!("Found states in {} advances", advances + offset),
            None => println!("Could not find provided states"),
        },
        Some(SubCommand::Timeline {
            duration,
            offset,
            include_all,
        }) => {
            let animation_timeline =
                timeline::create_timeline(&mut rng, duration, offset, include_all);
            println!("Animation timeline:\n{}", animation_timeline);
        }
        Some(SubCommand::FindTimeline {
            duration,
            offset,
            min,
            max,
            timeline,
        }) => {
            rng.advance(min);

            for advance in min..max {
                let mut copied_rng = rng;
                let animations =
                    timeline::create_timeline(&mut copied_rng, duration, offset, false)
                        .get_animations();

                let rng_state = copied_rng.get_state();
                if animations == timeline {
                    println!(
                        "Found timeline at advance {} with state {:x} {:x} {:x} {:x}",
                        advance, rng_state[0], rng_state[1], rng_state[2], rng_state[3]
                    );
                }

                rng.next();
            }
        }
        None => { /* no-op */ }
    }

    let rng_state = rng.get_state();
    println!(
        "RNG states: {:x} {:x} {:x} {:x}",
        rng_state[0], rng_state[1], rng_state[2], rng_state[3]
    );
}
