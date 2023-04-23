// An attribute to hide warnings for unused code.
#[allow(dead_code)]

use std::{env, ffi::OsString,fs::File, error::Error, io,path, process};
use random_number::random; 
use serde::Deserialize; 
use rand::thread_rng;
use rand::seq::SliceRandom;

//Remember, each of these attributes has to match the csv header fields
#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    phone: String, 
    email: String, 
    interest: String, 
    party_size: u8,
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn findCampers() -> Result<(), Box<dyn Error>>{
    let file_path = get_first_arg()?;
    //let file = File::open(file_path)?;
    //let mut rdr = csv::Reader::from_reader(file);
    let mut rdr = csv::Reader::from_path(file_path)?;
    //for result in rdr.deserialize() {
    //    let camper: Person = result?;
    //    println!("{:?}", camper);
    //}
    let mut campers_list: Vec<Person> = rdr.deserialize::<Person>().collect::<Result<_, _>>()?;
        // Shuffle vector of people
    campers_list.shuffle(&mut thread_rng());

    // Set maximum occupancy size
    let max_occupancy = 30;

    // Initialize variables for tracking occupancy and selected people
    let mut occupancy = 0;
    let mut selected_people: Vec<Person> = Vec::new();
    let mut rejected_people: Vec<Person> = Vec::new();

    // Randomly select people until maximum occupancy is reached
    while occupancy < max_occupancy {
        let person = campers_list.pop().unwrap(); // Pop off the last person in the shuffled vector
        let mut selflag = false; 
        let party_size = person.party_size;
        if person.interest == "Very Interested" && occupancy + party_size <= max_occupancy {
                selected_people.push(person); // Add person to selected people vector
                occupancy += party_size; // Increase occupancy by person's party size
                selflag = true; 
        }
        else{
            rejected_people.push(person); 
        }

    }

    // Print selected people and their party sizes
    println!("Selected campers: "); 
    for person in selected_people {
        println!("{}, {}, {}", person.name, person.email, person.party_size);
    }
    println!("Wait-listed campers: "); 
    for person in rejected_people{
        println!("{}, {}, {}", person.name, person.email, person.party_size); 
    }
    
    //for person in &selected_people {
    //    print!("{}", person.email);
    //}
    Ok(())    

}


fn main() {
    if let Err(err) = findCampers(){
        println!("error running findingCampers: {}", err); 
        process::exit(1); 

    }
    //let path = Path::new("campersAll.csv");
    //let mut file = File::open(&path); 
    

/*
    
    let mut n: u8 = random!(0, 8); 
    let party_size = n; 
    let name = String::from("Xisen"); 
    let email = String::from("blah@gmail.com"); 
    let i:u8 = 1; 
    let xisen = Person{name, party_size, interest:i, email}; 

    let occupancy = 30; 
    let mut totalCampers = 0; 
    while totalCampers <= occupancy{
        n = random!(0, 8); 
        println!("total campers {}", totalCampers);  
        totalCampers += n; 
    }
*/ 
}
