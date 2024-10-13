// Sorting Algorithms
// 1.selection sort only for i32
// 2.quicksort only for i32 (13/10/2024)
// comment for test
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

    selection_sort(&mut vect);
    let n = vect.len();
    quicksort(&mut vect, 0, n - 1);

    // Print elements of vector
    println!("Elements of sorted vector: {:?}", vect);
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

fn get_pivot (vect: &mut Vec<i32>, low: usize, high: usize) -> usize {
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
}

fn quicksort (vect: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pivot = get_pivot(vect, low, high);

        quicksort(vect, low, pivot - 1);
        quicksort(vect, pivot + 1, high);
    }
}