use std::fs;
// use std::collections::HashMap;
// use std::collections::HashSet;

fn do_step( pos: (usize,usize) , map: &mut Vec<Vec<char>>, longest_path :&mut u32 ) -> bool {
    if pos==(map[0].len()-2,map.len()-1) {
        let mut steps = 0;
        //println!();
        for el in map.clone() {
            for c in el{
                if c == 'O' {
                    steps+=1;
                }
                //print!("{}", c);
            }
            //println!();
        }
        if steps>*longest_path {
            *longest_path=steps;
            println!("steps: {}", steps)
        };
        return false;
    }

    map[pos.1][pos.0] = 'O';

    if map[pos.1][pos.0]=='#' || map[pos.1][pos.0]=='0'{

        return false;

    } else if map[pos.1][pos.0]=='>'  {

        return do_step((pos.0+1,pos.1), &mut map.clone(), longest_path) ;

    }else if map[pos.1][pos.0]=='<'{

        return do_step((pos.0-1,pos.1), &mut map.clone(), longest_path) ;

    }else if map[pos.1][pos.0]=='v'{

        return do_step((pos.0,pos.1+1), &mut map.clone(), longest_path) ;

    }else if map[pos.1][pos.0]=='^'{

        return do_step((pos.0,pos.1-1), &mut map.clone(), longest_path) ;

    }else{
        //go up
        if pos.1!=0 && map[pos.1-1][pos.0]!='#' && map[pos.1-1][pos.0]!='O'  && map[pos.1-1][pos.0]!='v' {
            if do_step((pos.0,pos.1-1), &mut map.clone(), longest_path) {
                return true;
            }
        }
        //go left
        if pos.0!=0 && map[pos.1][pos.0-1]!='#' && map[pos.1][pos.0-1]!='O' && map[pos.1][pos.0-1]!='>' {
            if do_step((pos.0-1,pos.1), &mut map.clone(), longest_path) {
                return true;
            }
        }
        //go down
        if pos.1!=map.len() && map[pos.1+1][pos.0]!='#' && map[pos.1+1][pos.0]!='O' && map[pos.1][pos.0-1]!='^' {
            if do_step((pos.0,pos.1+1), &mut map.clone(), longest_path) {
                return true;
            }
        }
        //go right
        if pos.0!=map[0].len() && map[pos.1][pos.0+1]!='#' && map[pos.1][pos.0+1]!='O' && map[pos.1][pos.0+1]!='<' {
            if do_step((pos.0+1,pos.1), &mut map.clone(), longest_path){
                return true;
            }
        }
    } 

    return false;
}

fn main() {
   
    // DFS trying to go left or up before going down or right

    let mut map : Vec<Vec<char>> = vec![];
    for line in fs::read_to_string("input").expect("No file\n").split("\n") {
        map.push( line.chars().collect::<Vec<char>>() );
    }

    // println!();
    // for el in map.clone() {
    //     for c in el{
    //         print!("{}", c);
    //     }
    //     println!();
    // }
  
    do_step((1,0), &mut map, &mut 0);
}