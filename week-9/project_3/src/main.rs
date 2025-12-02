use std::io::Write;

fn main() {
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Muritala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiyeye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = std::fs::File::create("efcc_records.txt").expect("create failed");

    file.write_all("EFCC - Convicted Ministers Record\n\n".as_bytes()).expect("write failed");
    file.write_all("S/N\tName\t\t\t\tMinistry\t\t\tZone\n".as_bytes()).expect("write failed");
    file.write_all("--------------------------------------------------------------------------\n".as_bytes()).expect("write failed");

    for i in 0..commissioners.len() {
        let line = format!(
            "{}\t{}\t{}\t{}\n",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
        file.write_all(line.as_bytes()).expect("write failed");
    }

    println!("Merged EFCC records saved to file.");
}
