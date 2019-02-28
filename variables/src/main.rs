//Const must be annotated
//Naming is all caps with underscores
//You can also underscore large numbers for readability
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("const MAX POINTS is: {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Example of varaible shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    //Example of shadowing where we change the type
    let spaces = "      ";
    let spaces = spaces.len();
    println!("There are {} spaces!", spaces);

}
