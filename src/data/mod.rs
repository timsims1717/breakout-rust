mod stage;

use std::{
    fs, str
};

pub use self::stage::{
    STAGE_HEIGHT, STAGE_WIDTH, Stage
};

pub fn load(path: String) -> Stage {
    use amethyst::utils::application_root_dir;

    // load file
    let filename = format!("{}/stages/{}", application_root_dir(), path);
    let contents = fs::read_to_string(filename).unwrap();
    let rows: Vec<&str> = contents.split('\n').collect();
    let mut stage = Vec::with_capacity(STAGE_HEIGHT);
    for row in rows.iter() {
        let mut r = Vec::with_capacity(STAGE_WIDTH);
        let bs: Vec<&str> = row.split(' ').collect();
        for b in bs.iter() {
            r.push(b.parse::<usize>().unwrap());
        }
        stage.push(r)
    }
    Stage::new(stage)
}