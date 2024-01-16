

pub fn call() {
    println!("Using vector");

    let mut nums = vec![5, 4, 1, 10, 2, 6, 3];

    let first = &nums[0];
    println!("{}", first);

    nums.push(8);
    //println!("{}", first); The first ref `maybe` invalid after appending new element.
    //
    println!("{:?}", nums);

    nums.sort();
    println!("{:?}", nums);
}
