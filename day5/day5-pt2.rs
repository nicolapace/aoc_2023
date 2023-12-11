use std::fs;
//use std::collections::HashSet;

fn main() {

    let contents = fs::read_to_string("input").expect("No file\n");
    let parts = contents.split("\n\n");
    let seeds : Vec<u64> = parts.clone().collect::<Vec<_>>()[0].split("seeds: ").nth(1).unwrap().split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect();
    println!("seeds: {:?}",seeds );

    /* PARSING */
    let mut connections = vec![];
    let mut i = 1;
    while i<parts.clone().count(){
        let rows = parts.clone().collect::<Vec<_>>()[i].split(":\n");
        
        let row =  rows.clone().collect::<Vec<_>>()[1].split("\n").collect::<Vec<_>>();

        let mut parsed_rows = vec![];
        for subrow in row {
            let temp =  subrow.split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
            parsed_rows.push(temp);
        }
        parsed_rows.sort_by_key(|v| v[1]) ;
        connections.push(parsed_rows);
        
        i+=1;
    }
    
    let mut min = 4294967295u64;
    let mut passaggi = 0;

    /* SEARCH ALGORITHM */
    let mut j = 0;
    while j < seeds.len(){
        let mut k = 0;
        while k < seeds[j+1]{
            print!("seed: {:?} ", seeds[j]+k);
            let mut comp = seeds[j]+k as u64;
            let mut jump = 4294967295u64;
            
            for conn in &connections {  
                
                //if connections[i][connections[i].len()-1][1]+connections[i][connections[i].len()-1][2] > comp {

                    let mut r = 0;
                    for row in conn {
                      
                        if row[1]<=comp && row[1]+row[2]>comp {

                            if row[1]+row[2]-comp < jump {
                                jump = row[1]+row[2]-comp ;
                            }
                            comp = row[0]+(comp-row[1]);

                            break;
                        }
                        else if row[1]>comp{
                            break;
                        }
                        
                        r+=1;
                        
                        

                    }
                //}

                print!(" -> {:?}", comp);
            }

            if comp<min {
                min=comp;
            }

            println!(" | jumping {:?}", jump);


            k+=jump;
            passaggi+=1;
        }
        println!();
        j+=2;
    }
    println!("result:  {:?} ", min);
    println!("passaggi:  {:?} ", passaggi);
}
