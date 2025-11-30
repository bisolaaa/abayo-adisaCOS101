use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Define the drink categories and their items
    let lager = vec![
        "33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star",
    ];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // Create or overwrite the output file
    let mut file = File::create("nigerian_breweries_drinks.txt")?;

    // Write header
    writeln!(file, "Nigerian Breweries Plc - Drink Categories\n")?;

    // Write Lager section
    writeln!(file, "Lager:")?;
    for drink in &lager {
        writeln!(file, "- {}", drink)?;
    }

    // Write Stout section
    writeln!(file, "\nStout:")?;
    for drink in &stout {
        writeln!(file, "- {}", drink)?;
    }

    // Write Non-Alcoholic section
    writeln!(file, "\nNon-Alcoholic:")?;
    for drink in &non_alcoholic {
        writeln!(file, "- {}", drink)?;
    }

    println!("Drink categories saved to 'nigerian_breweries_drinks.txt'");
    Ok(())
}
