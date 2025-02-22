use std::io;
use std::fs::OpenOptions;
use std::io::Write;
fn main() {
    // INTRO
    println!("\n*******     CARBON FOOTPRINT     *******\n");
    println!("Hello. This is Computer Science Stream 2 Group 7. We are going to calculate the carbon footprint of your laptops and phones.\n");
    println!("The formula for carbon footprint of mobile devices is \nCm + (E_usage X years of use), where:");
    println!("Cm => Carbon emission during manufacturing of the device,");
    println!("E_usage => Energy consumed when using your device per day\n");

    // VECTORS AND VARIABLES
    let choices = vec!['M', 'H', 'A', 'D', 'L', 'C', 'I', 'S', 'F', 'G', 'X'];
    let eh = vec![15, 40, 30, 30, 50, 30, 6, 6, 5, 5, 6];
    let cm = vec![170, 300, 400, 400, 250, 350, 50, 60, 40, 40, 70];
    let brand_n = vec!["Macbook", "HP", "Asus", "Dell", "Lenovo", "Acer", "iPhone", "Samsung", "Infinix", "Google Pixel", "Xiaomi"];

    // QUESTIONS
    println!("We are going to ask some questions to determine the footprint of your device.\n");
    println!("This is a list of laptop brands to choose from - [Macbook, HP, Asus, Dell, Lenovo, Acer, Xiaomi]");
    println!("Type:\n M/m for Macbook\n H/h for HP\n A/a for Asus\n D/d for Dell\n L/l for lenovo\n C/c for Acer\n I/i for iPhone\n S/s for Samsung\n I/i for Infinix\n G/g for Google Pixel\n X/x for Xiaomi");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("FAILED TO READ INPUT");
    let choice: char = choice.trim().parse().expect("FAILED TO PARSE INPUT");

    // TOTAL EMMISSION
    if choices.contains(&choice) {
        //CHECKING THE ENERGY USAGE
        println!("\nHow many hours do you spend on your device every day (use an estimated whole number)?");
        let mut hrs = String::new();
        io::stdin().read_line(&mut hrs).expect("FAILED TO READ INPUT");
        let hrs: u32 = hrs.trim().parse().expect("FAILED TO PARSE INPUT");

        println!("\nHow many years have you been using your device? If it’s less than a year, please enter 0.");
        let mut years = String::new();
        io::stdin().read_line(&mut years).expect("FAILED TO READ INPUT");
        let years: u32 = years.trim().parse().expect("FAILED TO PARSE INPUT");

        carbon_f(choice,choices, eh, cm, years, brand_n, hrs);
    }
    else {
        println!("Sorry, your device is not part of the list.");
    }
}

// FUNCTION FOR CARBON FOOTPRINT
fn carbon_f(choice:char, brands:Vec<char>, eh:Vec<u32>, cm:Vec<u32>, years:u32, brand_n:Vec<&str>, hours:u32){
    let mut index = 0;
    let energy_usage;
    let total_e;
    
    // getting index
    for i in 0..=10{
        if choice == brands[i]{
            index = i;
            break;
        }
    }

    // caluclating energy consumed and total emmission
    if years > 0 {
        energy_usage = eh[index] * hours * 365 * years;
        total_e = energy_usage + cm[index];
    }
    else if years == 0 {
        println!("\nHow many days have you been using your device?");
        let mut days = String::new();
        io::stdin().read_line(&mut days).expect("FAILED TO READ INPUT");
        let days: u32 = days.trim().parse().expect("FAILED TO PARSE INPUT");
        
        if days<1{
            println!("The number of days cannot be less than one.");
            return;
        }
        energy_usage = eh[index] * hours * days;
        total_e = energy_usage + cm[index];
    }
    else {
        println!("The number of years cannot be negative.");
        return;
    }
    
    println!("\nFrom the information you have inputed:");
    println!("Brand: {}", brand_n[index]);
    println!("Carbon Emission during manufacturing: {} kgCO2e", cm[index]);
    println!("Total energy consumption: {} Wh", energy_usage);
    println!("Total Carbon Footprint: {} kgCO2e", total_e);
    println!("\nA document containing this information has been saved in the project folder. Thank you!");
    
    // CREATING A TEXT FILE FOR THE RESULT
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("/Users/chiomadande/Documents/PersonalRust/carbonf/table.txt")
    .expect("cannot open file");
    file.write_all("\n++++++++++++     CARBON FOOTPRINT     ++++++++++++\n".as_bytes()).expect("write failed");

    writeln!(file, "Brand of device: {:>33}", brand_n[index]).expect("failed to write headers");
    writeln!(file, "Carbon Emission during manufacturing: {:>5} kgCO2e", cm[index]).expect("failed to write headers");
    writeln!(file, "Total energy consumed by device: {:>14} Wh", energy_usage).expect("failed to write headers");
    writeln!(file, "Total Carbon Footprint: {:>19} kgCO2e\n\n", total_e).expect("failed to write headers");
}
