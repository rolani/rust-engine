// test all functions in lib

use webapp::mult;
use webapp::add;
use webapp::sub;
use webapp::div;

fn main(){
    println!("Starting Tests");
    println!("mult(6,3) = {}", mult(6,3));
    println!("add(6,3) = {}", add(6,3));
    println!("sub(6,3) = {}", sub(6,3));
    println!("div(6,3) = {}", div(6,3));
}
