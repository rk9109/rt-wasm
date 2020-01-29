use std::env;
use std::f32;
use std::time;

use getopts::Options;
use rand::SeedableRng;

use rtwasm::cast;
use rtwasm::scenes;

fn parse_args() -> Option<scenes::Params> {
    // parse command-line arguments
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "");
    opts.optopt("x", "", "x resolution", "INT");
    opts.optopt("y", "", "y resolution", "INT");
    opts.optopt("s", "samples", "samples per pixel", "INT");
    opts.optopt("r", "random", "random seed for RNG", "INT");
    opts.optopt("o", "output", "output filename", "FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(error) => {
            println!("{}", error);
            return None;
        }
    };

    // process --help option
    if matches.opt_present("h") {
        let message = format!("Usage: {} [options]", args[0]);
        println!("{}", opts.usage(&message));
        return None;
    }

    // initialize default values
    let mut nx = 200;
    let mut ny = 100;
    let mut ns = 100;
    let mut random_seed = 0;
    let mut output = String::from("output.png");

    // parse optional options
    if matches.opt_present("x") {
        nx = matches.opt_str("x").unwrap().parse().unwrap();
    }
    if matches.opt_present("y") {
        ny = matches.opt_str("y").unwrap().parse().unwrap();
    }
    if matches.opt_present("s") {
        ns = matches.opt_str("s").unwrap().parse().unwrap();
    }
    if matches.opt_present("r") {
        random_seed = matches.opt_str("r").unwrap().parse().unwrap();
    }
    if matches.opt_present("o") {
        output = matches.opt_str("o").unwrap();
    }

    Some(scenes::Params::new(nx, ny, ns, random_seed, output))
}

fn main() {
    // parse command-line arguments
    let params = match parse_args() {
        Some(params) => params,
        None => return,
    };

    // initialize rng
    let mut rng = rand_pcg::Pcg64::seed_from_u64(params.random_seed);

    // initialize world and camera
    let (world, cam) = scenes::custom_scene(params.nx, params.ny);

    // initialize timer
    let start = time::Instant::now();

    cast(&params, &world, &cam, &mut rng, true, true);

    // print elapsed time
    let end = time::Instant::now();
    let time_secs = (end - start).as_secs();
    let time_millis = (end - start).subsec_millis();
    println!(
        "elapsed time: {}",
        time_secs as f32 + time_millis as f32 / 1000.0
    );
}
