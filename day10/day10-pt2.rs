use std::fs;
use std::cmp;
use std::collections::HashSet;

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n").collect::<Vec<_>>();
    let ssize = lines.len();
    
    println!("size y: {:?}", ssize);

    let mut matrix : Vec<Vec<char>> = Vec::with_capacity(ssize);
    let len = lines[0].bytes().count();
    
    println!("size x: {:?}", len);
    let mut start : Vec<usize> = vec![0, 0];
    for j in 0..ssize {
        matrix.push(vec!['0'; len]);
        for i in 0..lines[j].bytes().count(){
            matrix[j][i] = lines[j].chars().nth(i).unwrap();
            if matrix[j][i] == 'S'{
                start[0]=i;
                start[1]=j;
            }
        }
    }

    /* find first step after start*/
    let mut next_step = vec![0, 0];
    if matrix[start[1]][start[0]+1] == '-' || matrix[start[1]][start[0]+1] == '7' || matrix[start[1]][start[0]+1] == 'J' {
        next_step = vec![start[0]+1, start[1]];
    }else if matrix[start[1]+1][start[0]] == '|' || matrix[start[1]+1][start[0]] == 'J' || matrix[start[1]+1][start[0]] == 'L' {
        next_step = vec![start[0], start[1]+1];
    }else if matrix[start[1]][start[0]-1] == '-' || matrix[start[1]][start[0]-1] == 'L' || matrix[start[1]][start[0]-1] == 'F' {
        next_step = vec![start[0]-1, start[1]];
    }else if matrix[start[1]-1][start[0]] == '|' || matrix[start[1]-1][start[0]] == 'F' || matrix[start[1]-1][start[0]] == '7' {
        next_step = vec![start[0]-1, start[1]];
    }
    
    /* travel the loop */
    let mut path : Vec<Vec<usize>>  = vec![];
    let mut path_set : HashSet<Vec<usize>> = HashSet::new();
    path.push(start.clone());
    path_set.insert(start.clone());
    path.push(next_step.clone());
    path_set.insert(next_step.clone());
    while matrix[next_step[1]][next_step[0]]!='S'{
        match matrix[next_step[1]][next_step[0]] {
            '-' => if path[path.len()-2][0]==next_step[0]-1  { next_step[0]+=1 } else { next_step[0]-=1 },
            '|' => if path[path.len()-2][1]==next_step[1]-1  { next_step[1]+=1 } else { next_step[1]-=1 },
            'L' => if path[path.len()-2][1]==next_step[1]-1  { next_step[0]+=1 } else { next_step[1]-=1 },
            'J' => if path[path.len()-2][1]==next_step[1]-1  { next_step[0]-=1 } else { next_step[1]-=1 },
            'F' => if path[path.len()-2][1]==next_step[1]+1  { next_step[0]+=1 } else { next_step[1]+=1 },
            '7' => if path[path.len()-2][1]==next_step[1]+1  { next_step[0]-=1 } else { next_step[1]+=1 },
             _  => continue
        }
        path.push(vec![ next_step[0], next_step[1] ]);
        path_set.insert(vec![ next_step[0], next_step[1] ]);
    }
    
    /* for each tile if at north, south, est and west there are always odd number of tiles of the path, then the tile is inside the path-loop */

    let mut num_inside_loop = 0;
    for j in 1..=matrix.len()-2{
        for i in 1..=matrix[0].len()-2{

            if path_set.get(&vec![i,j])!=None {
                continue;
            }

            let mut num_crosses_left = 0;
            let mut num_l = 0;
            let mut num_7 = 0;
            let mut num_j = 0;
            let mut num_f = 0;
            for k in 0..i {
                if path_set.get(&vec![k,j])!=None {
                    match matrix[j][k] {
                        '|' => num_crosses_left+=1,
                        'L' => num_l+=1,
                        '7' => num_7+=1,
                        'J' => num_j+=1,
                        'F' => num_f+=1,
                         _  => continue
                    }  
                }
            }
            if ( num_crosses_left + cmp::min(num_7,num_l) + cmp::min(num_j,num_f) ) % 2 == 0 {
                continue;
            }
            

            let mut num_crosses_right = 0;
            let mut num_l = 0;
            let mut num_7 = 0;
            let mut num_j = 0;
            let mut num_f = 0;
            for k in i+1..matrix[0].len() {
                if path_set.get(&vec![k,j])!=None {
                    match matrix[j][k] {
                        '|' => num_crosses_right+=1,
                        'L' => num_l+=1,
                        '7' => num_7+=1,
                        'J' => num_j+=1,
                        'F' => num_f+=1,
                         _  => continue
                    }  
                }
            }
            if ( num_crosses_right + cmp::min(num_7,num_l) + cmp::min(num_j,num_f) )  % 2 == 0 {
                continue;
            }

            
            let mut num_crosses_up = 0;
            let mut num_l = 0;
            let mut num_7 = 0;
            let mut num_j = 0;
            let mut num_f = 0;
            for k in 0..j {
                if path_set.get(&vec![i,k])!=None {
                    match matrix[k][i] {
                        '-' => num_crosses_up+=1,
                        'L' => num_l+=1,
                        '7' => num_7+=1,
                        'J' => num_j+=1,
                        'F' => num_f+=1,
                         _  => continue
                    }  
                }
            }
            if ( num_crosses_up + cmp::min(num_7,num_l) + cmp::min(num_j,num_f) ) % 2 == 0 {
                continue;
            }


            let mut num_crosses_down = 0;
            let mut num_l = 0;
            let mut num_7 = 0;
            let mut num_j = 0;
            let mut num_f = 0;
            for k in j+1..matrix.len() {
                if path_set.get(&vec![i,k])!=None {
                    match matrix[k][i] {
                        '-' => num_crosses_down+=1,
                        'L' => num_l+=1,
                        '7' => num_7+=1,
                        'J' => num_j+=1,
                        'F' => num_f+=1,
                         _  => continue
                    } 
                }
            }
            if ( num_crosses_down + cmp::min(num_7,num_l) + cmp::min(num_j,num_f) ) % 2 == 0 {
                continue;
            }

            num_inside_loop+=1;
        }
    }

    println!("\nnum_inside_loop: {:?}", num_inside_loop);
    
}
