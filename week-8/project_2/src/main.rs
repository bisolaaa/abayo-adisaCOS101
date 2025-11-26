use std::cmp::Ordering;

#[derive(Debug)]
struct Candidate {
    name: String,
    years_experience: u32,
}

fn find_most_experienced(candidates: &[Candidate]) -> Option<&Candidate> {
    candidates.iter().max_by(|a, b| a.years_experience.cmp(&b.years_experience))
}

fn main() {
    let candidates = vec![
        Candidate {
            name: String::from("Ada"),
            years_experience: 5,
        },
        Candidate {
            name: String::from("Chinedu"),
            years_experience: 8,
        },
        Candidate {
            name: String::from("Fatima"),
            years_experience: 3,
        },
        Candidate {
            name: String::from("Tunde"),
            years_experience: 10,
        },
    ];

    match find_most_experienced(&candidates) {
        Some(candidate) => {
            println!(
                "The most experienced candidate is {} with {} years of experience.",
                candidate.name, candidate.years_experience
            );
        }
        None => println!("No candidates were provided."),
    }
}
