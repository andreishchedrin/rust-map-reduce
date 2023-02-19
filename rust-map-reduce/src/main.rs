
struct Player {
    name: String,
    rating: u8,
    position: String,
    nationality: String,
    link: String,
    stats: Stats
}

struct Stats {
    pace: u8,
    shoot: u8,
    pass: u8,
    dribbling: u8,
    defend: u8,
    physics: u8,
}

fn main() {
    let mut data: Vec<Player> = Vec::new();
}
