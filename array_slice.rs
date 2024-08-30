// Simple Rust program
fn main() 
{
    let arr = [0, 1, 2, 3];
    let slice = &arr[1..3];

    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]){
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("Length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}