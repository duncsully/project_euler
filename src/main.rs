#![allow(dead_code)]

use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs, iter,
    thread::{self, JoinHandle},
};
mod utils;
use crate::utils::*;

fn main() {
    println!("{}", solver47());
}

/// Oof, getting back into things after 5 months. Thankfully this problem itself was relatively
/// simple. I just needed utilities for getting the distinct prime factors of a number. I'm not
/// entirely happy with having bolted it onto my newer Primes iterative struct, based on my older
/// PrimeNumbers struct that I felt got out of hand. I'm still trying to understand the best way
/// to handle essentially memoization and DP in Rust.
fn solver47() -> u64 {
    let mut consecutive = 0;
    let mut next = 646;
    let mut primes = Primes::new();
    while consecutive < 4 {
        next += 1;
        if primes.distinct_factors(next).len() == 4 {
            consecutive += 1;
        } else {
            consecutive = 0;
        }
    }
    (0..=3)
        .rev()
        .for_each(|d| println!("{}: {:?}", next - d, primes.distinct_factors(next - 3)));

    next - 3
}

/// I was expecting this one to be a lot worse than it actually was. This time Rust iterators
/// not only were handy for code's sake but for literally thinking through the problem iteratively.
/// I realized it'd be easier to keep track of a list of doubled squares and then simply check that
/// the differences for all of them are not prime.
fn solver46() -> u32 {
    let mut twice_squares = vec![2, 8, 18, 32];
    (35u32..)
        .step_by(2)
        .filter(|n| !n.is_prime())
        .find(|n| {
            if n > twice_squares.last().unwrap() {
                twice_squares.push((twice_squares.len().pow(2) * 2) as u32)
            }
            twice_squares
                .iter()
                .filter(|twice_square| *twice_square < n)
                .all(|twice_square| !(n - twice_square).is_prime())
        })
        .expect("Didn't find number")
}

/// So really this is only needing to check for the next pentagonal and hexagonal number
/// since all hexagonal numbers are triangular. I realize that there is a purely mathematical
/// solution that starts with setting the formulas equal to each other to find a set of solutions
/// to that equation, but my math isn't that strong anymore.
///
/// So brute force once again! I slightly optimized this (I think?) by iterating each series
/// only as needed until they both match. Parallelization might've been possible too but I
/// didn't feel like diving into a shared mutable value quite yet.
fn solver45() -> u32 {
    // Set up iterators and skip until past 40755
    let mut pent_add = 1;
    let mut pentagonal_num = 0;
    let mut pentagonal_nums = iter::from_fn(move || {
        pentagonal_num += pent_add;
        pent_add += 3;
        Some(pentagonal_num)
    })
    .skip(165);

    let mut hex_add = 1;
    let mut hexagonal_num = 0;
    let mut hexagonal_nums = iter::from_fn(move || {
        hexagonal_num += hex_add;
        hex_add += 4;
        Some(hexagonal_num)
    })
    .skip(143);

    let mut p = pentagonal_nums.next().unwrap();
    let mut h = hexagonal_nums.next().unwrap();
    while p != h {
        if p < h {
            p = pentagonal_nums.next().unwrap();
        } else {
            h = hexagonal_nums.next().unwrap();
        }
    }

    p
}

/// It's amazing what a difference using a set vs a vector does for search speeds. I suppose
/// I could've also used a binary search since the vectors should be sorted. Once again I
/// didn't have any mathematical basis for the number of numbers I generated. The approach
/// was otherwise a fairly standard brute force iteration through all of the pairs of the
/// arbitrary amount generated.
fn solver44() -> u32 {
    let mut add = 1;
    let mut pentagonal_num = 0;
    let pentagonal_nums: Vec<u32> = iter::from_fn(move || {
        pentagonal_num += add;
        add += 3;
        Some(pentagonal_num)
    })
    .take(5000)
    .collect();
    let pentagonal_nums_set = HashSet::<&u32>::from_iter(pentagonal_nums.iter());

    let mut min: Option<u32> = None;
    for i in 0..pentagonal_nums.len() {
        for k in i + 1..pentagonal_nums.len() {
            let n1 = pentagonal_nums[i];
            let n2 = pentagonal_nums[k];
            let diff = n1.abs_diff(n2);

            if pentagonal_nums_set.contains(&(n1 + n2)) && pentagonal_nums_set.contains(&diff) {
                min = min
                    .and_then(|old_min| Some(old_min.min(diff)))
                    .or(Some(diff))
            }
        }
    }
    min.expect("Didn't find a pair")
}

/// Was feeling lazy with this one and decided to brute force it, which was relatively
/// simple with the utilities I already created for permuting and converting slices
/// representing digits. Still took a moment to calculate. Some obvious optimizations
/// would be to use division rules to avoid converting whole digit slices. If I had
/// more patience I might've used an approach where I found multiples of the primes
/// with three unique digits and then tried to combine them.
fn solver43() -> u64 {
    permutations(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 10)
        .filter(|digits| {
            let primes = [2, 3, 5, 7, 11, 13, 17];
            primes
                .iter()
                .enumerate()
                .all(|(i, prime)| digits_to_u64(&digits[1 + i..=3 + i]) % prime == 0)
        })
        .fold(0, |sum, digits| sum + digits_to_u64(&digits))
}

/// This one was a nice break. Very straightforward. I calculated an arbitrary number
/// of triangle numbers since I wasn't sure what the largest word would be, doing so
/// additively and storing in a hash for quick reference. After that it was just a
/// matter of parsing the text file and liberal use of iterators.
fn solver42() -> usize {
    let mut triangle_num = 0;
    let mut index = 1;
    let triangle_nums = HashSet::<u32>::from_iter(
        iter::from_fn(move || {
            triangle_num += index;
            index += 1;
            Some(triangle_num)
        })
        .take(100),
    );

    let content = fs::read_to_string("inputs/problem42.txt")
        .expect("Unable to read input")
        .replace("\"", "");
    let strings = content.split(',');

    let ascii_offset = 64;
    strings
        .map(|string| {
            string
                .chars()
                .map(|char| (char as u32) - ascii_offset)
                .sum()
        })
        .filter(|n| triangle_nums.contains(n))
        .count()
}

