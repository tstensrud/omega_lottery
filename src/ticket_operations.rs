use rand::distributions::{Distribution, Uniform};

// generates and returns a sorted row with random numbers from 1 to range. 8 numbers per row
pub fn new_row(range: u8) -> [u8;8] {
    let mut row: [u8; 8] = [0;8];
    let mut rng = rand::thread_rng();
    let balls = Uniform::from(1..=range);
    
    for i in 0..row.len() {
        let mut ball = balls.sample(&mut rng);
        while row.contains(&ball) {
            ball = balls.sample(&mut rng);
        }
        row[i] = ball;
    }
    row.sort();
    return row;
}