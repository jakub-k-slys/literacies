use string_to_integer_atoi::{my_atoi, parse_int32_prefix};

fn main() {
    // A tiny demo so `cargo run` does something visible.
    for sample in [
        "42",
        "   -42",
        "4193 with words",
        "words and 987",
        "0",
        "99999999999",
    ] {
        println!(
            "{:>18}  parse_int32_prefix = {:<36} my_atoi = {}",
            format!("{sample:?}"),
            format!("{:?}", parse_int32_prefix(sample)),
            my_atoi(sample)
        );
    }
}
