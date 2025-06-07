use std::collections::HashMap;


fn main() {

}


enum BuildingType {
    Habitat,
    PowerPlant,
    WaterPurifier,
}

struct Building {
    building_type: BuildingType,
    name: String,
    id: u32,
    func_val: i32,
}

impl Building {
    fn new(building_type: BuildingType, name: &str, id: u32, func_val: i32) -> Building {
        Building {
            building_type,
            name: name.to_string(),
            id,
            func_val
        }
    }

    fn print(&self) {
        match self.building_type {
            BuildingType::Habitat => {
                println!("- [Habitat] {} (ID: {}), Function value: {}", self.name, self.id, self.func_val);
            }
            BuildingType::PowerPlant => {
                println!("- [Power Plant] {} (ID: {}), Function value: {}", self.name, self.id, self.func_val);
            }
            BuildingType::WaterPurifier => {
                println!("- [Water Purifier] {} (ID: {}), Function value: {}", self.name, self.id, self.func_val)
            }
        }
    }

    fn print_list(buildings: &[Building]) {
        println!("Buildings:");
        for building in buildings {
            building.print();
        }
    }
}

enum Profession {
    Scientist,
    Engineer,
    Farmer,
}

struct Colonist {
    name: String,
    profession: Profession,
    health_status: u32,
    is_alive: bool,
}

enum ResourceType {
    Electricity,
    Water,
    Food,
    Oxygen,
}

struct Colony {
    name: String,
    resources: HashMap<ResourceType, u32>,
    colonists: Vec<Colonist>,
    buildings: Vec<Building>,
}


enum Event {
    MeteorShower,
    PowerOutage,
}
