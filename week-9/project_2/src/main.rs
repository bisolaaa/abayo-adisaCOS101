use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn display_students(students: &[Student]) {
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    println!("{:-<60}", "");
    for student in students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        );
    }
}

fn save_to_file(students: &[Student], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "PAU SMIS")?;
    writeln!(file, "{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level")?;
    writeln!(file, "{:-<60}", "")?;
    for student in students {
        writeln!(
            file,
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        )?;
    }
    Ok(())
}

fn main() {
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10210101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_number: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric_number: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    display_students(&students);

    match save_to_file(&students, "students.txt") {
        Ok(_) => println!("\nStudent details saved to 'students.txt'."),
        Err(e) => eprintln!("Error saving to file: {}", e),
    }
}