use std::fs;

fn main() {
    
    let mut y : i64 = 0;
    let mut min_y : i64 = 99999;
    let mut max_y : i64 = -99999;
    let mut data : Vec<(char, i64)> = vec![];

    /* PARSING WHILE FINDING MAX AND MIN ON THE y AXIS */
    for line in fs::read_to_string("input").expect("No file\n").split("\n"){
        let segment = line.split('#').collect::<Vec<_>>();
        let num = i64::from_str_radix(&segment[1][..segment[1].chars().count()-2], 16).unwrap();

        match segment[1].chars().nth(segment[1].chars().count()-2).unwrap() {
            '0' /*R*/ => {
                data.push(('R', num));
            },
            '2' /*L*/=> {
                data.push(('L', num));
            },
            '3' /*U*/ => {
                y-=num;
                if y > max_y {
                    max_y = y;
                }
                if y < min_y {
                    min_y = y;
                }
                data.push(('U', num));
            },
            '1'/*D*/ => {
                y+=num;
                if y > max_y {
                    max_y = y;
                }
                if y < min_y {
                    min_y = y;
                }
                data.push(('D', num));
            },
             _  => println!("ERROR!")
        }
    }
      
    println!("min y {}  max y {}", min_y, max_y);

    let mut integral : i64 = 0; 
    let mut command_before;
    let mut command_after;
    let mut i=0;
    
    /* Adding area if right direction, subtracting if left */
    for command in &data {
        command_before = if i==0 {data[data.len()-1].0} else {data[i-1].0};
        command_after = if i==data.len()-1 {data[0].0} else {data[i+1].0};
        match command.0 {
            'R'  => {
                if  command_before=='U' && command_after == 'D' {
                    integral+=(command.1+1)*(max_y-y+1);
                } else if command_before=='D' && command_after == 'U' {
                    integral+=(command.1-1)*(max_y-y+1);
                } else  {
                    integral+=(command.1)*(max_y-y+1);
                }
            },
            'L' => {
                if command_before=='D' && command_after == 'U' {
                    integral-=(command.1+1)*(max_y-y);
                } else if command_before=='U' && command_after == 'D' {
                    integral-=(command.1-1)*(max_y-y);
                } else {
                    integral-=(command.1)*(max_y-y);
                }
            },
            'D' => {
                y+=command.1;
            },
            'U' => {
                y-=command.1;
            },
            _  => println!("ERROR!")
       }
       i+=1;
    }
    println!("integral: {}", integral);
}