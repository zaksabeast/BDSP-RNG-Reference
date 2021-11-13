use crate::xorshift::Xorshift;

fn check_is_shiny_u32(param_1: u32, param_2: u32) -> bool {
    (param_1 & 0xfff0 ^ param_1 >> 0x10 ^ param_2 >> 0x10 ^ param_2 & 0xfff0) < 0x10
}

fn is_shiny_path(mut rng: Xorshift, delay: usize) -> bool {
    rng.advance(delay);

    let pid = rng.next_range(0x80000000, 0xffffffff);
    let shiny_rand = rng.next_range(0x80000000, 0xffffffff);

    check_is_shiny_u32(pid, shiny_rand)
}

pub fn find_shiny_pokemon(rng: &mut Xorshift, delay: usize) -> usize {
    let mut is_shiny;
    let mut advances = 0;

    loop {
        is_shiny = is_shiny_path(*rng, delay);

        if is_shiny {
            break;
        }

        rng.next_range(0x80000000, 0xffffffff);
        advances += 1;
    }

    advances
}
