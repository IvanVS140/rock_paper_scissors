#[allow(unused)]

pub fn who_wins(x: usize, y: usize) -> u32 {
    if x == y {
        println!("\nDraw.");
        return 0;
    }
    else if x == (y + 2) % 3 {
        println!("Vega wins!");
        return 1;
    }
    else {
        println!("User wins!");
        return 2;
    }
}