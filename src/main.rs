mod random;

use random::Random;

struct VehicleRegistry {
}

impl VehicleRegistry {
    fn generate_vehicle_id(self, length: u8) -> String {
	Random::rand_alpha(12)
    }

    // fn generate_vehicle_license(self, id: String) -> String {
    // 	fn get_() {
	    
    // 	} 
    // }
}


fn main() {
    println!("{}", Random::rand_alpha(2));
    println!("{}", Random::rand_numeric(2));
}
