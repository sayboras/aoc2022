mod day_01;
mod day_02;
mod day_03;
mod file_reader;


fn main() {
    let day: String = std::env::args()
        .nth(1)
        .expect("No day given. Possible options are: 1-25.");
    let day_slice: &str = day.as_str();

    match day_slice {
        "1" => day_01::run(),
        "2" => day_02::run(),
        "3" => day_03::run(),
        _ => println!("No valid day given. Possible options are: 1-25."),
    };
}
