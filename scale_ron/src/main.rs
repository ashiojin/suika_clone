use clap::Parser;

use std::io;
use std::io::Read;

use game_ron::*;
use ron::ser::PrettyConfig;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cmd {
    scale: f32,

    #[arg(long)]
    balls: bool,

    #[arg(long)]
    bottle: bool,

    #[arg(short, long)]
    all: bool,

}

fn main() {
    let args = Cmd::parse();

    let mut buf: String = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("Failed to read stdio");

    let mut ron: GameRon = ron::from_str(&buf)
        .expect("");


    if args.balls || args.all {
        for s in ron.balls.iter_mut() {
            s.physics_radius *= args.scale;
            s.view_width *= args.scale;
            s.view_height *= args.scale;
        }
    }

    if args.bottle || args.all {
        let bottle = &mut ron.bottle;

        bottle.inner_width *= args.scale;
        bottle.inner_height *= args.scale;
        bottle.thickness *= args.scale;
    }


    let out = ron::ser::to_string_pretty(&ron, PrettyConfig::default())
        .expect("Failed to write stdout");

    print!("{}", out);
}
