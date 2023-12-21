use std::fs;

fn main() {
    let mut x : i32 = 0;
    let mut min_x : i32 = 99999;
    let mut max_x : i32 = -99999;
    let mut y : i32 = 0;
    let mut min_y : i32 = 99999;
    let mut max_y : i32 = -99999;
    let mut data : Vec<(char, usize)> = vec![];

    for line in fs::read_to_string("test").expect("No file\n").split("\n"){

        let segment = line.split_whitespace().collect::<Vec<_>>();
        let mut num :usize = 0;
        let mut k :usize = 1;
        for i in 0..segment[1].bytes().count() {
            num += k*(segment[1].bytes().nth( segment[1].chars().count()-1-i ).unwrap() - b'0') as usize ;
            k*=10;
        }
        match segment[0] {
            "R" => {
                x+=num as i32;
                if x > max_x {
                    max_x = x;
                }
                if x < min_x {
                    min_x = x;
                }
            },
            "L" => {
                x-=num as i32;
                if x > max_x {
                    max_x = x;
                }
                if x < min_x {
                    min_x = x;
                }
            },
            "U" => {
                y-=num as i32;
                if y > max_y {
                    max_y = y;
                }
                if y < min_y {
                    min_y = y;
                }
            },
            "D" => {
                y+=num as i32;
                if y > max_y {
                    max_y = y;
                }
                if y < min_y {
                    min_y = y;
                }
            },
             _  => println!("ERROR!")
        }
        
        data.push((segment[0].chars().nth(0).unwrap(), num));
    }
    println!("min x {}  max x {}", min_x, max_x);
    println!("min y {}  max y {}", min_y, max_y);

    let mut map : Vec<Vec<char>> = vec![vec!['.'; (1+max_x-min_x) as usize]; (1+max_y-min_y) as usize];

    let mut x = -min_x as usize;
    let mut y = -min_y as usize;
    map[y][x] = '#';
    for command in data{

       match command.0 {
        'R' => {       
            for i in x+1..=x+command.1 {
                map[y][i] = '#';
            }
            x+=command.1;
        },
        'L' => {
            for i in x-command.1..x {
                map[y][i] = '#';
            }
            x-=command.1;
        },
        'D' => {
            for j in y+1..=y+command.1 {
                map[j][x] = '#';
            }
            y+=command.1;
        },
        'U' => {
            for j in y-command.1..y {
                map[j][x] = '#';
            }
            y-=command.1;
        },
         _  => println!("ERROR!")
       }
    }
    let mut mapcopy= map.clone();
    for j in 0..map.len() {
        let mut i = 0;
        let mut fill = false;

        loop {
            while i<map[j].len()-1 && map[j][i] != '#'  {
                if fill {
                    mapcopy[j][i] = '#'
                }
                i+=1;
            }
            let mut interval = 0;
            while i+interval<map[j].len()-1 && map[j][i+interval] == '#'  {
                interval+=1;
            }
            if i+interval==map[j].len()-1 {
                break;
            }
            if interval == 1 || !( (j != 0 && map[j-1][i] == '#' && map[j-1][i+interval-1] == '#') || ( j != map.len()-1 && map[j+1][i] == '#' && map[j+1][i+interval-1] == '#') ){
                fill = !fill;
            }
            i+=interval;
        }
        
        // println!("{:?}", mapcopy[j]);
    }

    let mut sum=0;
    for el in mapcopy {
        for e in el {
            if e == '#' {
                sum+=1;
            }
        }
    }

    println!("sum: {}", sum);   
}