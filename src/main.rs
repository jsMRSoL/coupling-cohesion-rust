mod random;

use std::collections::HashMap;

use random::Random;

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

struct VehicleRegistry {
    vehicle_info: HashMap<String, VehicleInfo>
}

impl VehicleRegistry {
    fn new(self) -> Self {
	// FIXME!!
	let mut map: = HashMap::new();
	self.add_vehicle_info("Tesla Model 3", true, 60000);
	self.add_vehicle_info("Volkswagen ID 3", true, 35000);
	self.add_vehicle_info("BMW 5", false, 45000);
	self.add_vehicle_info("Tesla Model Y", true, 75000);
    }

    fn add_vehicle_info(&self, brand: String, electric: bool, catalogue_price: i32) {
	let v = VehicleInfo::new(brand, electric, catalogue_price);
	&self.vehicle_info.insert(brand, v);
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
}

struct Application {}
impl Application {
    fn register_vehicle(&self, brand: &str) {
	// create a registry instance
	let registry = VehicleRegistry{};
	// generate a vehicle id of length 12
	let vehicle_id: String = registry.generate_vehicle_id(12);
	// now generate a license plate
	let license_plate: String = registry.generate_vehicle_license(&vehicle_id);

	
	// print out the vehicle registration information
	println!("Registration complete. Vehicle info:");
	println!("Brand: {}", &brand);
	println!("Id: {}", &vehicle_id);
	println!("License plate: {}", &license_plate);
	println!("Payable tax: {}", payable_tax);
    }
}

fn main() {
    // println!("{}", Random::rand_alpha(2));
    // println!("{}", Random::rand_numeric(2));
    let app = Application{};
    app.register_vehicle("Volkswagen ID3");
    app.register_vehicle("Tesla Model 3");
    app.register_vehicle("BMW 5");
}
