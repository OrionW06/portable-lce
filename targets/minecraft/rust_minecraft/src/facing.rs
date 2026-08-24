pub const DOWN: usize = 0;
pub const UP: usize = 1;
pub const NORTH: usize = 2;
pub const SOUTH: usize = 3;
pub const WEST: usize = 4;
pub const EAST: usize = 5;

pub const OPPOSITE_FACING: [usize; 6] = [1, 0, 3, 2, 5, 4];
pub const STEP_X: [i32; 6] = [0, 0, 0, 0, -1, 1];
pub const STEP_Y: [i32; 6] = [-1, 1, 0, 0, 0, 0];
pub const STEP_Z: [i32; 6] = [0, 0, -1, 1, 0, 0];

pub const NAMES: [&str; 6] = ["DOWN", "UP", "NORTH", "SOUTH", "WEST", "EAST"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facing() {
        assert_eq!(OPPOSITE_FACING[DOWN], UP);
        assert_eq!(OPPOSITE_FACING[UP], DOWN);
        assert_eq!(OPPOSITE_FACING[NORTH], SOUTH);
        assert_eq!(OPPOSITE_FACING[WEST], EAST);
        assert_eq!(STEP_Y[UP], 1);
        assert_eq!(STEP_Y[DOWN], -1);
        assert_eq!(STEP_X[WEST], -1);
        assert_eq!(STEP_X[EAST], 1);
    }
}
