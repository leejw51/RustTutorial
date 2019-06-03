fn setup(numbers:&mut [i32;20])
{
    let mut i:i32 =0 ;
    for i in 0.. numbers.len() {
        numbers[i]= 2000-(i as i32)*200;
    }
}
fn make() -> [i32;20]
{
    let mut numbers: [i32;20]=[5;20];
    setup(&mut numbers);
    let b= &numbers;
    println!("pointer {:?}",b as *const i32);
    return numbers;
}
fn main() 
{
    let numbers = make();
    let b= &numbers;
    println!("pointer {:?}",b as *const i32);
    for i in &numbers {
        print!("{}  ",i);
    }
}
