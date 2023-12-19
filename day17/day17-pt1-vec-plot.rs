use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone)]
struct Data {
    heatloss: u32,
    pos: (usize, usize, (u8,u8)),
    path: HashSet<(usize, usize)>
}

fn step( map: &Vec<Vec<u32>> , paths : &mut Vec<Data>) {
    
    let mut found :HashSet<(usize, usize)> = HashSet::new();
    let mut val_found :u32 = 0xffffffff;
    let mut visited : HashMap<(usize, usize, (u8,u8)), u32> = HashMap::new();
    
    while !paths.is_empty() {
    
        println!("priority queue size: {}, best:{}", paths.len(), paths[0].heatloss);
            
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

            path.path.insert((x,y));
                    
            if x==map[0].len()-1 && y==map.len()-1 && path.heatloss < val_found {
                found = path.path.clone();
                val_found =  path.heatloss;
                continue;
            }

            if direction.0 == 0 {  // >

                if x+1< map[0].len() && direction.1 < 3 {  
                    new_paths.push(Data {
                        pos:        (x+1, y, (direction.0, direction.1+1)),
                        heatloss:   heat_loss+map[y][x+1],
                        path: path.path.clone()
                    });
                }
                if y+1<map.len() {
                    new_paths.push(Data {
                        pos:        (x, y+1, (2,1)),
                        heatloss:   heat_loss+map[y+1][x],
                        path: path.path.clone()
                    });
                }
                if y>0 {
                    new_paths.push(Data {
                        pos:        (x, y-1, (3,1)),
                        heatloss:   heat_loss+map[y-1][x],
                        path: path.path.clone()
                    });
                } 
        
            } else if direction.0 == 1 { // <        
        
        
                if y+1<map.len() {
                    new_paths.push(Data {
                        pos:        (x, y+1, (2,1)),
                        heatloss:   heat_loss+map[y+1][x],
                        path: path.path.clone()
                    });
                }
                if y>0 {
                    new_paths.push(Data {
                        pos:        (x, y-1, (3,1)),
                        heatloss:   heat_loss+map[y-1][x],
                        path: path.path.clone()
                    });
                }        
                if x>0 && direction.1 < 3 {
                    new_paths.push(Data {
                        pos:        (x-1, y, (direction.0, direction.1+1)),
                        heatloss:   heat_loss+map[y][x-1],
                        path: path.path.clone()
                    });
                }
        
            } else if direction.0 == 2 { // v
                
                if y+1< map.len() && direction.1 < 3  {
                    new_paths.push(Data {
                        pos:         (x, y+1, (direction.0, direction.1+1)),
                        heatloss:    heat_loss+map[y+1][x],
                        path: path.path.clone()
                    });
                }
                if x+1<map[0].len() {
                    new_paths.push(Data {
                        pos:         (x+1, y, (0, 1)),
                        heatloss:    heat_loss+map[y][x+1],
                        path: path.path.clone()
                    });
                }
                if x>0 {
                    new_paths.push( Data {
                        pos:        (x-1, y, (1, 1)),
                        heatloss:   heat_loss+map[y][x-1],
                        path: path.path.clone()
                    });
                }
        
            } else /*if direction == 3*/ { // ^
                    
                if x+1<map[0].len() {
                    new_paths.push(Data {
                        pos:         (x+1 , y , (0,1)),
                        heatloss:    heat_loss+map[y][x+1],
                        path: path.path.clone()
                    });
                }
                if x>0 {
                    new_paths.push(Data {
                        pos:         (x-1 , y , (1,1)),
                        heatloss:    heat_loss+map[y][x-1],
                        path: path.path.clone()
                    });
                }
                if y>0 && direction.1 < 3 {
                    new_paths.push(Data {
                        pos:         (x , y-1 , (direction.0, direction.1+1)),
                        heatloss:    heat_loss+map[y-1][x],
                        path: path.path.clone()
                    });
                }
        
            }
        }
                
        new_paths.sort_by(|a,b| (a.heatloss).cmp(&b.heatloss));
        *paths = new_paths;
    }

    
    println!("\nsum: {}", val_found);
    for j in 0..map.len() {
        println!();
        for i in 0..map[j].len(){
            if found.get(&(i,j)) != None ||  i==map[0].len()-1 && j==map.len()-1 {
                print!("{}",map[j][i]);
            }else {
                print!(".");
            }
        }
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

    let mut init_path :HashSet<(usize, usize)> = HashSet::new();
    init_path.insert((1,0));
    paths.push(Data {
        pos: (1,0, (0,1)),
        heatloss: map[0][1],
        path: init_path
    });

    let mut init_path :HashSet<(usize, usize)> = HashSet::new();
    init_path.insert((0,1));
    paths.push(Data {
        pos: (0,1, (2,1)),
        heatloss: map[1][0],
        path: init_path
    });

    step( &map, &mut paths );
}