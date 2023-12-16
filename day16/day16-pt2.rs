use std::fs;
use std::collections::HashSet;

fn do_step( x : usize , y: usize , direction : u8 , map : &Vec<Vec<char>> , path : &mut HashSet<(usize,usize,u8)>, energized_map :&mut HashSet<(usize,usize)>)  {

    if path.get(&(x,y,direction)) != None {
        return;
    }
    path.insert((x,y,direction));
    energized_map.insert((x,y));
    
    if direction == 0 {  
        match map[y][x] {
            '|'  => {if y+1<map.len()      {do_step(x,y+1,2, &map, path, energized_map)} ; if y>0 {do_step(x,y-1,3, &map, path, energized_map)} },
            '\\' => {if y+1<map.len()      {do_step(x,y+1,2, &map, path, energized_map)} },
            '/'  => {if y>0                {do_step(x,y-1,3, &map, path, energized_map)} },
             _   => {if x+1<map[0].len()   {do_step(x+1,y,0, &map, path, energized_map)} },
        }  
    } else if direction == 1 {  
        match map[y][x] {  
            '|'  => {if y+1<map.len()      {do_step(x,y+1,2, &map, path, energized_map)}; if y>0 {do_step(x,y-1,3, &map, path, energized_map)} },
            '\\' => {if y>0                {do_step(x,y-1,3, &map, path, energized_map)} },
            '/'  => {if y+1<map.len()      {do_step(x,y+1,2, &map, path, energized_map)} },
             _   => {if x>0                {do_step(x-1,y,1, &map, path, energized_map)} },
        }  
    } else if direction == 2 {  
        match map[y][x] {  
            '\\' => {if x+1<map[0].len()   {do_step(x+1,y,0, &map, path, energized_map)} },
            '/'  => {if x>0                {do_step(x-1,y,1, &map, path, energized_map)} },
            '-'  => {if x+1<map[0].len()   {do_step(x+1,y,0, &map, path, energized_map)} ; if x>0 {do_step(x-1,y,1, &map, path, energized_map)} },
             _   => {if y+1<map.len()      {do_step(x,y+1,2, &map, path, energized_map)} },
        }  
    } else /*if direction == 3*/ {  
        match map[y][x] {  
            '\\' => {if x>0                {do_step(x-1,y,1, &map, path, energized_map)} },
            '/'  => {if x+1<map[0].len()   {do_step(x+1,y,0, &map, path, energized_map)} },
            '-'  => {if x+1<map[0].len()   {do_step(x+1,y,0, &map, path, energized_map)} ; if x>0 {do_step(x-1,y,1, &map, path, energized_map)} },
             _   => {if y>0                {do_step(x,y-1,3, &map, path, energized_map)} },
        }
    }

}

fn main() {
    let mut map :Vec<Vec<char>> = vec![];
    for line in fs::read_to_string("input").expect("No file\n").split("\n"){
        map.push(line.chars().collect::<Vec<_>>());
    }

    let mut max = 0;

    for j in 0..map[0].len() {
        let mut path :HashSet<(usize,usize,u8)> = HashSet::new();
        let mut energized_map :HashSet<(usize,usize)> = HashSet::new();   
        do_step(0,j,0,&map,&mut path, &mut energized_map);
        if energized_map.len() > max {
            max = energized_map.len();
        }

        let mut path :HashSet<(usize,usize,u8)> = HashSet::new();
        let mut energized_map :HashSet<(usize,usize)> = HashSet::new();   
        do_step(map[0].len()-1,j,1 ,&map, &mut path, &mut energized_map);
        if energized_map.len() > max {
            max = energized_map.len();
        }
    }
  
    for i in 0..map.len() {
        let mut path :HashSet<(usize,usize,u8)> = HashSet::new();
        let mut energized_map :HashSet<(usize,usize)> = HashSet::new();   
        do_step(i,0,2,&map, &mut path, &mut energized_map);
        if energized_map.len() > max {
            max = energized_map.len();
        }

        let mut path :HashSet<(usize,usize,u8)> = HashSet::new();
        let mut energized_map :HashSet<(usize,usize)> = HashSet::new();   
        do_step(i,map.len()-1,3,&map, &mut path, &mut energized_map);
        if energized_map.len() > max {
            max = energized_map.len();
        }
    }

    println!("max: {:?}", max);
   
   
}