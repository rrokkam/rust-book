fn main() {
    let rect = (30, 50);
    println!("Area = {}", area(rect));
}

fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
