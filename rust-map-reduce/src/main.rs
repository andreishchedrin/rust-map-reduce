use std::{
    thread,
    error::Error,
    fs::File,
};

// #[feature(core_intrinsics)]
// fn print_type_of<T>(_: &T) {
//     println!("{}", unsafe { std::intrinsics::type_name::<T>() });
// }

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

const PATH: &str = "../Fifa22.csv";

fn main() -> Result<(), Box<dyn Error>> {
    let mut data: Vec<Player> = Vec::new();

    let file = File::open(PATH)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", record);
        // print_type_of(&record);

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

        let player = map_player(record);
        data.push(thread::spawn(move || -> Player {
            player
        }));
        // break;
    }

    let final_result = data.into_iter();

    println!("Final first result: {}", final_result[1]);
    println!("Final result len: {}", final_result.len());

    Ok(())
}
