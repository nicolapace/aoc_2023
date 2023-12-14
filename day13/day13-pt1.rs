use std::fs;
use std::collections::HashMap;


fn main() {
    /* PARSING */
    let contents = fs::read_to_string("test").expect("No file\n");

    for block in contents.split("\n\n"){
        let mut horizontal_vec : Vec<u32> = vec![];
        
        let mut matrix : Vec<_> = vec![];

        for line in block.split("\n") {
            let mut num = 0;
            let mut array : Vec<u32> = vec![];
            for el in line.chars() {
                if el == '#' {
                    num+=1;
                    array.push(1);
                }else{
                    array.push(0);
                }
                num = num<<1;
            }
            horizontal_vec.push(num>>1);
            matrix.push(array);
        }

        let mut vertical_vec : Vec<u32> = vec![0;matrix[0].len()];
        for i in 0..matrix[0].len(){
            for row in &matrix {
                if row[i] == 1 {
                    vertical_vec[i]+=1;
                }
                vertical_vec[i]= vertical_vec[i]<<1;
            }
            vertical_vec[i]= vertical_vec[i]>>1;
        }

        for el in vertical_vec {
            println!("{:b}", el);
        }
        println!();
    }
    
}
