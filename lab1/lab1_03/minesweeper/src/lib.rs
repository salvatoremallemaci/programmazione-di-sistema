use std::char::from_digit;

pub fn adjacent(minefield: &[&str], r: usize, c: usize) -> u32 {

    let mut adj= 0;
    let r_max = minefield.len();
    let c_max = minefield[0].len();

    // check N
    if r>0 && minefield[r-1].bytes().nth(c).unwrap()==b'*' {	adj+=1; }

    // check S
    if r< r_max -1 && minefield[r+1].bytes().nth(c).unwrap()==b'*' { adj+=1; }

    // check E
    if c< c_max -1 && minefield[r].bytes().nth(c+1).unwrap()==b'*' { adj+=1; }

    // check O
    if c>0 && minefield[r].bytes().nth(c-1).unwrap()==b'*' { adj+=1; }

    // check NO
    if r>0 && c>0 && minefield[r-1].bytes().nth(c-1).unwrap()==b'*' { adj+=1; }

    // check NE
    if r>0 && c< c_max -1 && minefield[r-1].bytes().nth(c+1).unwrap()==b'*' { adj+=1; }

    // check SO
    if r< r_max -1 && c>0 && minefield[r+1].bytes().nth(c-1).unwrap()==b'*' { adj+=1; }

    // check SE
    if r< r_max -1 && c< c_max -1 && minefield[r+1].bytes().nth(c+1).unwrap()==b'*' { adj+=1; }

    return adj;
}


pub fn annotate(minefield: &[&str]) -> Vec<String> {

    println!("{:?}", minefield);

    let mut result: Vec<String> = Vec::new();

    for (r, &s) in minefield.iter().enumerate() {
        let mut line = String::new();

        for (c, &ch) in s.as_bytes().iter().enumerate() {
            match ch {
                b' ' => {
                    let adj = adjacent(minefield, r, c);
                    if adj == 0 { line.push(' '); }
                    else { line.push( from_digit(adj, 10).unwrap()); }
                },
                b'*' => {
                    line.push('*');
                },
                _ => panic!("Carattere non previsto")
            }
        }
        result.push(line);
    }
    println!("{:?}", result);
    return result;
}