fn main() {
    
    let a = 18;
    let b = a;
    println!("a = {}",a);
    println!("b = {}",b);

    let my_name:String = String ::from("Hello from Heap Memory");
    {
        let your_name = my_name;
        //println!("my_name = {}",my_name);
        println!("your_name= {}",your_name);
    }
    //println!("my_name = {}",my_name);

    let s1:String = String :: from("Hello");
    let s2:String = get_string(s1);
    println!("{}",s2);

    let s3:String = String :: from("Hello from Avoid Ownership using Tuple");
    let (s4,len) = get_len(s3);
    println!("The length of string ' {} ' is {}",s4,len);

    let s5:String = String :: from ("Hello from the string clone");
    let len = get_clone_len(s5.clone());
    println!("The length of the string clone '{}' is {} , but this operation is expensive man",s5,len);

    let s6:String = String :: from ("Hello from the string borrowed");
    let len = get_length(&s6);
    println!("The length of the borrowed reference of string '{}' is {}",s6,len);

    let mut s7:String = String :: from ("Hello from mutable reference");
    
    let my_string:String= get_my_string();
    println!("{}",my_string);

}

fn get_string(received_string:String)->String{
    return received_string;
}

fn get_len(str:String)->(String,usize){
    let len:usize = str.len();
    return (str,len);
}

fn get_clone_len(str:String)->usize{
    return str.len();
}

fn get_length(str:&String)->usize{
    return str.len();
}

fn get_my_string()->String{
    let s = String::from("hiiiiiii");
    return s;
}
