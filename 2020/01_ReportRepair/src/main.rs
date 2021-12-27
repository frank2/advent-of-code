use std::io;

fn read_report() -> Result<Vec<usize>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut result = Vec::<usize>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        result.push(buffer.trim().parse().unwrap());
        buffer.clear();
    }

    if result.len() == 0 { Err(()) }
    else { Ok(result) }
}

fn part1() {
    if let Ok(report) = read_report() {
        for i in 0..report.len() {
            for j in 0..report.len() {
                if i == j { continue; }

                if report[i]+report[j] != 2020 { continue; }

                println!("{}", report[i]*report[j]);
                return;
            }
        }
    }
    else { panic!("couldn't read report!"); }
}

fn part2() {
    if let Ok(report) = read_report() {
        for i in 0..report.len() {
            for j in 0..report.len() {
                if i == j { continue; }
                
                for k in 0..report.len() {
                    if i == k || j == k { continue; }

                    if report[i]+report[j]+report[k] != 2020 { continue; }

                    println!("{}", report[i]*report[j]*report[k]);
                    return;
                }
            }
        }
    }
    else { panic!("couldn't read report!"); }
}

fn main() {
    // part1();
    part2();
}
