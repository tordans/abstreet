// Copyright 2018 Google LLC, licensed under http://www.apache.org/licenses/LICENSE-2.0

extern crate abstutil;
extern crate control;
extern crate map_model;
extern crate sim;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "headless")]
struct Flags {
    /// Map or savestate to load
    #[structopt(name = "load")]
    load: String,

    /// Optional RNG seed
    #[structopt(long = "rng_seed")]
    rng_seed: Option<u8>,

    /// Optional time to savestate
    #[structopt(long = "save_at")]
    save_at: Option<String>,

    /// Big or large random scenario?
    #[structopt(long = "big_sim")]
    big_sim: bool,

    /// Scenario name for savestating
    #[structopt(long = "scenario_name", default_value = "editor")]
    scenario_name: String,
}

fn main() {
    let flags = Flags::from_args();

    let (map, _, control_map, mut sim) = sim::init::load(
        flags.load,
        flags.scenario_name,
        flags.rng_seed,
        Some(sim::Tick::from_seconds(30)),
    );

    if sim.time == sim::Tick::zero() {
        // TODO need a notion of scenarios
        if flags.big_sim {
            sim::init::big_spawn(&mut sim, &map);
        } else {
            sim::init::small_spawn(&mut sim, &map);
        }
    }

    let save_at = if let Some(ref time_str) = flags.save_at {
        if let Some(t) = sim::Tick::parse(time_str) {
            Some(t)
        } else {
            panic!("Couldn't parse time {}", time_str);
        }
    } else {
        None
    };

    let mut benchmark = sim.start_benchmark();
    loop {
        sim.step(&map, &control_map);
        if sim.time.is_multiple_of(sim::Tick::from_seconds(60)) {
            let speed = sim.measure_speed(&mut benchmark);
            println!("{0}, speed = {1:.2}x", sim.summary(), speed);
        }
        if Some(sim.time) == save_at {
            sim.save();
        }
    }
}
