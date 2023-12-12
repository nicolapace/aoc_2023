use std::fs;

fn manhattan_distance(point1 :& Vec<usize> , point2 :& Vec<usize>) -> usize {
    let mut distance = 0;
    if point1[0]>point2[0] {
        distance += point1[0]-point2[0];
    }else{
        distance += point2[0]-point1[0];
    }
    if point1[1]>point2[1] {
        distance += point1[1]-point2[1];
    }else{
        distance += point2[1]-point1[1];
    }   
    return distance;
}

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n").collect::<Vec<_>>();
    
    let ssize = lines.len();
    let len = lines[0].bytes().count();
    let mut matrix : Vec<_> = vec![];
    
    println!("size x: {:?}", len);
    println!("size y: {:?}", ssize);

    let mut rows_occupation : Vec<_> = vec![0; ssize];
    let mut cols_occupation : Vec<_> = vec![0; len];
    let mut galaxies : Vec<_> = vec![];

    /* finding galaxies */
    for j in 0..ssize {
        matrix.push(vec![0; len]);
        for i in 0..len{
            if lines[j].chars().nth(i).unwrap() == '#' {
                matrix[j][i] = 1;
                rows_occupation[j] = 1;
                cols_occupation[i] = 1;
                galaxies.push(vec![i,j]);
            }
        }
    }
    println!("rows_occupation: {:?}",rows_occupation);
    println!("cols_occupation: {:?}",cols_occupation);
    println!("galaxies: {:?}", galaxies);
    
    /* calculating distances */
    let mut sum = 0;
    for i in 0..galaxies.len()-1 {
        for k in i+1..galaxies.len(){
            
            let mut num_free_space = 0;
            if galaxies[i][0]>galaxies[k][0] {
                for x in galaxies[k][0]+1..galaxies[i][0] {
                    if cols_occupation[x]==0 {
                        num_free_space+=1;
                    }
                }
            } else if galaxies[i][0]<galaxies[k][0] {
                for x in galaxies[i][0]+1..galaxies[k][0] {
                    if cols_occupation[x]==0 {
                        num_free_space+=1;
                    }
                }
            }

            if galaxies[i][1]>galaxies[k][1] {
                for x in galaxies[k][1]+1..galaxies[i][1] {
                    if rows_occupation[x]==0 {
                        num_free_space+=1;
                    }
                }
            } else if galaxies[i][1]<galaxies[k][1] {
                for x in galaxies[i][1]+1..galaxies[k][1] {
                    if rows_occupation[x]==0 {
                        num_free_space+=1;
                    }
                }
            }

            sum += manhattan_distance(&galaxies[i],&galaxies[k]) + num_free_space*999999 ;
        }
    }
    println!("sum: {:?}", sum);
}
