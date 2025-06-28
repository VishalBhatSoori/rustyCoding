fn main() {
    //integer
    let mut num : u8 = 255;
    println!("The value stored in num is :{}",num);
    num =199;
    println!("The value stored in num is :{}",num);
    //strings
    let my_string: &str = "Hello this is Vishal";
    println!("This is a string literal which is of fixed length: {}",my_string);

    let mut my_stringy:String = String :: from("Hiii from Vishal");
    my_stringy.push_str(" to Kavyaa");
    println!("This is a dynamic string: {}",my_stringy);
    //tuples
    let emp_info :(&str,u8) = ("Vishal",21);
    let emp_name=emp_info.0;
    let emp_age=emp_info.1;
    println!("Emp_name: {} & Emp_age: {}",emp_name,emp_age);

    let (employee_name,employee_age) = emp_info;
    println!("Emp_name: {} & Emp_age: {}",employee_name,employee_age);

    //functions
    //calling void functions 
    print_hello();
    let num : u8 = 18;
    print_value(num);
    let num2: u8 = 255;
    let ans=addfn(num,num2);
    println!("{}",ans);

}

fn print_hello(){
    println!("Hello from Vishal");
}
fn print_value(num:u8){
    println!("{}",num);
}

fn addfn(num:u8,num2:u8)->u16{
    return (num as u16 + num2 as u16);
}

