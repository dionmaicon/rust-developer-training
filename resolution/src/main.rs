
enum CarManufacturing {
    Fiat, Mercedes, Chevrolet
}

fn car_country( car_manufacturing:CarManufacturing ) {
    match car_manufacturing {
        CarManufacturing::Fiat => println!("Italian"),
        CarManufacturing::Mercedes => println!("German"),
        CarManufacturing::Chevrolet => println!("American"),
    }
}
fn main() {

    let chev:CarManufacturing = CarManufacturing::Chevrolet;
    let fiat:CarManufacturing = CarManufacturing::Fiat;
    let mercedes:CarManufacturing = CarManufacturing::Mercedes;
    
    car_country(chev);
    car_country(fiat);
    car_country(mercedes);
}
