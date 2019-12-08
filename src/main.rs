fn traverse_array1() {
    let mut sample: [[i32; 1024]; 1024] = [[0; 1024]; 1024];
    for i in 0..1024 {
        for j in 0..1024 {
            sample[i][j] += 1;
        }
    }
}

fn traverse_array2() {
    let mut sample: [[i32; 1024]; 1024] = [[0; 1024]; 1024];
    for i in 0..1024 {
        for j in 0..1024 {
            sample[j][i] += 1;
        }
    }
}

fn main() {
    traverse_array1();
    traverse_array2();
    println!("Hello, cache!");
}
