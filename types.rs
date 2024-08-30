fn main() 
{
    // u8, u16, u32, u64, u128
    let unsigned: u8 = 10;
    // i8, i16, i32, i64, i128
    let signed: i8 = -10;
    let float: f32 = 1.1;

    println!("unsigned: {} signed: {} float: {}", unsigned, signed, float);

    let letter = "c1232";
    let emoji = "\u{1F600}";

    println!("letter: {} emoji: {}", letter, emoji);

    let is_true: bool = true;

    println!("isTrue: {}", is_true);
}