use std::io;

fn get_role(aps_level: u8, profession: &str) -> &'static str {
    match profession.to_lowercase().as_str() {
        "office administrator" => match aps_level {
            1..=2 => "Intern",
            3..=5 => "Administrator",
            5..=8 => "Senior Administrator",
            8..=10 => "Office Manager",
            10..=13 => "Director",
            _ => "CEO",
        },
        "academic" => match aps_level {
            3..=5 => "Research Assistant",
            5..=8 => "PhD Candidate",
            8..=10 => "Post-Doc Researcher",
            10..=13 => "Senior Lecturer",
            _ => "Dean",
        },
        "lawyer" => match aps_level {
            1..=2 => "Paralegal",
            3..=5 => "Junior Associate",
            5..=8 => "Associate",
            8..=10 => "Senior Associate 1-2",
            10..=13 => "Senior Associate 3-4",
            _ => "Partner",
        },
        "teacher" => match aps_level {
            1..=2 => "Placement",
            3..=5 => "Classroom Teacher",
            5..=8 => "Snr Teacher",
            8..=10 => "Leading Teacher",
            10..=13 => "Deputy Principal",
            _ => "Principal",
        },
        _ => "Unknown Profession",
    }
}

fn main() {
    println!("Welcome to the APS Level Checker!");

    let mut level_input = String::new();
    println!("Enter APS level (1–13 or SES):");
    io::stdin().read_line(&mut level_input).expect("Failed to read line");
    let level_input = level_input.trim();

    let aps_level = if level_input.to_lowercase() == "ses" {
        14 // Treat SES as level 14 for simplicity
    } else {
        match level_input.parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid APS level.");
                return;
            }
        }
    };

    let mut profession_input = String::new();
    println!("Enter profession (Office Administrator, Academic, Lawyer, Teacher):");
    io::stdin().read_line(&mut profession_input).expect("Failed to read line");
    let profession = profession_input.trim();

    let role = get_role(aps_level, profession);
    println!("Role for APS level {} in {}: {}", level_input, profession, role);
}
