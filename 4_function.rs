fn main() {
    //simple function
    dis();
    dis();

    //passing &str argument to the function
    let n_1: &str = "anurag00";
    dis_name(n_1);

    dis_name("anurag11");

    //passing String argument to the funtion
    let n: String = String::from("ashutosh22");
    dis_name_1(n);

    dis_name_1(String::from("Anurag33"));

    //get the
    let ans: String = get_name();
    println!("{}", ans);

    //passing u8 argument ot the function
    print_value(10);
    let num: u8 = 100;
    print_value(num);


    //calling the sum function and storing returning value into sum1 variable
    let sum1: u8 = sum(10, 220);

    println!("{}", sum1);

    //defining the variables with u8 data type
    let num1: u8 = 32;
    let num2: u8 = 18;

    let sum2: u8 = sum(num1, num2); //passing the above value to function

    println!("the sum of values= {}", sum2);
    // println!("the sum of values= {}",sum(num1,num2));


    let add_result: u8 = sum(50, 50);
    println!("{}", add_result);
}

fn dis() {
    print!("anurag")
}

fn dis_name(name: &str) {
    println!("{}", name)
}

fn dis_name_1(name: String) {
    //this function is not returning any data type it just printing init
    println!("{}", name)
}

fn get_name() -> String {
    //this function returning a String data type
    return String::from("Anurag String type");
}

fn sum(n1: u8, n2: u8) -> u8 { //function return  u8 data type
    let ans: u8 = n1 + n2;
    return ans;
}

fn print_value(item: u8) {
    println!("{}", item);
}
