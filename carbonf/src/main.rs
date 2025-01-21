use std::io;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // INTRO
    println!("\n*******CARBON FOOTPRINT*******\n");
    println!("Hello. This is Computer Science Stream 2 Group 7. We are going to calculate the carbon footprint of your laptops.\n");
    println!("The formula for carbon footprint of laptops is \nCm + (E_usage X years of use), where:");
    println!("Cm => Carbon emission during manufacturing,");
    println!("E_usage => Energy used per day\n");

    // VECTORS AND VARIABLES
    let choices = vec!['M', 'H', 'A', 'D', 'L', 'C'];
    let eh = vec![15, 40, 30, 30, 50, 30];
    let cm = vec![170, 300, 400, 400, 250, 350];
    let brand_n = vec!["Macbook", "HP", "Asus", "Dell", "Lenovo", "Acer"];

    // QUESTIONS
    println!("We are going to ask some questions to determine the footprint of your laptop.\n");
    println!("This is a list of laptop brands to choose from - [Macbook, HP, Asus, Dell, Lenovo, Acer]");
    println!("Type:\n M for Macbook\n H for HP\n A for Asus\n D for Dell\n L for Lenovo\n C for Acer");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("FAILED TO READ INPUT");
    let choice: char = choice.trim().to_uppercase().parse().expect("FAILED TO PARSE INPUT");

    // TOTAL EMISSION
    if let Some(index) = choices.iter().position(|&c| c == choice) {
        println!("\nHow many years have you been using your laptop? If it’s less than a year, please enter 0.");
        let mut years = String::new();
        io::stdin().read_line(&mut years).expect("FAILED TO READ INPUT");
        let years: u32 = years.trim().parse().expect("FAILED TO PARSE INPUT");

        carbon_f(index, eh, cm, years, brand_n);
    } else {
        println!("Sorry, your laptop is not part of the list.");
    }
}

// FUNCTION FOR CARBON FOOTPRINT
fn carbon_f(index: usize, eh: Vec<u32>, cm: Vec<u32>, years: u32, brand_n: Vec<&str>) {
    let energy_usage;
    let total_e;

    if years > 0 {
        energy_usage = eh[index] * 24 * 365 * years;
        total_e = energy_usage + cm[index];
    } else {
        println!("How many days have you been using your laptop?");
        let mut days = String::new();
        io::stdin().read_line(&mut days).expect("FAILED TO READ INPUT");
        let days: u32 = days.trim().parse().expect("FAILED TO PARSE INPUT");

        if days < 1 {
            println!("The number of days cannot be less than one.");
            return;
        }
        energy_usage = eh[index] * 24 * days;
        total_e = energy_usage + cm[index];
    }

    println!("\nFrom the information you have inputted:");
    println!("Brand: {}", brand_n[index]);
    println!("Carbon Emission during manufacturing: {} kgCO2e", cm[index]);
    println!("Total energy consumption: {} Wh", energy_usage);
    println!("Total Carbon Footprint: {} kgCO2e", total_e);
    println!("\nA document containing this information has been saved in the project folder. Thank you!");

    // CREATING A TEXT FILE FOR THE RESULT
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/Users/chiomadande/Documents/PersonalRust/table.txt")
        .expect("cannot open file");
    file.write_all("\n++++++++++++     CARBON FOOTPRINT     ++++++++++++\n".as_bytes())
        .expect("write failed");

    writeln!(file, "Brand of Laptop: {:>33}", brand_n[index]).expect("failed to write headers");
    writeln!(file, "Carbon Emission during manufacturing: {:>5} kgCO2e", cm[index]).expect("failed to write headers");
    writeln!(file, "Total energy consumed by laptop: {:>14} Wh", energy_usage).expect("failed to write headers");
    writeln!(file, "Total Carbon Footprint: {:>19} kgCO2e\n\n", total_e).expect("failed to write headers");
}