/// Oi vey! I was a bit all over with this one trying to decide what utils would make this
/// easy without the utils themselves becoming too much of a burden. My first attempt figured
/// I could just iterate through all of the primes less than 10^10 and then look in reverse
/// for the first that passed is_pandigital...until I realized that 10,000,000,000 is quite
/// a large number.
///
/// So then I came from the other direction. Luckily I had just added utils for iterating
/// permutations, but this meant that instead I'd need a utility for testing primality, and
/// trying to do so efficiently was going to take up way too much of my time so I settled for
/// an inefficient but good enough checker. I also decided to make a quick util to convert
/// digits to u64s since I figured I'd probably be doing this again in future problems. Maybe
/// it'll become a struct eventually.
fn solver41() -> u64 {
    let digits = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    (0..9)
        .find_map(|d| {
            Permutations::new(&digits[d..], 9 - d).find_map(|p| {
                let num = digits_to_u64(&p);
                if num.is_prime() {
                    return Some(num);
                }
                None
            })
        })
        .expect("Didn't find pandigital prime")
}

/// I feel like there was a more clever way to solve this but I decided to just brute force it
/// anyway. Not at all a problem for modern hardware.
fn solver40() -> u32 {
    let mut silly_string = String::with_capacity(1_000_000);
    let mut n = 1;
    while silly_string.len() < 1_000_000 {
        silly_string.push_str(&n.to_string());
        n += 1;
    }
    [1usize, 10, 100, 1000, 10000, 100000, 1000000]
        .map(|n| {
            silly_string
                .chars()
                .nth(n - 1)
                .unwrap()
                .to_digit(10)
                .unwrap()
        })
        .iter()
        .product()
}

/// I'm not sure if this is the best approach but I ended up finding a method
/// for generating Pythagorean triples and then just iterating every combination
/// of values that didn't break a perimeter of 1000, tracking the perimeter counts
/// in a hash map. I'm not sure if my loops are as elegant as they could be but
/// they got the job done.
fn solver39() -> u64 {
    let mut perimeter_counts: HashMap<u64, u32> = HashMap::new();
    'outer: for n in 1.. {
        'inner: for m in n + 1.. {
            if n % 2 == m % 2 || gcd(n, m) != 1 {
                continue;
            }
            // Euclid's formula, which generates primitive pythagorean triples
            let base_p = (m.pow(2) - n.pow(2)) + 2 * m * n + (m.pow(2) + n.pow(2));
            let mut p = 0;
            // Check every multiple of the primitive until breaking 1,000
            for k in 1.. {
                p += base_p;
                if p <= 1_000 {
                    perimeter_counts
                        .entry(p)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                } else if k > 1 {
                    // Try next m, resetting k
                    break;
                } else if m > n + 1 {
                    // Try next n, resetting m
                    break 'inner;
                } else {
                    // Done
                    break 'outer;
                }
            }
        }
    }
    *perimeter_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .expect("Max not found")
        .0
}

/// The funny thing is that I figured out to only check 4 digit numbers, started coming up
/// with a solver, made one little mistake that got me the almost right answer, and so I continued
/// working through a deduction of how to find the right answer which became a second solution in
/// its own right without any code necessary. But I fixed up the code so I'll include it anyway.
/// Ironically despite my stubbornness against pulling in crates I still resorted to AI to help create
/// an iterator for permutations because that was an entirely separate sub problem I just didn't have
/// the interest in solving currently even though I created BigInt to solve other problems.
///
/// Observations:
/// - Needs to start with 9, and you'll always get an 1 and 8 on double, i.e. an added digit
/// - Two digits doesn't work because then you'd have 2 + 3 + 3 + (>2) > 9
/// - Three digits doesn't work because you'd have 3 + 4 + (>2) > 9
/// - Logically it would have to be 4 digits because 4 + 5 = 9
/// - So this leaves 2, 3, 4, 5, 6, 7
/// - Second digit needs to be <5 or else you get another 9
/// - Can't use 4 because that'll double to 8
/// - Second digit must be either 2 or 3
/// - 2 also needs to show up because we're getting 1 from doubling 9
/// - 3, 5, 7 are odd numbers that we wouldn't normally get through doubling:
///     - We don't have room for all of them
///     - 5 could be created by 2 preceding 7 to make 54
///       - This also sorts 2 showing up but not conflicting with 7 doubling
/// - That means that we must end up with a number such as 9273 or 9327
/// - We can quickly check both options to verify the answer
fn solver38() -> String {
    let digits = [9u32, 8, 7, 6, 5, 4, 3, 2, 1];

    for permutation in permutations(&digits, 4) {
        let mut remaining: Vec<u32> = digits
            .iter()
            .filter(|&n| !permutation.contains(n))
            .map(|&n| n)
            .collect();
        remaining.sort();
        let remaining_sorted_str: String = remaining.iter().map(|&n| n.to_string()).collect();

        let combo_nbr = permutation.iter().fold(0, |acc, &n| acc * 10 + n);
        let double = combo_nbr * 2;
        let mut double_chars: Vec<char> = double.to_string().chars().collect();
        double_chars.sort();
        let double_sorted_str: String = double_chars.iter().collect();

        if remaining_sorted_str == double_sorted_str {
            return format!("{combo_nbr}{double}");
        }
    }
    panic!("Did not find answer")
}

/// This one was interesting in that it went through a few iterations to arrive
/// at an implementation I was finally happy with. An important optimization I
/// realized along the way was this seemed like another classic case for DP. Every
/// later prime would have to end with one of the single digit primes. So I could
/// simply do the left and right truncations on a prime and check if that truncation
/// was already in either set to put the prime into it. Since I was iterating toward
/// larger primes, I was guaranteed to have any previous truncatable primes cached
/// already and would only need to check one left and right truncation. And once
/// again Rust's iterators made what was originally a scrappy while loop in a prev
/// implementation much more elegant.
fn solver37() -> u64 {
    let primes = Primes::new();
    let mut rtl_trunctable_primes = HashSet::<u64>::new();
    let mut ltr_trunctable_primes = HashSet::<u64>::new();

    primes
        .filter(|&prime| {
            if prime < 10 {
                rtl_trunctable_primes.insert(prime);
                ltr_trunctable_primes.insert(prime);
                return false;
            }

            // Right to left truncation
            let rtl_truncated = prime / 10;
            if rtl_trunctable_primes.contains(&rtl_truncated) {
                rtl_trunctable_primes.insert(prime);
            }

            // Left to right truncation
            let position = 10u64.pow((prime as f64).log10().floor() as u32);
            let ltr_truncated = prime % position;
            if ltr_trunctable_primes.contains(&ltr_truncated) {
                ltr_trunctable_primes.insert(prime);
            }

            ltr_trunctable_primes.contains(&prime) && rtl_trunctable_primes.contains(&prime)
        })
        .take(11)
        .sum()
}

