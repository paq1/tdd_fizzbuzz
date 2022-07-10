const FIZZ_MULTIPLE: u32 = 3;
const BUZZ_MULTIPLE: u32 = 5;
const FIZZ: &str = "fizz";
const BUZZ: &str = "buzz";

pub fn fizzbuzz(number: &u32) -> String {
    match number {
        a if multiple(a, &FIZZ_MULTIPLE) && multiple(a, &BUZZ_MULTIPLE) => format!("{FIZZ}{BUZZ}"),
        a if multiple(a, &FIZZ_MULTIPLE) => FIZZ.to_string(),
        a if multiple(a, &BUZZ_MULTIPLE) => BUZZ.to_string(),
        _ => number.to_string()
    }
}

fn multiple(a: &u32, b: &u32) -> bool {
    a % b == 0
}