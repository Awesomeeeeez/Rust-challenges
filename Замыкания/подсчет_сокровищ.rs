#[derive(Debug)]
struct Location {
    name: String,
    treasures: u32,
}

#[derive(Debug)]
struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location)
    {
        let last_index = self.locations.len() - 1;
        let mut current_index = 0;

        while current_index <= last_index {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}

fn main() {
    let locations = [
        Location {
            name: "Enchanted Forest".to_string(),
            treasures: 5,
        },
        Location {
            name: "Mystic Mountain".to_string(),
            treasures: 10,
        },
    ];

    let map = Map { locations: &locations };

    // Подсчёт общего количества сокровищ
    let mut total_treasures = 0;

    map.explore(|location| {
        total_treasures += location.treasures;
    });

    println!("Total treasures: {}", total_treasures); // 15
}