/// This was a short and sweet one thanks to Rust's iterators once again.
fn solver36() -> u32 {
    (1..1_000_000)
        .filter(|n| {
            let str = n.to_string();
            let rev: String = str.chars().rev().collect();
            if str != rev {
                return false;
            };
            let bin = format!("{:b}", *n);
            let rev_bin: String = bin.chars().rev().collect();

            bin == rev_bin
        })
        .sum()
}

/// Woo! Another one I got right on the first try. This ended up being a little
/// messier and suboptimal than I would've liked. The thing about circular primes
/// is they exist in groups of up to the number of digits, and so if I validate that one
/// three-digit prime is circular, then I've technically validated that up to three
/// primes are circular. The trick is that for a number like, say, 1717 (which isn't
/// prime, but for illustrative purposes), it technically only has two numbers in its
/// rotation before repeating. And so unless I also kept track of that somehow I
/// couldn't as easily optimize like I wanted. Oh well, it ran plenty quickly.
fn solver35() -> u32 {
    let primes = Primes::new();
    let primes_to_million: HashSet<u64> = primes.take_while(|p| *p < 1_000_000).collect();
    let mut total = 0;

    'outer: for prime in primes_to_million.iter() {
        // Store the digits in a vector for easier manipulation
        let mut digits: Vec<u64> = Vec::new();
        let mut remainder = *prime;
        while remainder > 0 {
            digits.push(remainder % 10);
            remainder /= 10;
        }
        // For a prime of d digits, there are d - 1 other possible primes in its circle
        for _ in 0..digits.len() {
            digits.rotate_left(1);
            // Build the next possible circular prime
            let contender = digits.iter().rev().fold(0, |acc, digit| acc * 10 + digit);
            // No need to check the remaining, let's move to the next prime
            if !primes_to_million.contains(&contender) {
                continue 'outer;
            }
        }
        total += 1;
    }
    total
}

/// Needed a little help from AI to figure out how to calculate a ceiling but it
/// was smooth sailing from there otherwise. The obvious optimization to make at
/// first was to cache all of the factorials of the digits. Other that that, I
/// took a fairly brute force approach (what else is new?).
fn solver34() -> u32 {
    // Cache all digit factorials
    let mut factorials = [1u32; 10];
    for i in 2..factorials.len() {
        factorials[i] = factorials[i - 1] * i as u32;
    }
    // Some d digit number can only possibly work if its minimum value is less
    // than the maximum possible sum (i.e. d * 9!). Find the max digits that
    // could possibly work.
    let digit_ceiling = (1..)
        .find(|d| d * factorials[9] < 10u32.pow(d - 1))
        .expect("Did not find ceiling");
    let ceiling = 10u32.pow(digit_ceiling - 1) - 1;

    // No single digit number will work, and the calculated ceiling is the
    // highest possible number that could work.
    (10..=ceiling)
        .filter(|n| {
            let mut remainder = *n;
            let mut sum = 0;
            while remainder > 0 {
                sum += factorials[(remainder % 10) as usize];
                remainder /= 10;
            }
            sum == *n
        })
        .sum()
}

/// Ick, I started this one before vacation and came back to it luckily with instruction
/// from my past self on what I had left to do. This one was messy and not very "fun" to
/// solve but I got there in the end. I was afraid I'd be stuck on this one for a while
/// given the answer and how wrong it seemed, but hark, the answer 'twas!
fn solver33() -> u64 {
    let mut numerators = vec![49u64];
    let mut denominators = vec![98u64];
    for numerator in 10..=99 {
        let tens_digit = numerator % 10;
        let tens_add = tens_digit * 10;
        for denominator in (1..=9)
            .map(|n| n + tens_add)
            .filter(|denom| *denom > numerator)
        {
            let other_numerator = numerator / 10;
            let other_denominator = denominator % 10;
            let numerators_are_divisible = numerator % other_numerator == 0;
            let denominators_are_divisible = denominator % other_denominator == 0;
            let numerator_factor = numerator / other_numerator;
            let denominator_factor = denominator / other_denominator;
            let same_factor = numerator_factor == denominator_factor;

            if numerators_are_divisible && denominators_are_divisible && same_factor {
                numerators.push(numerator);
                denominators.push(denominator);
                println!("{numerator} / {denominator}")
            }
        }
    }
    let mut final_numerator: u64 = numerators.iter().product();
    let mut final_denominator: u64 = denominators.iter().product();

    let mut primes = Primes::new();
    let mut next_prime = primes.next().unwrap();
    loop {
        if next_prime > final_numerator {
            break;
        }
        if final_numerator % next_prime != 0 || final_denominator % next_prime != 0 {
            next_prime = primes.next().unwrap();
        } else {
            final_numerator /= next_prime;
            final_denominator /= next_prime;
        }
    }
    println!("{final_numerator}");
    final_denominator
}

/// This was a case of borrowing someone else's homework and modifying it at little
/// just to try different techniques out. It's definitely another brute force approach
/// that could be optimized in several ways. Honestly this one just did not pique my
/// interest as much as others.
fn solver32() -> u32 {
    let mut products: HashSet<u32> = HashSet::new();
    let pandigital_set = (1..=9)
        .map(|n| char::from_digit(n, 10).unwrap())
        .collect::<HashSet<char>>();
    for multiplicand in 1..=99u32 {
        for multiplier in 1..=9999u32 / multiplicand {
            let product = multiplicand * multiplier;
            let mut char_set: HashSet<char> = HashSet::new();
            char_set.extend(multiplicand.to_string().chars());
            char_set.extend(multiplier.to_string().chars());
            char_set.extend(product.to_string().chars());

            if char_set == pandigital_set {
                products.insert(product);
            }
        }
    }
    products.iter().sum()
}

/// Not gonna lie, this one was mostly looking up example code. I had the
/// general idea that this seemed like a recursive and/or DP approachable
/// problem because the amount of ways of representing some larger amount
/// depends on the ways to represent a smaller amount, but then I got hung
/// up on how to find distinct combinations and not just permutations, so
/// that's when I needed to look it up.
fn solver31() -> u32 {
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];

    coin_combination_count(&coins, 200)
}

fn coin_combination_count(coins: &[u32], amount: u32) -> u32 {
    let mut ways = vec![0; (amount + 1) as usize];
    ways[0] = 1;

    coins.iter().for_each(|coin| {
        (*coin..=amount).for_each(|i| ways[i as usize] += ways[(i - coin) as usize]);
    });

    ways[amount as usize]
}

