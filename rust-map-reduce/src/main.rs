use std::{
    thread,
    error::Error,
    fs::File,
    time::Instant,
};

#[derive(Debug)]
struct Player {
    name: String,
    rating: u8,
    position: String,
    nationality: String,
    link: String,
    stats: Stats
}

#[derive(Debug)]
struct Stats {
    pace: u8,
    shoot: u8,
    pass: u8,
    dribbling: u8,
    defend: u8,
    physics: u8,
}

const PATH: &str = "../Fifa22.csv";

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mut data = vec![];

    let file = File::open(PATH)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;

        let map_player = |player: csv::StringRecord| -> Player {
            let result = Player{
                name: (&player[0]).to_string(),
                rating: (&player[1]).to_string().parse().unwrap(),
                position: (&player[2]).to_string(),
                nationality: (&player[4]).to_string(),
                link: (&player[11]).to_string(),
                stats: Stats {
                    pace: (&player[5]).to_string().parse().unwrap(),
                    shoot: (&player[6]).to_string().parse().unwrap(),
                    pass: (&player[7]).to_string().parse().unwrap(),
                    dribbling: (&player[8]).to_string().parse().unwrap(),
                    defend: (&player[9]).to_string().parse().unwrap(),
                    physics: (&player[10]).to_string().parse().unwrap()
                }
            };
            result
        };

        data.push(thread::spawn(move || {
            let player = map_player(record);
            player
        }));
    }

    let final_result = data.into_iter().map(|c| c.join().unwrap()).collect::<Vec<_>>();

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("Final first result: {:?}", final_result[1]);
    println!("Final result len: {:?}", final_result.len());

    Ok(())
}
