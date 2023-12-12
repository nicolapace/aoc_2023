use std::fs;

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n").collect::<Vec<_>>();
    let ssize = lines.len();
    
    let mut matrix : Vec<Vec<char>> = Vec::with_capacity(ssize);
    let len = lines[0].bytes().count();
    
    println!("size x: {:?}", len);
    println!("size y: {:?}", ssize);

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
        println!("{:?}", matrix[j]);
    }
    println!("start: {:?}", start);

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
    path.push(start);
    path.push(next_step.clone());
    while matrix[next_step[1]][next_step[0]]!='S' {
        println!("step: {:?}", next_step);
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
    }
    println!("result: {:?}", path.len()/2);
    
}
