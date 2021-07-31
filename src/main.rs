mod random;

use random::Random;

struct VehicleRegistry {}

impl VehicleRegistry {
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

	// compute the catalogue price
	let mut catalogue_price: i32 = 0;
	if brand == "Tesla Model 3" {
	    catalogue_price = 60000;
	} else if brand == "Volkswagen ID3" {
	    catalogue_price = 35000;
	} else if brand == "BMW 5" {
	    catalogue_price = 45000;
	}
	
	// compute the tax percentage
	// default is 5%
	// 2% if electric
	let mut tax_percentage = 0.05;
	if brand == "Tesla Model 3" || brand == "Volkswagen ID3" {
	    tax_percentage = 0.02
	}

	// compute the payable tax
	let payable_tax = tax_percentage * catalogue_price as f32;
	
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
