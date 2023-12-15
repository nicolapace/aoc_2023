use std::fs;
use std::collections::HashMap;


fn turn_left( matrix : &mut Vec<Vec<char>> ) {
    
    let mut matrix_ret : Vec<Vec<char>> = vec![vec!['.';matrix.len()]; matrix[0].len()];

    for j in 0..matrix.len() {
        for i in 0..matrix[j].len() {
            matrix_ret[matrix[j].len()-1-i][j] = matrix[j][i];
        }
    }
    *matrix = matrix_ret;
}

fn turn_right( matrix : &mut Vec<Vec<char>> ) {
    
    let mut matrix_ret : Vec<Vec<char>> = vec![vec!['.';matrix.len()]; matrix[0].len()];

    for j in 0..matrix.len() {
        for i in 0..matrix[j].len() {
            matrix_ret[i][matrix.len()-1-j] = matrix[j][i];
        }
    }
    *matrix = matrix_ret;
}

fn split_hastags( v : &Vec<char> ) -> Vec<Vec<usize>> {
    let mut ret_vec : Vec<Vec<usize>> = vec![];
    let mut elements : Vec<usize> = vec![0,0];
    for el in v {
        match el {
            'O' => elements[0] += 1,
            '.' => elements[1] += 1,
             _  => {ret_vec.push(elements); elements = vec![0,0]}
        }
    }
    ret_vec.push(elements);
    return ret_vec;
} 

fn move_balls_left_vec( data : &Vec<Vec<usize>> ) -> Vec<char> {
    let mut ret_vec : Vec<char> = vec![];
   
    for k in 0..data.len() {
        for _ in 0..data[k][0] {
            ret_vec.push('O');
        }
        for _ in 0..data[k][1] {
            ret_vec.push('.');
        }
        ret_vec.push('#');
    }
    ret_vec.pop();
    return ret_vec;
} 

fn move_balls_right_vec( data : &Vec<Vec<usize>> ) -> Vec<char> {
    let mut ret_vec : Vec<char> = vec![];
   
    for k in 0..data.len() {
        for _ in 0..data[k][1] {
            ret_vec.push('.');
        }
        for _ in 0..data[k][0] {
            ret_vec.push('O');
        }
        ret_vec.push('#');
    }
    ret_vec.pop();
    return ret_vec;
} 

fn move_balls_left( matrix : &mut Vec<Vec<char>> ) {
    let mut matrix_ret :Vec<_> = vec![];
    for el in &mut *matrix {
        matrix_ret.push(move_balls_left_vec(&split_hastags(&el)));
    }
    *matrix = matrix_ret;
} 
fn move_balls_right( matrix : &mut Vec<Vec<char>> ) {
    let mut matrix_ret :Vec<_> = vec![];
    for el in &mut *matrix {
        matrix_ret.push(move_balls_right_vec(&split_hastags(&el)));
    }
    *matrix = matrix_ret;
} 

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");

    let mut matrix : Vec<Vec<char>> = vec![];
    for line in contents.split("\n"){
        matrix.push(line.chars().collect::<Vec<char>>());
    }

    let mut matrices_set : HashMap<Vec<Vec<char>>, Vec<usize>> = HashMap::new();
    
    let mut loop_found = false;
    let mut num_cycles = 1000000000;
    let mut i = 0;
    while i < num_cycles{
        /* tilting north*/
        turn_left(&mut matrix);
        move_balls_left(&mut matrix);
        turn_right(&mut matrix);

        /* tilting west*/
        move_balls_left(&mut matrix);

        /* tilting south*/
        turn_left(&mut matrix);
        move_balls_right(&mut matrix);
        turn_right(&mut matrix);

        /* tilting east*/
        move_balls_right(&mut matrix); 
        
        /* searching for loops */
        if !loop_found{
            if matrices_set.get(&matrix) == None {
                matrices_set.insert(matrix.clone(), vec![i,0]);
            } else {

                if matrices_set.get(&matrix).unwrap()[1]==1 {
                    loop_found = true;
                    /* do the remaining cycles */
                    num_cycles = (num_cycles-(matrices_set.get(&matrix).unwrap()[0]))%(i-matrices_set.get(&matrix).unwrap()[0]);
                    i=0;
                }else{
                    matrices_set.insert(matrix.clone(), vec![matrices_set.get(&matrix).unwrap()[0],matrices_set.get(&matrix).unwrap()[1]+1]);
                    println!("FOUND LOOP! {} = {:?}", i , matrices_set.get(&matrix).unwrap()[0]);
                }
                
            }
        }
        i+=1;
    }   

    /* CALCULATING RESULT */
    let mut sum = 0; 
    for i in 0..matrix.len() {
        let mut row_sum = 0;
        for el in &matrix[i]{
            if *el == 'O' {
                row_sum += 1;
            }
        }
        sum += row_sum * (matrix.len()-i);
    }
    println!("total: {:?}",sum);
    
}
