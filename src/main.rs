fn main() {
    using_iter_on_vec();
    using_map_and_collect();
    using_closure_with_borrow();
    using_closure_with_mut_borrow();
    using_closure_with_ownership();
    using_fn_with_closure_input(|str| println!("{}", str));
    using_filter();
    using_map_and_filter();
    using_sum_to_consume();
    using_fold_to_accumulate();
    using_into_iter_to_take_ownership();
    using_iter_mut_for_mutation();
    using_enumerate_to_index();
    using_custom_iterator();
}

// iterating through a vector
fn using_iter_on_vec() {
    let some_vec = vec!["a", "b", "c"];
    let my_iterator = some_vec.iter();
    for val in my_iterator {
        println!("Value: {}", val);
    }
}

// mapping and collecting iterms
fn using_map_and_collect() {
    let v = vec![0, 1, 2];
    let incremented: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", incremented);
}

// making a simple closure which captures it's environment by borrowing
fn using_closure_with_borrow() {
    let x = 10;
    let my_closure = || println!("x is {}", x);
    my_closure();
}

// making a closure which mutably borrows a value from it's environment
fn using_closure_with_mut_borrow() {
    let mut x = 10;
    let mut increment_x = |y| x += y;
    increment_x(1);
    println!("x is {}", x);
}

// making a closure take ownership of it's environment
fn using_closure_with_ownership() {
    let x = String::from("I am owned!");
    let take_x = move || println!("{}", x);
    take_x();
    // println!("{}", x); // Error: `x` has been moved
}

// making a function which uses a closure as input
fn using_fn_with_closure_input<F>(func: F) where F: Fn(String) {
    func(String::from("I am injected into the closure!"));
}

// filtering elements out using filter
fn using_filter() {
    let nums = vec![1,2,3,4,5,6,7,8,9,10];
    let even: Vec<_> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{:?}", even);
}

// combining methods like map and filter
fn using_map_and_filter() {
    let nums = vec![1,2,3,4,5,6];
    let doubled_even: Vec<_> = nums
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();
    println!("{:?}", doubled_even);
}

// consuming an iterator using sum
fn using_sum_to_consume() {
    let nums = vec![1,2,3,4];
    let total: i32 = nums.iter().sum();
    println!("{:?}", total);
}

// chaining operations with fold and building accumulitive values
fn using_fold_to_accumulate() {
    let nums = vec![1,2,3,4,5];
    let sum = nums.iter().fold(0, |acc, &x| acc + x);
    println!("{:?}", sum);
}

// using into_iter to consume and take ownership of a collection
fn using_into_iter_to_take_ownership() {
    let nums = vec![1,2,3,4,5];
    for num in nums.into_iter() {
        println!("{}", num);
    }
    // nums is no longer valid
}

// using iter_mut to mutate values in place
fn using_iter_mut_for_mutation() {
    let mut nums = vec![1,2,3,4];
    for num in nums.iter_mut() {
        *num = *num + 1;
    }
    println!("{:?}", nums);
}

// using enumerate to add an index to each iter_mut
fn using_enumerate_to_index() {
    let colors = vec!["red", "green", "blue"];
    for (i, color) in colors.iter().enumerate() {
        println!("{}: {}", i, color);
    }
}

struct TimeBomb {
    count: u32,
    limit: u32,
}

impl Iterator for TimeBomb {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.limit {
            Some(self.count)
        } else {
            println!("💥");
            None
        }
    }
}

fn using_custom_iterator() {
    let mut tb = TimeBomb{
        count: 0,
        limit: 10,
    };
    while let Some(count) = tb.next() {
        println!("{}", count);
    }
}
