fn main() {
    // Dataset 1: Names of Commissioners
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Dataset 3: Geopolitical Zones
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Merge datasets into a single vector of tuples
    let mut merged_data = Vec::new();

    for i in 0..names.len() {
        let record = (
            i + 1,
            names[i],
            ministries[i],
            zones[i],
        );
        merged_data.push(record);
    }

    // Display the merged data
    println!("{:<3} | {:<30} | {:<15} | {:<12}", "S/N", "Name", "Ministry", "Zone");
    println!("{}", "-".repeat(70));
    for (sn, name, ministry, zone) in merged_data {
        println!("{:<3} | {:<30} | {:<15} | {:<12}", sn, name, ministry, zone);
    }
}