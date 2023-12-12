use std::fs;
use std::cmp;
use std::collections::HashMap;

fn main() {

    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n");
    let ssize = lines.clone().collect::<Vec<_>>().len();
    let len = lines.clone().collect::<Vec<_>>()[1].bytes().count() as i32;
    println!("size x: {:?}", len);
    println!("size y: {:?}", ssize);

    let mut matrix : Vec<Vec<u8>> = vec![vec![0; len as usize]; ssize as usize];
    let mut matrix_num = matrix.clone();

    let mut sum : u32 = 0;
    let mut i : i32 = 0;
    let mut j : i32 ;
    let mut gear :u32 = 1;
    for line in lines {
        j=0;
        for el in line.bytes() {

            if el==b'.' {
                matrix_num[i as usize ][j as usize ] = b'.' as u32;
            } else if  el >= b'0' && el <= b'9'  {
                matrix_num[i as usize ][j as usize ] = el as u32 -b'0'  as u32;
            } else if el==b'*' {
                
                gear += 1;

                matrix_num[i as usize ][j as usize ] = b'.'  as u32;                

                matrix[i as usize ][j as usize ]=gear;

                matrix[i as usize ][cmp::max(j-1,0) as usize]= gear;
                matrix[i as usize ][cmp::min(j+1,len-1) as usize]= gear;
                matrix[cmp::max(i-1,0) as usize][j as usize ]= gear;
                matrix[cmp::min(i+1,len-1) as usize][j as usize ]= gear;

                matrix[cmp::max(i-1,0) as usize ][cmp::max(j-1,0) as usize] = gear;
                matrix[cmp::min(i+1,len-1) as usize ][cmp::min(j+1,len-1) as usize] = gear;
                matrix[cmp::max(i-1,0) as usize ][cmp::min(j+1,len-1) as usize]=gear;
                matrix[cmp::min(i+1,len-1) as usize][cmp::max(j-1,0) as usize]=gear;

            }
            j+=1;
        }
        i+=1;    
    }
    
    let mut index;
    let mut to_add;  
    let mut num;
    let mut gears : HashMap<u32, u32> = HashMap::new();
    let mut near_gear = 0;
        
    for i in 0..ssize {
        index = 0;
        to_add = false;

        num = 0 as u32;
        for j in 0..=(len as usize)  {
                        
            if j == len as usize || matrix_num[i][len as usize -1-j as usize]==b'.'  as u32{
                if to_add {
                    let old_gear_val =  gears.get(&near_gear) ;
                    
                    if old_gear_val==None {
                        gears.insert( near_gear, num );
                    }else{
                        sum+=num * (*gears.get(&near_gear).unwrap() as u32);
                    }
                }
                index = 0;
                to_add = false;
                num = 0;
            }else if j<len as usize {
                num += 10_u32.pow(index) * matrix_num[i][len as usize -1-j as usize] as u32;
                index+=1;
                if !to_add && matrix[i][len as usize -1-j as usize]!=0  {
                    to_add = true;
                    near_gear = matrix[i][len as usize -1-j as usize];
                }
            }
        }

    }

    println!("result: {:?}", sum); 
}