/// I'm knocking these out relatively quick if I keep making lazy decisions.
/// I didn't know a great way to find the upper ceiling so I did some trial
/// and error to figure out when I stopped getting a larger answer... Otherwise
/// it's another brute force solution that checks every number. Once again Rust
/// makes inelegant solutions look and run more elegantly than they deserve.
fn solver30() -> u32 {
    (2..=1_000_000)
        .filter(|n| {
            let mut sum = 0;
            let mut remainder = *n;
            while remainder > 0 {
                let term: u32 = remainder % 10;
                sum += term.pow(5);
                remainder /= 10;
            }
            sum == *n
        })
        .sum()
}

/// Another one that felt like it was going to be worse than it ended up being.
/// Also another that I just brute forced. I prematurely implemented MulAssign
/// between BigInts but realized I didn't need it and could just stick with
/// u32 MulAssign. I did decide that having a ToString impl was worthwhile and
/// used that for the hash set. The one maybe optimization was realizing I could
/// just keep multiplying the same BigInt instance. I'm sure there are other
/// mathgic optimizations.
fn solver29() -> usize {
    let mut terms: HashSet<String> = HashSet::new();
    for a in 2..=100 {
        let mut term = BigInt::from(a);
        for _ in 2..=100 {
            term *= a;
            terms.insert(term.to_string());
        }
    }
    terms.len()
}

/// I finally did some mathematical analysis...well more like casual inference.
/// I noticed that:
/// 1. Each top-right corner seemed to be increasing by n*8
/// 2. The other corners decreased by 2n each, in sum being 12n
/// So I just looped over the amount of "rounds" doing this calculation. Come to
/// think of it, I didn't think at the time to find any pattern in what the running
/// sum looked like, if I could infer an n1 solution. Ah well, I'm happy enough with
/// this solution.
fn solver28() -> u32 {
    let mut next = 1;
    let mut sum = 1;
    for n in 1..501u32 {
        next = n * 8 + next;
        sum += next * 4 - n * 12
    }
    sum
}

/// OK, this one wasn't as bad as I thought it'd be. I wasn't sure exactly how to
/// approach it, but I decided to just brute force check it. I had no real basis
/// for deciding how many primes to load into the set (well, I suppose n = 80, a=999,
/// and b = 1000 could've set the upper ceiling?) but after that it was just iterating
/// through every combination of a and b and finding the position of the first nonprime
/// number which in effect represented the length of consecutive primes, and tracking
/// the max thus far.
fn solver27() -> i32 {
    let primes = Primes::new();
    let prime_set: HashSet<u64> = primes.take(10_000).collect();
    let mut max_consecutive_primes = 0;
    let mut answer = 0;

    for a in -999..=999 {
        for b in -1000..=1000 {
            let consecutive_primes = (0i32..)
                .position(|n| {
                    let y = n.pow(2) + a * n + b;
                    0 >= y || !prime_set.contains(&(y as u64))
                })
                .expect("Didn't find nonprime");
            if consecutive_primes > max_consecutive_primes {
                max_consecutive_primes = consecutive_primes;
                answer = a * b;
            }
        }
    }

    println!("Max consecutive primes: {max_consecutive_primes}");
    answer
}

/// This one was a doozy. I noticed that, long story short, primes seemed to be
/// the numbers to focus on, and after a lot of research into reciprocals of primes
/// and how to calculate the length of one's repetend (and that that's the name of
/// of the repeating bits of a decimal) it was actually straight forward. I was
/// struggling to understand the math so I had to breakdown and use an AI to help
/// with the calculation portion. Also I was finally bothered to redo Primes as
/// an iterator that hopefully will devolve less than my other implementation.
fn solver26() -> u64 {
    let primes = Primes::new();

    primes
        .take_while(|prime| *prime < 1000)
        .max_by(|a, b| {
            prime_repetend_length(*a)
                .unwrap_or(0)
                .cmp(&prime_repetend_length(*b).unwrap_or(0))
        })
        .expect("Max not found")
}

fn prime_repetend_length(p: u64) -> Option<u64> {
    if p == 2 || p == 5 {
        return Some(0); // These primes yield terminating decimals
    }

    let mut k = 1;
    let mut mod_pow = 10 % p;

    while mod_pow != 1 {
        mod_pow = (mod_pow * 10) % p;
        k += 1;

        if k > p {
            return None;
        }
    }

    Some(k)
}

/// Euler Project or: How I learned to stop overflowing and love iteration.
/// I'm just gonna end up fully implementing BigInt at this rate and turning
/// anything I possibly can into an iterator. Not sure that this was the best
/// approach but it worked out.
fn solver25() -> usize {
    FibonacciSequence::new()
        .position(|big_int| big_int.digit_count() == 1000)
        .unwrap()
        + 2 // Technically the 0th iteration gives the 2nd term
}

