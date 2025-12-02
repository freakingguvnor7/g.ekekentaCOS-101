use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("drinks.txt").expect("create failed");

    file.write_all("Lager:\n".as_bytes()).expect("write failed");
    file.write_all("33 Export\nDesperados\nGoldberg\nGulder\nHeineken\nStar\n\n".as_bytes()).expect("write failed");

    file.write_all("Stout:\n".as_bytes()).expect("write failed");
    file.write_all("Legend\nTurbo King\nWilliams\n\n".as_bytes()).expect("write failed");

    file.write_all("Non-Alcoholic:\n".as_bytes()).expect("write failed");
    file.write_all("Maltina\nAmstel Malta\nMalta Gold\nFayrouz\n".as_bytes()).expect("write failed");

    println!("Drink categories written to file.");
}