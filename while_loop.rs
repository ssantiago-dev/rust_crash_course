fn main() 
{
    let mut n = 0;
    while n < 5{
        println!("{}", n);
        n+=1;
        if n == 3{
            println!("Exit");
            break
        }
    }    
}