/// This was a fun one with an obvious, albeit relatively insignificant optimization.
/// I recall from combinatorics that the number of permutations of n items is just n!
/// I also realized that n! < 1,000,000 so I would needlessly cycle through unless I
/// calculated the number of iterations via a circular indexing calculation. After that
/// it was just running the algorithm to find the next lexicographic permutation. I
/// modify a vector in place to avoid allocating a new one for each iteration. It could
/// probably be done with arrays but...ah well.
fn solver24() -> String {
    let permutation_count = factorial(10);
    // Just like calculating the index of a circular array, we want to figure out how many
    // iterations we need starting from the base case to get to the millionth permutation
    // (which is 999_999 items after the base permutation)
    let cycled_index = (999_999 % permutation_count + permutation_count) % permutation_count;

    let mut ordering = vec![0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for _ in 0..cycled_index {
        ordering.next_lexicographic_permutation();
    }
    ordering
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// So this was silly. The first implementation used a vector instead of a hash set, which
/// took a long but sufferable amount of time and it did produce the correct answer on the
/// first try. I didn't know how else to optimize since I thought collecting all of the
/// abundant numbers up front would help but had my eureka moment when I realized searching
/// the vector was taking too long. I didn't need order necessarily. It was more important
/// to check that a number was present in the set at all. This dramatically improved runtime.
/// And Rust is amazing in that all I had to do was change the type declaration and everything
/// just magically worked.
fn solver23() -> u32 {
    let abundant_numbers: HashSet<u32> = (12..=28111u32).filter(|n| n.is_abundant()).collect();

    (1..24).sum::<u32>()
        + (25..=28123u32).fold(0u32, |acc, n| {
            if !abundant_numbers
                .iter()
                .filter(|&&abundant_number| abundant_number <= n / 2)
                .any(|abundant_number| abundant_numbers.contains(&(n - abundant_number)))
            {
                acc + n
            } else {
                acc
            }
        })
}

/// This one was fairly straightforward with not much to talk about other than
/// that TIL to get the ASCII value of a char...you just cast it to a u8.
fn solver22() -> u32 {
    let content = fs::read_to_string("inputs/0022_names.txt").expect("Unable to read input");

    let cleaned_content = content.replace("\"", "");
    let mut names: Vec<&str> = cleaned_content.split(",").collect();
    names.sort_unstable();

    names.iter().enumerate().fold(0, |mut total, (i, name)| {
        total += name
            .chars()
            .map(|char| (char as u8 - 64) as u32)
            .sum::<u32>()
            * (i as u32 + 1);
        total
    })
}

/// So the first 10 pairs are actually listed on Wikipedia...but I pretended
/// not to see that and figured I'd calculate them myself, though I verified
/// my answer based on the sum I got from the numbers on Wikipedia. It helped
/// me catch a bug before submission where I was counting perfect numbers.
/// The approach is pretty self-explanatory: for all integers between 2 and
/// 10,000 check its aliquot sum and verify that it isn't itself, and then if
/// the aliquot sum of its aliquot sum is itself, you have a pair of amicable
/// numbers, so add them to the set. Since a pair of numbers is found each time
/// we can skip the second one when we get to it. Then it's as easy as summing
/// the set. Have I mentioned I love Rust's iterators? Oh, and I tried my hand
/// at adding a custom trait.
fn solver21() -> u32 {
    (2..10_000u32)
        .fold(HashSet::new(), |mut amicable_numbers, n| {
            if !amicable_numbers.contains(&n) {
                let contender = n.aliquot_sum();
                if contender != n && contender.aliquot_sum() == n {
                    amicable_numbers.insert(n);
                    amicable_numbers.insert(contender);
                }
            }
            amicable_numbers
        })
        .iter()
        .sum()
}

/// Ugh, making us avoid overflows again, eh? Fine, you've broken me, I'll
/// start my own big int implementation. But I'm only implementing the
/// functionality I need at the moment! So, similar to the last problem
/// that was doubling each digit, BigInt uses a vector for arbitrarily large
/// integers, but now I have multiplication support...for u32s.
fn solver20() -> u32 {
    (2..=100)
        .fold(BigInt::from(1), |mut digits, f| {
            digits *= f;
            digits
        })
        .digit_sum()
}

/// This one was a bit scrappy because I didn't want to spend a lot of time on validation
/// or enums, even though those would be more proper. I'm quite fond of making custom
/// iterators now since I can leverage all of the built-in iterator adapters like
/// skip and take to elegantly calculate a solution. The custom iterator logic itself is
/// a bit tedious but straight forward.
fn solver19() -> usize {
    let first_of_months = FirstOfMonth {
        day: 1,
        month: 0,
        year: 1900,
    };
    first_of_months
        // We start on 1900 since we know it starts on a Monday but we want to count
        // from 1901
        .skip(12)
        .take(12 * 100)
        .filter(|day| *day == 0)
        .count()
}

struct FirstOfMonth {
    day: u32,
    month: u8,
    year: u32,
}

impl Iterator for FirstOfMonth {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.day = (match self.month {
            0 => 31,
            1 => {
                if self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0 {
                    29
                } else {
                    28
                }
            }
            2 => 31,
            3 => 30,
            4 => 31,
            5 => 30,
            6 => 31,
            7 => 31,
            8 => 30,
            9 => 31,
            10 => 30,
            11 => 31,
            _ => panic!("Not a valid day"),
        } + self.day)
            % 7;

        self.month = if self.month < 11 { self.month + 1 } else { 0 };
        if self.month == 11 {
            self.year += 1
        };

        Some(self.day)
    }
}

/// Well they at least tipped us off this time that we'll be revisiting this problem
/// with a larger triangle later so I tried to find the mathgic right away. Essentially
/// we have a DAG, though thankfully it's simple enough that I didn't actually need
/// any complex data structures. The order of the parsed numbers is already a topological
/// sorting so then I can just do some good ol' dynamic programming to iteratively
/// calculate each node's "length" from the starting node. Then I just find the max length.
fn solver18() -> u32 {
    let content = fs::read_to_string("inputs/problem18.txt").expect("Unable to read input");

    max_path_through_triangle(&content)
}

fn max_path_through_triangle(triangle: &str) -> u32 {
    let mut length_map = HashMap::new();
    for (y, line) in triangle.lines().enumerate() {
        for (x, num) in line
            .split(' ')
            .map(|string| string.parse::<u32>().expect("Not a number"))
            .enumerate()
        {
            let top_left: u32 = if y != 0 && x != 0 {
                *length_map.entry((y - 1, x - 1)).or_default()
            } else {
                0
            };
            let top_right: u32 = if y != 0 {
                *length_map.entry((y - 1, x)).or_default()
            } else {
                0
            };
            length_map.insert((y, x), num + max(top_left, top_right));
        }
    }
    *length_map.values().max().expect("No max found")
}

/// This one was weird because it was a lot of manual enumeration. As a base, 1-19
/// all are more or less special cases. 20-99 can be recursively checked adding the
/// tens digit to the ones digit. Likewise with the hundreds except we also have to
/// account for "and" if not on an exact hundreds number. Great use of Rust pattern
/// matching, though! I ended up collapsing a bunch of ranges together for conciseness.
fn solver17() -> u32 {
    (1..=1000).map(letter_count).sum()
}

fn letter_count(n: u32) -> u32 {
    match n {
        0 => 0, // As one's digit for any multi-digit number
        1..=2 => 3,
        3 => 5,
        4..=5 => 4,
        6 => 3,
        7..=8 => 5,
        9 => 4,
        10 => 3,
        11..=12 => 6,
        13..=14 => 8,
        15..=16 => 7,
        17 => 9,
        18..=19 => 8,
        20..=39 => 6 + letter_count(n % 10),
        40..=69 => 5 + letter_count(n % 10),
        70..=79 => 7 + letter_count(n % 10),
        80..=99 => 6 + letter_count(n % 10),
        100..=999 => {
            let tens = letter_count(n % 100);
            7 + letter_count(n / 100) + tens + if tens > 0 { 3 } else { 0 }
        }
        1000 => 11,
        _ => panic!("Numbers higher than 1000 currently not supported"),
    }
}

/// Good ol' integer overflows... Since the operation was simple, I basically made
/// a very rudimentary big int implementation with a vector to store the digits,
/// manually iterating it 1000 times and doubling each digit, adding a carry over
/// if it exists. The order of the digits don't actually matter when they're just
/// being summed in the end.
fn solver16() -> u32 {
    let mut digits = vec![1];
    for _ in 0..1000 {
        let mut carry = 0;
        for digit in &mut digits {
            let double = *digit * 2;
            *digit = double % 10 + carry;
            carry = double / 10;
        }
        if carry > 0 {
            digits.push(carry)
        }
    }
    digits.iter().sum()
}

/// Did a little research because this felt familiar from my combinatorics class
/// way back when. Turns out this is just a combination of essentially 40 choose 20.
/// Or at least I thought I figured out the mathgic but I needed u128s in order to
/// not overflow, so there must be more optimizations that can still be made.
fn solver15() -> u128 {
    combinations_count(20 + 20, 20)
}

fn combinations_count(items_count: u128, pick: u128) -> u128 {
    let lower = items_count - pick + 1;
    (lower..=items_count).product::<u128>() / factorial(pick)
}

// TODO: Make generic and maybe memoize
fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

/// OK, I definitely overthought the last problem. This one was fun and relatively simple.
/// I decided to try my hand at implementing an iterator which the Collatz sequence seemed
/// well suited for. Another brute force approach without any attempt to optimize. I'll be
/// very curious what the mathgic is for this one.
///
/// OK, all very obvious optimizations in hindsight. Nothing terribly interesting enough for
/// me to retroactively add.
fn solver14() -> u64 {
    let mut max_len = 0;
    let mut num_with_max = 0;
    for n in 1..1_000_000 {
        let n_collatz_sequence_len = collatz_sequence(n).count();
        if n_collatz_sequence_len > max_len {
            max_len = n_collatz_sequence_len;
            num_with_max = n;
        }
    }
    println!("{num_with_max} produces Collatz sequence of length {max_len}");
    num_with_max
}

struct CollatzSequence {
    curr: u64,
}

impl Iterator for CollatzSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == 1 {
            None
        } else if self.curr % 2 == 0 {
            self.curr /= 2;
            Some(self.curr)
        } else {
            self.curr = 3 * self.curr + 1;
            Some(self.curr)
        }
    }
}

