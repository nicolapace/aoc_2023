use std::fs;
use std::cmp;

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
    for line in lines {
        j=0;
        for el in line.bytes() {

            if el==b'.' {
                matrix_num[i as usize ][j as usize ] = b'.';
            } else if  el >= b'0' && el <= b'9'  {
                matrix_num[i as usize ][j as usize ] = el-b'0';
            } else {
                matrix_num[i as usize ][j as usize ] = b'.';
                matrix[i as usize ][j as usize ]=1;

                matrix[i as usize ][cmp::max(j-1,0) as usize]=1;
                matrix[i as usize ][cmp::min(j+1,len-1) as usize]=1;
                matrix[cmp::max(i-1,0) as usize][j as usize ]=1;
                matrix[cmp::min(i+1,len-1) as usize][j as usize ]=1;

                matrix[cmp::max(i-1,0) as usize ][cmp::max(j-1,0) as usize]=1;
                matrix[cmp::min(i+1,len-1) as usize ][cmp::min(j+1,len-1) as usize]=1;
                matrix[cmp::max(i-1,0) as usize ][cmp::min(j+1,len-1) as usize]=1;
                matrix[cmp::min(i+1,len-1) as usize][cmp::max(j-1,0) as usize]=1;
            }
            j=j+1;
        }
        i=i+1;    
    }
    
    for i in 0..=ssize-1 {
        println!("{:?}", matrix[i]);
    }   
    println!("");
    for i in 0..=ssize-1 {
        println!("{:?}", matrix_num[i]);
    }
    let mut index;
    let mut to_add;  
    let mut num;

    for i in 0..=ssize-1 {
        index = 0;
        to_add = false;
        num = 0 as u32;
        for j in 0..=(len as usize)  {
                        
            if j == len as usize || matrix_num[i][len as usize -1-j as usize]==b'.' {
                if to_add {
                    sum+=num as u32;
                    println!("num {} ", num); 
                }
                index = 0;
                to_add = false;
                num = 0;
                
            } else if j<len as usize {

                num += 10_u32.pow(index) * matrix_num[i][len as usize -1-j as usize] as u32;
                index+=1;

                if !to_add && matrix[i][len as usize -1-j as usize]!=0  {
                    to_add = true;
                }
                
            }

            

 
        }

       
    }
    println!("result: {}", sum); 
}
