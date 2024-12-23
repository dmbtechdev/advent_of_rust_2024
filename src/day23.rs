use std::collections::HashMap;
#[derive(Default)]
pub struct SantaList {
    // 1. Define the records field
    // to store children’s names as keys and their behaviors (true for nice, false for naughty) as values.
    records: HashMap<String, bool>,
}

impl SantaList {
    // 2. Implement the new method
    pub fn new() -> Self {
        // Self { records: HashMap::new() }
        Default::default()
    }

    // 3. Implement the add method
    pub fn add(&mut self, name: &str, behavior: bool) {
        self.records.insert(name.to_string(), behavior);
    }

    // 4. Implement the remove method
    pub fn remove(&mut self, name: &str) {
        self.records.remove(name);
    }

    // 5. Implement the get method
    pub fn get(&self, name: &str) -> Option<bool> {
        self.records.get(name).cloned()
    }

    // 6. Implement the count method
    pub fn count(&self) -> (u32, u32) {
        let (mut nice,_) = (0,0);
        let total = self.records.values().len() as u32;
        nice = self.records.values().filter(|&&b| b).count() as u32;
        // self.records.values().filter(|&b| *b==true).count();
        // for (_, &behavior) in &self.records {
        //     if behavior {
        //         nice += 1;
        //     } else {
        //         naughty += 1;
        //     }
        // }
        (nice, total - nice)
    }

    // 7. Implement the list_by_behavior method
    pub fn list_by_behavior(&self, behavior: bool) -> Vec<String> {
        self.records
            .iter()
            .filter_map(|(k, &v)| (v == behavior).then_some(k.to_owned()))
            .collect()

            // .iter()
            // .filter(|(_, &b)| b == behavior)
            // .map(|(name, _)| name.clone())
            // .collect()

    }
}

pub fn main() {
    let mut santa_list = SantaList::new();

    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);

    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }

    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );

    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);

    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);

    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}
