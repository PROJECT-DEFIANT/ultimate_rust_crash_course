const MISSILES: i32 = 8;
const READY: i32 = 2;
fn main() {
    println!("Fireing of my {} missiles, when {}", MISSILES, READY);
    let missiles = MISSILES -  READY;

    let (mut x, y) = (4,5);
    x = x - y;

    println!("x:{},y:{}",x,y);
    println!("Remaining missiles {}", missiles);
}
