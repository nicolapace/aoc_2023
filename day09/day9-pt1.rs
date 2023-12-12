use std::fs;

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n").collect::<Vec<_>>();
    let mut sequences : Vec<_> = vec![];
    for line in lines{
        let mut seq : Vec<i64> = vec![];
        for num in line.split_whitespace().collect::<Vec<_>>(){
            let mut pow = 1;
            let mut num_int = 0;
            for i in 1..=num.bytes().count(){
                if num.chars().count()-i == 0 && num.bytes().nth(num.chars().count()-i).unwrap() == b'-' {
                        num_int = -num_int;
                }else{
                    num_int+=pow*(num.bytes().nth(num.chars().count()-i).unwrap()-b'0') as i64;
                    pow*=10;
                }

            }
            seq.push(num_int);
        }
        sequences.push(seq.clone());
        println!("sequences: {:?}", seq);
    }   
    
    /* SOLVE: construct the NxM matix for each sequence */
    let mut sum = 0;
    for seq in sequences {
        let mut matrix : Vec<_> = vec![];
        let mut all_zeros = false;
        let mut row = 0;
        matrix.push(seq.clone());
        while !all_zeros{
            let mut new_row : Vec<i64> = vec![0; seq.len()];
            all_zeros=true;
            for i in row..=seq.len()-2{
                new_row[i+1] = matrix[matrix.len()-1][i+1]-matrix[matrix.len()-1][i];
                if all_zeros && new_row[i+1]!=0 {
                    all_zeros=false;
                }
            }
            row+=1;
            matrix.push(new_row);
        }
        for step in &matrix {
            sum+=step[step.len()-1];
        }
        println!("{:?}", matrix);
    }
    println!("result: {:?}", sum);
}
