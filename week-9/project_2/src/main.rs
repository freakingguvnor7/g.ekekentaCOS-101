use std::io::Write;

struct Student {
    name: &'static str,
    matric: &'static str,
    department: &'static str,
    level: u32,
}

fn main() {
    let students = vec![
        Student { name: "Oluchi Morid", matric: "ACC10211111", department: "Accounting", level: 300 },
        Student { name: "Adams Aliyu", matric: "ECO10110101", department: "Economics", level: 300 },
        Student { name: "Shania Bolade", matric: "CSC10328828", department: "Computer", level: 200 },
        Student { name: "Adekunle Gold", matric: "EEE10202002", department: "Electrical", level: 100 },
        Student { name: "Blanca Edemoh", matric: "MEE10202001", department: "Mechanical", level: 200 },
    ];

    let mut file = std::fs::File::create("students.txt").expect("create failed");

    file.write_all("PAU SMIS - Student Records\n\n".as_bytes()).expect("write failed");
    file.write_all("Name\t\tMatric\t\tDepartment\tLevel\n".as_bytes()).expect("write failed");
    file.write_all("--------------------------------------------------------\n".as_bytes()).expect("write failed");

    for student in students {
        let line = format!("{}\t{}\t{}\t{}\n", student.name, student.matric, student.department, student.level);
        file.write_all(line.as_bytes()).expect("write failed");
    }

    println!("Student records saved to file.");
}