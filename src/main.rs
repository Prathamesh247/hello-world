//print_type function lets us fetch the data type of a variable.

trait TypeDebug {
    fn print_type(&self);
}
impl <T> TypeDebug for T {
    fn print_type(&self) {
        println!("The type of variable is '{}'", std::any::type_name::<T>());
    }
}

fn main() {
    //Since variables in Rust are by default immutable,
    //We make them mutable by adding the keyword "mut".
    let mut x = "Hello";
    println!("{}, world!", x);
    x = "Bye";
    println!("{}, world!", x);
    x.print_type();

    let y:i32 = 5;
    y.print_type();

    //Tuple
    let t = (1, 2, 3);
    let (one, two, _) = t;
    println!("{} & {}", one, two);
    t.print_type();

    //Array
    let a = [1; 3];
    println!("The array is {:?}", a);
    a.print_type();

    //if else if flow control
    let b = 7;
    if b%2 == 0 {
        println!("Number is Even");
    }  else if b%2 == 1 {
        println!("Number is Odd");
    }  else {
        println!("It's not an integer!")
    }

    //loops
    let mut d = 2;
    loop {
        d = d*2;

        if d>5000 {
            break;
        } else {
            println!("d = {}", d);
        }
    }
    println!("Loop Ends!");

    //while loop
    let mut w = 2;
    while w < 5000 {
        w = w*2;
        println!("w = {}", w);
    }


    //implicit for loop
    for p in 0..5 { //upper limit is exclusive
        println!("p = {}", p*2);
    }
    //explicit for loop
    for q in 0..=5 { //upper limit is inclusive
        println!("q = {}", q*2);
    }
    //for loop used with iterators
    let arr = [7, 8, 9]; 
    for val in arr {
        println!("The value is {}", val);
    }
}