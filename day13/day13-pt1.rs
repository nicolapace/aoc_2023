use std::fs;
// use std::collections::HashMap;

fn is_even_palindrome( vec : &[u32] /*,  cache: &mut HashMap<Vec<u32>, bool> */) -> bool{
    if vec.len() < 2 || vec.len() % 2 == 1 {
        return false;
    }
    // if let Some(result) = cache.get(&vec.to_vec()) {
    //     println!("using cache");
    //     return result.clone();
    // }
    for i in 0..vec.len()/2 {
        if vec[i] != vec[vec.len()-1-i] {
            // cache.insert(vec.to_vec(), false);
            return false;
        }
    }
    // cache.insert(vec.to_vec(), true);
    return true;


}

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");

    // let mut cache_palindorme : HashMap<Vec<u32>, bool> = HashMap::new();
    let mut sum =0;

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

        let mut found = false;   
        let mut result :usize= 0;

        let mut i = horizontal_vec.len()%2;
        while i<horizontal_vec.len() -1{
            if is_even_palindrome(&horizontal_vec[..i]/*, &mut cache_palindorme*/){
                found = true;
                result = i/2;
                break;
            }
            if is_even_palindrome(&horizontal_vec[i..]/*, &mut cache_palindorme*/){
                found = true;
                result = i + (horizontal_vec.len()-i)/2 ;
                break;
            }
            i+=  2-horizontal_vec.len()%2 ;
        }
        result *= 100;
        if !found {
            let mut i = vertical_vec.len()%2;
            while i<vertical_vec.len()-1 {
                if is_even_palindrome( &vertical_vec[..i] /*, &mut cache_palindorme*/){
                    result = i/2;
                    break;
                }
                if is_even_palindrome( &vertical_vec[i..],/* &mut cache_palindorme*/){
                    result = i + (vertical_vec.len()-i)/2 ;
                    break;
                }
                i+=  2-vertical_vec.len()%2 ;
            }
        }
        sum+=result;       
    }
    println!("total: {}", sum);
    
}
