pub struct Forge {
    pub name: String,
}

impl Forge {
    pub fn new(name: &str) -> Self {
        Forge {
            name: name.to_string(),
        }
    }

    pub fn run(&self) {
        println!("{} is now forging...", self.name);
    }
}

pub fn initialize() {
    println!("Initializing Sulaymaniyah Forge system...");
}
