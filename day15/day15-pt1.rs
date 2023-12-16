use std::fs;

fn main() {
    let mut sum = 0;
    for string  in fs::read_to_string("input").expect("No file\n").split(","){
        let mut current = 0;
        for el in string.bytes().collect::<Vec<_>>(){
            current+=el as u32;
            current*=17;
            current%=256;
        }
        sum+=current;
    }
    println!("total: {:?}",sum);   
}