fn collatz_sequence(start: u64) -> CollatzSequence {
    CollatzSequence { curr: start }
}

/// Alright, this was probably a little unorthodox. I finally decided to try out parsing
/// a file for the input! As for solving, first off, it's pretty clear we would overflow
/// with brute math so I got the idea to do elementary addition digit by digit. I collect
/// the sum of each placement into a vector.
///
/// Now, one idea I had originally was to go through the effort to "carry over" any
/// overflow into the next indices working from so that I'd have exactly 1 digit in each
/// index in the vector, and then I could just grab the range of the "last" 10 items, but
/// this seemed like needless work when I knew I needed only 10 of the 50+ digits.
///
/// Instead I realized that since each place is x10 from the next, I can just keep "shifting"
/// my number and adding the next sum of the digits in a place. However, since there was
/// still the possibility of carry-over, I needed to overshoot and cross my fingers that
/// I had enough that nothing else was likely to affect the first 10 digits. 20 sums resulted in
/// overflows so I settled for 15 sums, though after confirming the answer it so happens
/// you only need the 11th sum, but of course this can't be guaranteed AFAIK. As usual
/// there's probably some mathgic I'm missing, though.
fn solver13() -> String {
    let contents = fs::read_to_string("inputs/problem13.txt").expect("Unable to load input file");
    // Create flat array with everything "rotated" so that the 1s digits are together, then 10s, etc.
    // i.e. 50 virtual "rows" (placements) of 100 "columns" (digits in placement)
    let mut digits = [0u8; 5000];
    contents.lines().enumerate().for_each(|(row, line)| {
        line.chars()
            .rev()
            .map(|char| char.to_digit(10).expect("Not a digit") as u8)
            .enumerate()
            .for_each(|(column, digit)| digits[100 * column + row] = digit);
    });
    // Worst case scenario = 9 * 100 = 900
    let digit_sums: Vec<u16> = (0..5000)
        .step_by(100)
        .map(|i| digits[i..i + 100].iter().map(|digit| *digit as u16).sum())
        .rev()
        .collect();

    let mut first_digits = 0;
    // Honestly, this is probably not reliable but chances are an extra 5 digits in we're not going to have
    // any more carry-overs...
    for i in 0..15 {
        first_digits += digit_sums[i] as u64;
        // Shift all digits over to prepare adding the next
        first_digits *= 10;
    }
    first_digits.to_string()[0..10].to_string()
}

/// I'm never going to escape my PrimeNumbers, am I? Was easy enough to look up how to calculate
/// the nth triangular number, and that you can get the number of divisors via prime factorization,
/// which did make this relatively straight forward to solve and surprisingly quickly. Not sure if
/// there is any optimization to this one other than maybe starting with a more sensible n depending
/// on the input.
fn solver12(divisors_min: u32) -> u64 {
    let mut primes = PrimeNumbers::new();
    let mut n = 1; // If this was less general purpose, we could start with something higher
    loop {
        let mut factors: Vec<u64> = Vec::new();
        let mut factor_counts: HashMap<u64, u32> = HashMap::new();
        let triangular_number = nth_triangular_number(n);
        primes.get_prime_factors(triangular_number, &mut factors);

        for factor in factors {
            *factor_counts.entry(factor).or_default() += 1;
        }

        let divisors_count: u32 = factor_counts.values().map(|val| val + 1).product();
        if divisors_count > divisors_min {
            println!("The n={n} triangular number is the first with over 500 divisors:");
            return triangular_number;
        }
        n += 1;
    }
}

fn nth_triangular_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

