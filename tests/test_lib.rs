// test all functions in lib

use webapp::add;
use webapp::div;
use webapp::mult;
use webapp::sub;

#[test]
fn test_mult(){
    assert_eq!(mult(6, 3), 18);
}


#[test]
fn test_add(){
    assert_eq!(add(6, 3), 9);
}


#[test]
fn test_sub(){
    assert_eq!(sub(6, 3), 3);
}


#[test]
fn test_div(){
    assert_eq!(div(6, 3), 2);
}
