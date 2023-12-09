use std::fs;

fn main() {
    let start_time = std::time::Instant::now();
    let is_test = false;
    let content = get_content(is_test);
    let sum = content
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let mut final_numbers = Vec::new();
            let mut queue = Vec::new();
            queue.push(numbers.clone());
            final_numbers.push(numbers);

            loop {
                let cur_nums = queue.pop().unwrap();
                if cur_nums.iter().all(|&x| x == 0) {
                    break;
                }
                let mut next_nums = Vec::new();
                for i in 0..cur_nums.len() - 1 {
                    let prev = cur_nums[i];
                    let next = cur_nums[i + 1];
                    let diff = derivative(prev, next);
                    next_nums.push(diff);
                }
                queue.push(next_nums.clone());
                final_numbers.push(next_nums);
            }
            final_numbers.pop().unwrap(); // lazy way to remove the last 0s

            let mut sum = 0;
            while let Some(nums) = final_numbers.pop() {
                let last = nums.last().unwrap();
                sum += last;
            }

            sum
        })
        .sum::<i32>();

    println!("Sum: {}", sum);
    println!("Run time: {:?}", start_time.elapsed());
}

fn derivative(prev: i32, next: i32) -> i32 {
    next - prev
}

fn get_content(is_test: bool) -> String {
    if is_test {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .to_string()
    } else {
        let filename = "input.txt";
        fs::read_to_string(filename).expect("Something went wrong reading the file")
    }
}
