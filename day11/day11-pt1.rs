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
    
    let mut ssize = lines.len();
    let len = lines[0].bytes().count();
    let mut matrix : Vec<_> = vec![];
    
    println!("size x: {:?}", len);
    println!("size y: {:?}", ssize);

    
    /* expanding universe */
    let mut cols_occupation : Vec<_> = vec![0; len];
    let mut diff=0;
    
    for j in 0..ssize {
        let mut row_empty = true;
        matrix.push(vec![0; len]);
        for i in 0..len{
            if lines[j].chars().nth(i).unwrap() == '#' {
                row_empty = false;
                matrix[j+diff][i] = 1;
                cols_occupation[i] = 1;
            }
        }
        println!("{:?}",matrix[j]);
        if row_empty {
            matrix.push(vec![0; len]);
            diff+=1;
        }
    }
    println!("cols_occupation: {:?}",cols_occupation);
    let mut diff = 0;
    for i in 0..matrix[0].len() {
        if cols_occupation[i]==0 {
            for row in &mut matrix {
                row.insert(i+diff,0);
            }
            diff+=1;
        }
    }
    println!("expanded size x: {:?}", matrix[0].len());
    println!("expanded size y: {:?}", matrix.len());
    for l in &matrix {
        println!("{:?}",l);
    }
    
    /* finding galaxies */
    let mut galaxies : Vec<_> = vec![];
    for j in 0..matrix.len() {
        for i in 0..matrix[0].len(){
            if matrix[j][i] == 1 {
                galaxies.push(vec![i,j]);
            }
        }
    }
    println!("galaxies: {:?}", galaxies);
    
    let mut sum = 0;
    /* calculating distances */
    for i in 0..galaxies.len()-1 {
        for k in i+1..galaxies.len(){
            println!("distance:  {:?} - {:?} -> {:?} - {:?} = {:?}", i+1, k+1, &galaxies[i],&galaxies[k],manhattan_distance(&galaxies[i],&galaxies[k]));
            sum+=manhattan_distance(&galaxies[i],&galaxies[k]);
        }
    }
    println!("sum: {:?}", sum);
}
