mod random;

use std::collections::HashMap;

use random::Random;

#[derive(Debug, Clone)]
struct VehicleInfo {
    brand: String,
    electric: bool,
    catalogue_price: i32,
}

impl VehicleInfo {
    fn new(brand: String, electric: bool, catalogue_price: i32) -> Self {
	Self {
	    brand,
	    electric,
	    catalogue_price,
	}
    }

    fn compute_tax(&self) -> f32 {
	let mut tax_percentage = 0.05;
	if self.electric {
	    tax_percentage = 0.02
	}

	tax_percentage * self.catalogue_price as f32
    }

    fn print(&self) {
	println!("Brand: {}", &self.brand);
	println!("Payable tax: {}", &self.compute_tax());
    }
}

struct Vehicle {
    id: String,
    license_plate: String,
    info: VehicleInfo,
}

impl Vehicle {
    fn new(id: String, license_plate: String, info: VehicleInfo) -> Self {
	Self {
	    id,
	    license_plate,
	    info,
	}
    }

    fn print(&self) {
	println!("Id: {}", self.id);
	println!("License plate: {}", self.license_plate);
	self.info.print();
    }
}

struct VehicleRegistry {
    vehicle_info: HashMap<String, VehicleInfo>
}

impl VehicleRegistry {
    fn new() -> Self {
	let mut map = HashMap::new();
	VehicleRegistry::add_vehicle_info(&mut map, "Tesla Model 3", true, 60000);
	VehicleRegistry::add_vehicle_info(&mut map, "Volkswagen ID3", true, 35000);
	VehicleRegistry::add_vehicle_info(&mut map, "BMW 5", false, 45000);
	VehicleRegistry::add_vehicle_info(&mut map, "Tesla Model Y", true, 75000);
	Self {
	    vehicle_info: map,
	}
    }

    fn add_vehicle_info(map: &mut HashMap<String, VehicleInfo>, brand: &str, electric: bool, catalogue_price: i32) {
	let v = VehicleInfo::new(brand.to_string(), electric, catalogue_price);
	map.insert(brand.to_string(), v);
    }

    fn generate_vehicle_id(&self, length: u8) -> String {
        Random::rand_alpha(length)
    }

    fn generate_vehicle_license(&self, id: &str) -> String {
        format!(
            "{}-{}-{}",
            &id[..2],
            Random::rand_numeric(2),
            Random::rand_alpha(2)
        )
    }

    fn create_vehicle(&self, brand: &str) -> Vehicle {
	let id:String = self.generate_vehicle_id(12);
	let license_plate: String = self.generate_vehicle_license(&id);
	let info: VehicleInfo = self.vehicle_info.get(brand).unwrap().clone();
	return Vehicle::new(id, license_plate, info)
	}
}

struct Application {}

impl Application {
    fn register_vehicle(&self, brand: &str) {
	// create a registry instance
	let registry = VehicleRegistry::new();
	let vehicle = registry.create_vehicle(brand);
	vehicle.print();
    }
}

fn main() {
    let app = Application{};
    app.register_vehicle("Volkswagen ID3");
    app.register_vehicle("Tesla Model 3");
    app.register_vehicle("BMW 5");
}
