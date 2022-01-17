trait TypeDebug {
    fn print_type(&self);
}
impl <T> TypeDebug for T {
    fn print_type(&self) {
        println!("The type of variable is '{}'", std::any::type_name::<T>());
    }
}

fn main() {

    let mut x = "Hello";
    println!("{}, world!", x);
    x = "Bye";
    println!("{}, world!", x);
    x.print_type();

    let y:i32 = 5;
    y.print_type();

    let t = (1, 2, 3);
    let (one, two, _) = t;
    println!("{} & {}", one, two);
    t.print_type();

    let a = [1; 3];
    println!("The array is {:?}", a);
    a.print_type();
}
