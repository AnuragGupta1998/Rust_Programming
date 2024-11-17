fn main(){
    //calling function
    tuple_array()

}
fn tuple_array(){
    //tuple in rust.......

    let tup:(i32,f32,u8) = (500,5.9,2);
    let(x,y,z) = tup;  //Destructuring the tuple to accessing the tuples value

    //accessing the tuple by period(.)
    println!("first way access period to print tuple value x:{} y:{} z:{}",tup.0,tup.1,tup.2); // first way to print tuple value x:500 y:5.9 z:2
    println!("second way to print tuple value x:{}",x);                                        // second way to print tuple value x:500
    println!("third way to printing x:{} y:{} z:{}",x,y,z);                                    // third way to printing x:500 y:5.9 z:2

    let a= (1000,3.5,2);
    //accessing tuple by it period(.)
    println!("the forth way to print tuple {} {} {}",a.0,a.1,a.2);                             // the forth way to print tuple 1000 3.5 2
    
    //assigning the tuple value to another variable
    let anurag = a.0;
    println!("{anurag}");   // 1000

    let anu = x;
    println!("{}",anu);     // 500

    //Arrays..........................................
    let mut arr=[1,2,3,4,5];

    //accessing aray elements
    print!("{},{}",arr[0],arr[1]);

    //modifying arrays
    arr[0] = 100;
    println!("{}",arr[0])
}