// I attempted to use a const function to initialize the array but gave up so honestly
// I resorted to just parsing the input in JS so I could move on with the problem
const GRID: [[u32; 20]; 20] = [
    [
        8, 2, 22, 97, 38, 15, 0, 40, 0, 75, 4, 5, 7, 78, 52, 12, 50, 77, 91, 8,
    ],
    [
        49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 0,
    ],
    [
        81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 3, 49, 13, 36, 65,
    ],
    [
        52, 70, 95, 23, 4, 60, 11, 42, 69, 24, 68, 56, 1, 32, 56, 71, 37, 2, 36, 91,
    ],
    [
        22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
    ],
    [
        24, 47, 32, 60, 99, 3, 45, 2, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
    ],
    [
        32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
    ],
    [
        67, 26, 20, 68, 2, 62, 12, 20, 95, 63, 94, 39, 63, 8, 40, 91, 66, 49, 94, 21,
    ],
    [
        24, 55, 58, 5, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
    ],
    [
        21, 36, 23, 9, 75, 0, 76, 44, 20, 45, 35, 14, 0, 61, 33, 97, 34, 31, 33, 95,
    ],
    [
        78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 3, 80, 4, 62, 16, 14, 9, 53, 56, 92,
    ],
    [
        16, 39, 5, 42, 96, 35, 31, 47, 55, 58, 88, 24, 0, 17, 54, 24, 36, 29, 85, 57,
    ],
    [
        86, 56, 0, 48, 35, 71, 89, 7, 5, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
    ],
    [
        19, 80, 81, 68, 5, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 4, 89, 55, 40,
    ],
    [
        4, 52, 8, 83, 97, 35, 99, 16, 7, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
    ],
    [
        88, 36, 68, 87, 57, 62, 20, 72, 3, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
    ],
    [
        4, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 8, 46, 29, 32, 40, 62, 76, 36,
    ],
    [
        20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 4, 36, 16,
    ],
    [
        20, 73, 35, 29, 78, 31, 90, 1, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 5, 54,
    ],
    [
        1, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 1, 89, 19, 67, 48,
    ],
];

/// This reminds me of Advent of Code type problems. Starting from a cell, you can check in
/// 4 directions: NE, E SE, and S. And of course depending on the direction, you're limited
/// in how many rows and columns you need to check. Other than that, it's simply a matter of
/// iterating through every combo. If it's 0 we can just ignore that cell.
fn solver11() -> u32 {
    (0..GRID.len())
        .map(check_row)
        .max()
        .expect("Couldn't find max")
}

/// This seemed like a good opportunity to attempt multi-threading since each row can be checked
/// independently from a const grid.
fn solver11a() -> u32 {
    let handles: Vec<JoinHandle<u32>> = (0..GRID.len())
        .map(|row| thread::spawn(move || check_row(row)))
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .max()
        .expect("Couldn't find max")
}

fn check_row(row: usize) -> u32 {
    let mut max = 0u32;

    for (column, val) in GRID[row].iter().enumerate() {
        if *val == 0u32 {
            continue;
        }
        // NE
        if column < 17 && row > 3 {
            let product = (0..4).map(|diff| GRID[row - diff][column + diff]).product();
            max = max.max(product)
        }
        // E
        if column < 17 {
            let product = (0..4).map(|diff| GRID[row][column + diff]).product();
            max = max.max(product)
        }
        // SE
        if column < 17 && row < 17 {
            let product = (0..4).map(|diff| GRID[row + diff][column + diff]).product();
            max = max.max(product)
        }
        // S
        if row < 17 {
            let product = (0..4).map(|diff| GRID[row + diff][column]).product();
            max = max.max(product)
        }
    }
    println!("Found max for row {row} of {max}");
    max
}

/// Another case of Rust somewhat trivializing the problem, though this is likely not
/// terribly efficient. Not to mention PrimeNumbers does not feel very well designed or
/// idiomatic and so I'm only more prone to abusing it as I go.
fn solver10(n: u64) -> u64 {
    let mut primes = PrimeNumbers::new();
    primes.add_primes(n);
    primes.cache.iter().filter(|prime| **prime < n).sum()
}

/// Another relatively brute force method with some optimizations based on the limits
/// of what contendor numbers can be.
fn solver9() -> i32 {
    // Since x < y < z, we start at 1 and x can only ever be at most = 332 < 333 < 335
    for x in 1..332i32 {
        // Honestly, it's difficult to explain why the max is 500 - x / 2, but this
        // is what I inferred after some manual enumeration
        for y in x + 1..500 - x / 2 {
            let z = 1000 - x - y;
            let sum = f64::from(x.pow(2) + y.pow(2));
            if (sum.sqrt()) == z as f64 {
                println!("Found: {x} {y} {z}");
                return x * y * z;
            }
        }
    }
    panic!("Answer not found!")
}

/// Originally I wanted to be more clever by keeping track of a single mutable value where
/// I'd divide the starting digit and then multiply the next digit in a sort of queue but
/// 0s quickly shot down this approach without needing more complications to check for 0s
/// within a range and start fresh with a new product whenever we found a range without 0s.
/// So I went with a more naive brute force sliding range. Once again, Rust iterators to the
/// rescue. Apparently there isn't any PDF for this one describing any clever tricks so...I
/// guess I'm on my own if I want to optimize this.
fn solver8() -> u64 {
    let str_num = String::from(
        "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450",
    );
    let mut start = 0;
    let increment = 13;
    let mut max = 0u64;
    while start + increment < str_num.len() {
        let product = str_num[start..start + increment]
            .chars()
            .map(|char| char.to_digit(10).expect("Not a digit"))
            .map(u64::from)
            .product();
        if product > max {
            max = product
        }
        start += 1;
    }
    max
}

/// Alright, so I did end up reusing the PrimeNumbers object, and ultimately this was brute
/// force, but I guess still not too much for Rust to handle. I'm hoping there's some mathgic
/// to make this more efficient.
fn solver7() -> u64 {
    let mut primes = PrimeNumbers::new();
    primes.get_nth_prime(10_001)
}

/// Huh...OK that one was really straight forward. Though at least it helped me appreciate
/// Rust's iterators even more with how easy it was to implement
fn solver6() -> u64 {
    let square_of_sums = (1..=100).sum::<u64>().pow(2);
    let sum_of_squares = (1..=100).map(|num: u64| num.pow(2)).sum::<u64>();
    square_of_sums - sum_of_squares
}

/// OK, of course there were optimizations to make this run in O(1) even as the limit grows large
fn solver6a(limit: u64) -> u64 {
    let sum = limit * (limit + 1) / 2;
    let sum_sq = (2 * limit + 1) * (limit + 1) * limit / 6;
    sum.pow(2) - sum_sq
}

