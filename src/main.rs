use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("database.txt").expect("Ошибка");

    let ray = vec![
        "0000", "1111", "2222", "3333", "4444", "5555", "6666", "7777", "8888", "9999",
    ];
    let code = "99361";
    let mut x = 0.0000001;


    while x < 1.0 {
        x = x + 0.000001;
        let y = format!("{:.6}", x);
        let c = &y[2..];

        let z = format!("{}{}", code, c);

        if !z.contains(ray[0])
            && !z.contains(ray[1])
            && !z.contains(ray[2])
            && !z.contains(ray[3])
            && !z.contains(ray[4])
            && !z.contains(ray[5])
            && !z.contains(ray[6])
            && !z.contains(ray[7])
            && !z.contains(ray[8])
            && !z.contains(ray[9])
        {
            println!("{}", z);
            file.write_all(z.as_bytes()).expect("Error");
            file.write_all("\n".as_bytes()).expect("Error");
            println!("Number recorded");
        }
    }
}
