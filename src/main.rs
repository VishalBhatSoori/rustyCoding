fn main(){
    let arr:[&str;3]= ["Hello"," from ","Vishal"];
    change_arr(arr);
    println!("{:?}",arr);

    // passing array be reference in stack memory
    let mut arr2:[&str;3]=["Hello"," from ","Vishal the Boss"];
    change_mut_arr(& mut arr2);
    println!("{:?}",arr2);

    //vectors 

    //let mut v:Vec<u32>= Vec:: new();
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}",v);

    let mut v1 = vec![1,2,3,4,5];
    v1.pop();
    println!("{:?}",v1);

    //     NOte : vector follows ownership rules as its data resides in heap memory

    let x = 5;
    
}

fn change_arr(mut arr1:[&str;3]){
    arr1[0]="Hiii";
    println!("{:?}",arr1);
}

fn change_mut_arr(arr3:&mut[&str;3]){
    arr3[0]="Hiii";
    println!("{:?}",arr3);
}