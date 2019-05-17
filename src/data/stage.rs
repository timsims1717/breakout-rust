

pub const STAGE_HEIGHT: usize = 20;
pub const STAGE_WIDTH: usize = 20;

pub struct Stage {
    pub bricks: Vec<Vec<usize>>,
}

impl Stage {
    pub fn new(b: Vec<Vec<usize>>) -> Stage {
        assert_eq!(b.len(), STAGE_HEIGHT);
        for r in &b {
            assert_eq!(r.len(), STAGE_WIDTH);
        }
        return Stage{
            bricks: b
        }
    }
}