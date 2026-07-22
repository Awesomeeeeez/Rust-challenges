pub struct Pet {
    name: String,
    energy: u32,
    hunger: u32,
}
// методы
impl Pet {
    pub fn new(name: String) -> Self {
        Self {
            name,
            energy: 100,
            hunger: 0,
        }
    }

    pub fn feed(&mut self) -> &mut Self {
        if self.hunger < 30 {
            self.hunger = 0;
        } else {
            self.hunger -= 30;
        }
        println!("{} enjoyed the meal!", self.name);
        println!("--- After Activities ---");
        self
    }

    pub fn play(&mut self) -> &mut Self {
        if self.energy < 20 {
            self.energy = 0;
        } else {
            self.energy -= 20;
        }

        if self.hunger > 85 {
            self.hunger = 100;
        } else {
            self.hunger += 15;
        }
        println!("{} had fun playing!", self.name);
        self
    }
}

// геттеры
impl Pet {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn hunger(&self) -> u32 {
        self.hunger
    }

    pub fn energy(&self) -> u32 {
        self.energy
    }

    pub fn status(&mut self) -> &mut Self {
        print!(
            "--- Status Report ---\nName: {}\nEnergy: {}\nHunger: {}\nMood: {}\n",
            self.name,
            self.energy,
            self.hunger,
            if self.energy >= 50 && self.hunger <= 50 {"happy".to_string()} else {"tired".to_string()}
        );
        self
    }
}

// сеттеры
impl Pet {
    pub fn set_hunger(&mut self, hunger: u32) -> &mut Self {
        self.hunger = hunger;
        self
    }

    pub fn set_energy(&mut self, energy: u32) -> &mut Self {
        self.energy = energy;
        self
    }
}