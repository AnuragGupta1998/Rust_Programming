fn main() {
    //condition
    
    //if-else 
    let is_even: bool = false;

    if is_even {
        println!("if condition in rust")

    }else if !is_even {
        println!("if - else part")
    }else {
        println!("else part")
    }

    //loop for loop
    
    //it print from 0 to 10 in this loop 10 is includes
    for i in 0..=10{
        print!("{}",i)
    }
    //it print from 0 to 9 in this loop 10 is excluded(not include)
    for i in 0..10{
        println!("{}",i)
    }

    let a:i32=20;

    for a in 0..=a {
        println!("{}",a)

    }

    for i in 0..=10{
        println!("anurag={}",i)
    }

}
