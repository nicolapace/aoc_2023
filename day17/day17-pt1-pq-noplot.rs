use std::fs;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct HeapData {
    heatloss: u32,
    pos: (usize, usize, (u8,u8))
}

impl Ord for HeapData {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.heatloss).cmp(&self.heatloss)
    }
}
impl PartialOrd for HeapData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn step( map: &Vec<Vec<u32>> , paths : &mut BinaryHeap<HeapData>) {

    let mut val_found :u32 = 0xffffffff;
    let mut visited : HashMap<(usize, usize, (u8,u8)), u32> = HashMap::new();
    
    loop {
        
        let mut new_paths :BinaryHeap<HeapData> = BinaryHeap::new();

        while let Some(path) = paths.pop() {
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
                    
            if x==map[0].len()-1 && y==map.len()-1 && path.heatloss < val_found {
                val_found = path.heatloss;
                continue;
            }

            if direction.0 == 0 {  // >

                if x+1< map[0].len() && direction.1 < 3 {    
                    new_paths.push(HeapData {
                        pos:        (x+1, y, (direction.0, direction.1+1)),
                        heatloss:   heat_loss+map[y][x+1]
                    });
                }
                if y+1<map.len() {
                    new_paths.push(HeapData {
                        pos:        (x, y+1, (2,1)),
                        heatloss:   heat_loss+map[y+1][x]
                    });
                }
                if y>0 {
                    new_paths.push(HeapData {
                        pos:        (x, y-1, (3,1)),
                        heatloss:   heat_loss+map[y-1][x]
                    });
                } 
        
            } else if direction.0 == 1 { // <        
        
        
                if y+1<map.len() {
                    new_paths.push(HeapData {
                        pos:        (x, y+1, (2,1)),
                        heatloss:   heat_loss+map[y+1][x]
                    });
                }
                if y>0 {
                    new_paths.push(HeapData {
                        pos:        (x, y-1, (3,1)),
                        heatloss:   heat_loss+map[y-1][x]
                    });
                }        
                if x>0 && direction.1 < 3 {
                    new_paths.push(HeapData {
                        pos:        (x-1, y, (direction.0, direction.1+1)),
                        heatloss:   heat_loss+map[y][x-1]
                    });
                }
        
            } else if direction.0 == 2 { // v
                
                if y+1< map.len() && direction.1 < 3  {
                    new_paths.push(HeapData {
                        pos:         (x, y+1, (direction.0, direction.1+1)),
                        heatloss:    heat_loss+map[y+1][x]
                    });
                }
                if x+1<map[0].len() {
                    new_paths.push(HeapData {
                        pos:         (x+1, y, (0, 1)),
                        heatloss:    heat_loss+map[y][x+1]
                    });
                }
                if x>0 {
                    new_paths.push(HeapData {
                        pos:        (x-1, y, (1, 1)),
                        heatloss:   heat_loss+map[y][x-1]
                    });
                }
        
            } else /*if direction == 3*/ { // ^
                    
                if x+1<map[0].len() {
                    new_paths.push(HeapData {
                        pos:         (x+1 , y , (0,1)),
                        heatloss:    heat_loss+map[y][x+1]
                    });
                }
                if x>0 {
                    new_paths.push(HeapData {
                        pos:         (x-1 , y , (1,1)),
                        heatloss:    heat_loss+map[y][x-1]
                    });
                }
                if y>0 && direction.1 < 3 {
                    new_paths.push(HeapData {
                        pos:         (x , y-1 , (direction.0, direction.1+1)),
                        heatloss:    heat_loss+map[y-1][x]
                    });
                }
        
            }
        }
       
        *paths = new_paths;
        if paths.len() == 0 {
            break;
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
    
    let mut paths :BinaryHeap<HeapData> = BinaryHeap::new();

    paths.push(HeapData {
        pos: (1,0, (0,1)),
        heatloss: map[0][1]
    });

    paths.push( HeapData {
        pos: (0,1, (2,1)),
        heatloss: map[1][0]
    });

    step( &map, &mut paths );
}