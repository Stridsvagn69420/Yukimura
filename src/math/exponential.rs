use std::iter;

/// nth Fibonacci number
/// 
/// Gets the Fibonacci number in set place
pub fn fib(n: u32) -> Result<u32, ()> {
    if n <= 0 {
        Err(())
    } else {
        Ok(iter::once(0) // Creates iterator of ints starting from 0
            .cycle() // Unlimited numbers
            .take(n as usize) // Only takes the required range
            .fold( // Same as below, but only returns the final result
                (0, 1),
                |(p, i), _| (p + i, p)
            )
            .0) // Returns first item of tuple
    }
}

/// List to nth Fibonacci number
/// 
/// Gets all Fibonacci numbers to set place
pub fn fiblist(n: u32) -> Result<Vec<u32>, ()> {
    if n <= 0 {
        Err(())
    } else {
        Ok(iter::once(0) // Creates iterator of ints starting from 0
            .cycle() // Unlimited
            .take(n as usize) // Only takes the required range
            .scan((0, 1), |(p, i), _| { // Saves the previous calculated numbers to get an array
                let x = *p;
                *p += *i;
                *i = x;
                Some((*p, *i))
            })
            .map(|(i, _)| i) // Only get the first item of tuple
            .collect::<Vec<_>>()) // Collect it as a Vector
    }
}
