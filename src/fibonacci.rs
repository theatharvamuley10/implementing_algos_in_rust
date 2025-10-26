pub fn find_nth_fibonacci_recursion(n: u64) -> u64 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        find_nth_fibonacci_recursion(n - 1) + find_nth_fibonacci_recursion(n - 2)
    }
}

pub fn find_nth_fibonacci_loop(mut n: u64) -> u64 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        let mut last = 1;
        let mut second_last = 0;
        let mut num = 0;

        while n > 2 {
            num = last + second_last;
            second_last = last;
            last = num;
            n -= 1;
        }

        num
    }
}

pub fn print_fibonacci() {
    let mut n = 3;
    let mut last = 1;
    let mut second_last = 0;
    let mut num = 0;

    print!("{second_last} ");
    print!("{last} ");

    while (2 < n) & (n < 11) {
        num = last + second_last;
        second_last = last;
        last = num;
        n += 1;
        print!("{num} ")
    }
}
