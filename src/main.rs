mod z;

fn main() {
    if let Some(_z_data) = z::data_path() {
        println!("datafile:{:?}", _z_data);
        println!("paths:{:?}", z::_z_dirs(_z_data.as_str()));
    } else {
        println!("!");
    }
}
