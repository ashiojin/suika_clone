use std::env;
use std::io;
use std::io::Read;


use game_ron::*;
use ron::ser::PrettyConfig;

fn main() {
    let args: Vec<String> = env::args().collect();

    let scale: f32 = args[1].parse::<f32>().expect("Failed to read arg (`scale`)");

    let mut buf: String = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("Failed to read stdio");

    let mut ron: GameRon = ron::from_str(&buf)
        .expect("");


    for s in ron.balls.iter_mut() {
        s.physics_radius *= scale;
        s.view_width *= scale;
        s.view_height *= scale;
    }


    let out = ron::ser::to_string_pretty(&ron, PrettyConfig::default())
        .expect("Failed to write stdout");

    print!("{}", out);
}