/// They gave us an easy one this time as far as finding the mathgic. Just need to use
/// prime factorization and use the highest exponent for each prime that occurs in any
/// of the integers from 1-20. This could technically be generalized using my PrimeNumbers
/// struct to get prime factors for any arbitrary range, calculating and storing the max
/// exponent, but...maybe another time.
fn solver5() -> u32 {
    2u32.pow(4) * 3u32.pow(2) * 5 * 7 * 11 * 13 * 17 * 19
}

fn is_palindrome(n: &impl ToString) -> bool {
    let n_str = n.to_string();
    let reverse: String = n_str.chars().rev().collect();
    n_str == reverse
}

/// I'm pleased with how this one turned out (ignoring the palindrome check...)
/// because it *seems* like more idiomatic Rust this time around. It could be made
/// a bit more efficient and once again I'm probably missing some mathgic but this
/// one felt less bad than the last problem.
fn solver4() -> u32 {
    // Two three-digit numbers would result in a six-digit product
    (100_000..999_999)
        .filter(is_palindrome)
        .rev()
        .find_map(|product: u32| {
            // The factors switch after the sqrt, so we only need to check up to it
            let max = product.isqrt();
            if let Some(factor) = (100..max).rev().find(|check_factor| {
                product % check_factor == 0 && (100..999).contains(&(product / check_factor))
            }) {
                println!("Factors: {factor} * {}", product / factor);
                Some(product)
            } else {
                None
            }
        })
        .unwrap()
}

#[derive(Debug)]
struct PrimeNumbers {
    cache: Vec<u64>,
}

impl PrimeNumbers {
    // A better implementation might let you specify your own starting vector
    // but then I would need to at least validate it which in part eliminates
    // any benefit unless I blindly trust the input
    pub fn new() -> PrimeNumbers {
        // Seed the cache with known primes
        PrimeNumbers {
            cache: vec![2, 3, 5, 7, 11, 13],
        }
    }

    fn add_primes(&mut self, to: u64) {
        let mut next = self.cache.last().unwrap_or(&0) + 1;
        while next < to {
            self.cache_if_prime(next);
            next += 1;
        }
    }

    fn get_nth_prime(&mut self, n: usize) -> u64 {
        let mut check = self.cache.last().unwrap_or(&1).clone();
        while (self.cache.len()) < n {
            check += 1;
            self.cache_if_prime(check);
        }
        self.cache[n - 1]
    }

    fn cache_if_prime(&mut self, n: u64) {
        let max_check = n.isqrt();
        let upper_index = self
            .cache
            .iter()
            .position(|prime| *prime > max_check)
            .unwrap_or(self.cache.len() - 1);
        if self.cache[0..upper_index]
            .iter()
            .all(|prime| n % prime != 0)
        {
            self.cache.push(n);
        }
    }

    pub fn get_prime_factors(&mut self, n: u64, out: &mut Vec<u64>) {
        if self.cache.binary_search(&n).is_ok() {
            out.push(n);
            return;
        }
        let max = n.isqrt();
        self.add_primes(max);
        if let Some(factor) = self
            .cache
            .iter()
            .filter(|prime| **prime != 1 && **prime <= max)
            .find_map(|prime| if n % *prime == 0 { Some(prime) } else { None })
        {
            out.push(*factor);
            self.get_prime_factors(n / factor, out)
        } else {
            out.push(n)
        }
    }
}

/// This one was a bit messier than I'd like ironically because I was probably trying to be
/// too clever and there is a lot of leftover unnecessary work from previous iterations. I've
/// had my fill with this one so I'm just going to leave this inelegant solution in for now.
fn solver3() -> u64 {
    let mut primes = PrimeNumbers::new();

    let mut prime_factors = Vec::new();
    primes.get_prime_factors(600851475143, &mut prime_factors);

    prime_factors.sort();
    *prime_factors.last().expect("No prime factors found")
}

/// Finds the sum of all even Fibonacci sequence numbers below four million.
/// Uses naive approach of manually calculating and checking every number. There is likely
/// a math trick I am missing.
fn solver2() -> u32 {
    let mut term1 = 1;
    let mut term2 = 2;
    let mut sum = 0;

    while term2 < 4_000_000 {
        if term2 % 2 == 0 {
            sum += term2;
        }
        let next = term1 + term2;
        term1 = term2;
        term2 = next;
    }
    sum
}

/// Yep, I didn't come up with this on my own but wanted to implement it. This uses a formula
/// for getting only even numbers in the sequence.
fn solver2a() -> u32 {
    let mut term1 = 2;
    let mut term2 = 8;
    let mut sum = 2;

    while term2 < 4_000_000 {
        sum += term2;
        let next = term1 + term2 * 4;
        term1 = term2;
        term2 = next;
    }
    sum
}

/// Finds the sum of all multiples of 3 and 5 by finding the sums of each individually and then
/// subtracting the sum of multiples of 15 to remove "duplicates"
/// In theory this is faster than iterating on all 1000 integers: 333 + 200 + 66 = 599 < 1000
fn solver1() -> i32 {
    (3..1000).step_by(3).sum::<i32>() + (5..1000).step_by(5).sum::<i32>()
        - (15..1000).step_by(15).sum::<i32>()
}

/// Returns sum of arithmetic series from step to 1000 incrementing by step
fn sum_arithmetic_series(step: u32) -> u32 {
    let terms_count = 999 / step;
    let last_term = terms_count * step;
    terms_count * (step + last_term) / 2
}

/// More efficient solver using arithmetic series formula
fn solver1a() -> u32 {
    sum_arithmetic_series(3) + sum_arithmetic_series(5) - sum_arithmetic_series(15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_works() {
        assert!(is_palindrome(&1));
        assert!(is_palindrome(&1001));
        assert!(is_palindrome(&51615));
        assert!(is_palindrome(&987789));
        assert!(!is_palindrome(&123));
        assert!(!is_palindrome(&1212));

        assert!(is_palindrome(&String::from("racecar")));
        assert!(!is_palindrome(&String::from("extracredit")));
    }

    #[test]
    fn collatz_sequence_works() {
        assert_eq!(
            collatz_sequence(13).collect::<Vec<u64>>(),
            vec![40, 20, 10, 5, 16, 8, 4, 2, 1]
        )
    }

    #[test]
    fn letter_counter_works() {
        assert_eq!(letter_count(342), 23);
        assert_eq!(letter_count(115), 20);
        assert_eq!(letter_count(999), 24);
        assert_eq!(letter_count(100), 10);
        assert_eq!(letter_count(110), 16);
    }
}
