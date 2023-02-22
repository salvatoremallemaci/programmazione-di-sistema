/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut t = 0;

    if code.len() <= 1 { return false }

    for nmb in code.split_whitespace() {
        if nmb.len() != 4 { return false }
        for ch in nmb.chars(){
            if ch.is_numeric() == false {
                return false;
            }
        }
    }

    for nmb in code.split_whitespace() {

        for ch in nmb.chars().nth(0).unwrap().to_digit(10) {
            if ch * 2 > 9 { t += (ch*2)-9 } else { t += ch * 2}
            // println!("{}", t);
        }
        for ch in nmb.chars().nth(2).unwrap().to_digit(10) {
            if ch * 2 > 9 { t += (ch*2)-9 } else { t += ch * 2}
            // println!("{}", t);
        }
        for ch in nmb.chars().nth(1).unwrap().to_digit(10) {
            t += ch;
            // println!("{}", t);
        }
        for ch in nmb.chars().nth(3).unwrap().to_digit(10) {
            t += ch;
            // println!("{}", t);
        }
    }
    println!("t vale: {}", t);
    return t % 10 == 0;
}
