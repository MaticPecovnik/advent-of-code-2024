use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_input_file(filename: &str) -> io::Result<File> {
    // Get the current working directory of the program
    let current_dir = env::current_dir()?;

    // Construct the path to input.txt (assuming it's at the same level as 'src')
    let input_path = current_dir.join(filename);

    // Check if the file exists
    if !input_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "input.txt not found",
        ));
    }

    // Open the file and wrap it in a buffered reader
    let file = File::open(input_path)?;
    Ok(file)
}

fn read_vectors_from_file(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = read_input_file(filename)?;
    let reader = BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?; // Get the line from the iterator
        let parts: Vec<&str> = line.split_whitespace().collect(); // Split the line into parts

        // Ensure the line has exactly two parts
        if parts.len() == 2 {
            // Try to parse both parts as i32
            match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                (Ok(first), Ok(second)) => {
                    left.push(first);
                    right.push(second);
                }
                _ => {
                    eprintln!("Skipping invalid line: {}", line); // Handle invalid lines
                }
            }
        } else {
            eprintln!("Skipping line with invalid format: {}", line);
        }
    }

    Ok((left, right))
}

fn get_occurence_mapping(vector: &Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();

    for el in vector.iter() {
        let freq: &mut i32 = map.entry(*el).or_insert(0);
        *freq += 1;
    }
    map
}

fn main() {
    let mut sum_distances: i32 = 0;
    let mut similarity_score: i32 = 0;

    match read_vectors_from_file("input.txt") {
        Ok((mut left, mut right)) => {
            // Now you have `left` and `right` vectors, you can use them
            left.sort();
            right.sort();

            let occurances = get_occurence_mapping(&right);

            for (l, r) in left.iter().zip(right.iter()) {
                if l >= r {
                    sum_distances += l - r;
                } else {
                    sum_distances += r - l;
                }

                match occurances.get(&l) {
                    Some(freq_in_right) => similarity_score += l * freq_in_right,
                    None => (),
                }
            }

            println!("Got sum of distances: {}", sum_distances);
            println!("Got similarity score: {}", similarity_score);
        }
        Err(e) => {
            eprintln!("Error reading vectors from file: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_vectors_from_file() {
        match read_vectors_from_file("input.txt") {
            Ok((left, right)) => {
                assert_eq!(left.len(), right.len());
                assert_eq!(left.len(), 1000);
            }
            Err(e) => {
                panic!("Error reading file: {}", e);
            }
        }
    }
}
