use std::fs;


fn main(){
    let path = "/Users/mikeshinoda/Music";
    let result = fs::read_dir(path).unwrap();
    result.for_each(|r|{
        println!("{:?}",r);
    });
    
    println!("hello from service.")
}
