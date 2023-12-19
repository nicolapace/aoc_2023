use std::fs;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone)]
struct Data {
    heatloss: u32,
    pos: (usize, usize, (u8,u8))
}

fn step( map: &Vec<Vec<u32>> , paths : &mut Vec<Data>) {
    
    let mut val_found :u32 = 999999;
    let mut visited : HashMap<(usize, usize, (u8,u8)), u32> = HashMap::new();
    
   while !paths.is_empty() {
        
        let mut new_paths :Vec<Data> = vec![];

        for path in &mut *paths {
            let direction = path.pos.2;
            let x = path.pos.0;
            let y = path.pos.1;
            let heat_loss = path.heatloss;

            if x==0 && y==0 {
                continue;
            }

            let res = visited.get(&(x,y, direction));
            if res != None  {
                if heat_loss < *res.unwrap()  {
                    visited.insert((x,y, direction), heat_loss);
                }else{
                    continue;
                }
            }else {
                visited.insert((x,y, direction), heat_loss);
            }

            //path.path.insert((x,y));
                    
            if x==map[0].len()-1 && y==map.len()-1 && path.heatloss < val_found {
                val_found =  path.heatloss;
                continue;
            }

            if direction.0 == 0 {  // >

                if x+1< map[0].len() && direction.1 < 3 {  
                    new_paths.push(Data {
                        pos:        (x+1, y, (direction.0, direction.1+1)),
                        heatloss:   heat_loss+map[y][x+1]
                    });
                }
                if y+1<map.len() {
                    new_paths.push(Data {
                        pos:        (x, y+1, (2,1)),
                        heatloss:   heat_loss+map[y+1][x]
                    });
                }
                if y>0 {
                    new_paths.push(Data {
                        pos:        (x, y-1, (3,1)),
                        heatloss:   heat_loss+map[y-1][x]
                    });
                } 
        
            } else if direction.0 == 1 { // <        
        
        
                if y+1<map.len() {
                    new_paths.push(Data {
                        pos:        (x, y+1, (2,1)),
                        heatloss:   heat_loss+map[y+1][x]
                    });
                }
                if y>0 {
                    new_paths.push(Data {
                        pos:        (x, y-1, (3,1)),
                        heatloss:   heat_loss+map[y-1][x]
                    });
                }        
                if x>0 && direction.1 < 3 {
                    new_paths.push(Data {
                        pos:        (x-1, y, (direction.0, direction.1+1)),
                        heatloss:   heat_loss+map[y][x-1]
                    });
                }
        
            } else if direction.0 == 2 { // v
                
                if y+1< map.len() && direction.1 < 3  {
                    new_paths.push(Data {
                        pos:         (x, y+1, (direction.0, direction.1+1)),
                        heatloss:    heat_loss+map[y+1][x]
                    });
                }
                if x+1<map[0].len() {
                    new_paths.push(Data {
                        pos:         (x+1, y, (0, 1)),
                        heatloss:    heat_loss+map[y][x+1]
                    });
                }
                if x>0 {
                    new_paths.push( Data {
                        pos:        (x-1, y, (1, 1)),
                        heatloss:   heat_loss+map[y][x-1]
                    });
                }
        
            } else /*if direction == 3*/ { // ^
                    
                if x+1<map[0].len() {
                    new_paths.push(Data {
                        pos:         (x+1 , y , (0,1)),
                        heatloss:    heat_loss+map[y][x+1]
                    });
                }
                if x>0 {
                    new_paths.push(Data {
                        pos:         (x-1 , y , (1,1)),
                        heatloss:    heat_loss+map[y][x-1]
                    });
                }
                if y>0 && direction.1 < 3 {
                    new_paths.push(Data {
                        pos:         (x , y-1 , (direction.0, direction.1+1)),
                        heatloss:    heat_loss+map[y-1][x]
                    });
                }
        
            }
        }
       
        new_paths.sort_by(|a,b| (a.heatloss).cmp(&b.heatloss));
        // let len = map.len()+map[0].len()-2;
        // new_paths.sort_by( |a,b| (a.heatloss+ 3*(len-a.pos.0-a.pos.1) as u32 +5*map[a.pos.1][a.pos.0] ).cmp(&(b.heatloss + 3*(len-b.pos.0-b.pos.1) as u32 +5*map[b.pos.1][b.pos.0] ) ) );
        *paths = new_paths;
    }

    println!("\nsum: {}", val_found); 
}

fn main() {
    let mut map :Vec<Vec<u32>> = vec![];
    for line in fs::read_to_string("input").expect("No file\n").split("\n"){
        let mut num_vec : Vec<u32> = vec![];
        for num in line.bytes().collect::<Vec<_>>(){
            num_vec.push( (num - b'0') as u32);        }

        map.push(num_vec);
    }
    
    let mut paths :Vec<Data> = vec![];

    paths.push(Data {
        pos: (1,0, (0,1)),
        heatloss: map[0][1]
    });

    paths.push(Data {
        pos: (0,1, (2,1)),
        heatloss: map[1][0]
    });

    step( &map, &mut paths );
}