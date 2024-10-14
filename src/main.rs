// Sorting Algorithms
// 1.selection sort only for i32
// 2.quicksort only for i32 (13/10/2024)
// 3.binary search only for i32 (14/10/2024)
fn main() {
    
    let mut input = String::new();

    println!("Enter elements of vector, separated by spaces");
    
    // Read the input in string
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    // Parse the string
    let mut vect: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("not a valid integer!"))
        .collect();

    // print the vector
    println!("Elements of vector: {:?}", vect);

    // Selection sort
    selection_sort(&mut vect);
    println!("Elements of sorted vector: {:?}", vect);

    // Quicksort
    let n = vect.len() - 1;
    quicksort(&mut vect, 0, n);
    println!("Elements of sorted vector: {:?}", vect);

    // Binary search
    println!("Enter the key to search: ");
    input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    let key: i32 = input
        .trim()
        .parse::<i32>()
        .expect("not a valid integer!");

    println!("Enter the key to search: {}", key);
    println!("The key is found at index: {}", binary_search(&vect, key));
}

fn selection_sort (vect: &mut Vec<i32>) {
    for i in 0..(vect.len() - 1) {
        for j in (i + 1)..(vect.len()) {
            if vect[i] > vect[j] {
                vect.swap(i, j);
            }
        }
    }
}

fn quicksort (vect: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pivot = {
            let mut i = low;

            let pivot = match vect.get(high) {
                Some(v) => {v.clone()},
                _ => {panic!("Array index {:?} out of bounds", high)}
            };

            for j in  low..high {
                match vect.get(j) {
                    Some(v) => {
                        if v < &pivot {
                            vect.swap(i, j);
                            i += 1;
                        }
                    },
                    _ => {panic!("Array index {:?} for j out of bounds", j)}
                }
            }
            vect.swap(i, high);
            i
        };

        if low < pivot {
            quicksort(vect, low, pivot - 1);
        }
        quicksort(vect, pivot + 1, high);
    }
}

fn binary_search(vect: &Vec<i32>, key: i32) -> i32 {
    let mut low = 0;
    let mut high = vect.len() - 1;
    let mut mid = (high - low) / 2 + low;

    while low <= high {
        match vect[mid].cmp(&key) {
            std::cmp::Ordering::Equal => {return mid as i32},
            std::cmp::Ordering::Less => {low = mid + 1},
            std::cmp::Ordering::Greater => {high = mid - 1},
        }
        mid = (high - low) / 2 + low;
    }
    -1
}