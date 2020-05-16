use std::fs;

fn main() {
    let _z_data: &str = &(env!("HOME").to_string() + "/.z");
    if let Err(_) = fs::read(_z_data) {
        println!("ERROR: z.sh's datafile {} is not a file.", _z_data);
        return;
    }

    println!("ok")
}
