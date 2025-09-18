#![allow(unused)]

use std::borrow::Cow::Borrowed;
use std::collections::HashMap;
use std::io::Bytes;
use crate::loops::for_loop;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

// It turns out we can do it in one-pass.
// While we are iterating and inserting elements into the hash table,
// we also look back to check if current element's complement already exists in the hash table.
// If it exists, we have found a solution and return the indices immediately.
pub fn two_sum_o_n(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for i in 0..nums.len() {
        let search_for = target - nums[i];
        if !map.contains_key(&search_for) {
            map.insert(nums[i], i as i32);
        }
        else {
            let index = *map.get(&search_for).unwrap();
            if i as i32 != index {
                return vec![i as i32, index];
            }
        }
    }

    unreachable!()
}

// Longest Substring Without Repeating Characters
// abcabcbb => abc, 3
// bbbbbb => b, 1
// pwwkew => wke, 3
// dvdf => vdf, 3
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut last_seen = [-1; 256];
    let mut left = 0;
    let mut max_len = 0;

    for (right, ch) in s.chars().enumerate() {
        let idx = ch as usize;

        if last_seen[idx] >= left as i32 {
            left = (last_seen[idx] + 1) as usize;
        }

        last_seen[idx] = right as i32;
        max_len = max_len.max(right - left + 1);
    }
    
    max_len  as i32
}

// Given an integer x, return true if x is a palindrome, and false otherwise.
// 121 => true
// -121 => false
// 10 => false
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    
    if x < 10 {
        return true;
    }

    let mut digits = Vec::new();
    let mut n = x;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    
    for (index, digit) in (0.. digits.len() / 2).enumerate() {
        if digits[index] != digits[digits.len() - 1 - index] {
            return false;
        }
    }
    
    true
}


// Write a function to find the longest common prefix string amongst an array of strings.
// strs = ["flower","flow","flight"] => "fl"
// strs = ["dog","racecar","car"] => ""
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = strs.first().unwrap().bytes();
    for word in strs.iter().skip(1) {
        if word.is_empty() {
            return "".to_string();
        }
        
        let common_bytes = word.bytes()
            .zip(prefix)
            .take_while(|&(a, b)| a == b)
            .count();

        prefix = word[..common_bytes].bytes();
    }

    prefix.map(|b| b as char).collect()
}

// Given a signed 32-bit integer x, return x with its digits reversed.
// If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
// 123 => 321
// -123 => -321
// 120 => 21
pub fn reverse(x: i32) -> i32 {
    let mut digits = Vec::new();
    let mut n = if x < 0 { -x } else { x };
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    let number = digits
        .into_iter()
        .try_fold(0i32, |acc, digit| acc.checked_mul(10)?.checked_add(digit));
    
    match number {
        None => 0,
        Some(number) => if x < 0 { -number } else { number },
    }
}


// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
// such that i != j, i != k, and j != k,
// and nums[i] + nums[j] + nums[k] == 0.
// Notice that the solution set must not contain duplicate triplets.
// [-1,0,1,2,-1,-4] => [[-1,-1,2],[-1,0,1]]
// [0,1,1] => []
// [0,0,0] => [[0,0,0]]
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut numbers = nums;
    numbers.sort();
    
    for i in 0..numbers.len() {
        if i > 0 && numbers[i] == numbers[i - 1] {
            continue;   // skip duplicates for the first element
        }
        
        let mut left = i + 1;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[i] + numbers[left] + numbers[right];
            if sum == 0 {
                result.push(vec![numbers[i], numbers[left], numbers[right]]);
                
                // skip duplicates for left
                while left < right && numbers[left] == numbers[left + 1] {
                    left += 1;
                }
                
                // skip duplicates for right
                while left < right && numbers[right] == numbers[right - 1] {
                    right -= 1;
                }
                
                left += 1;
                right -= 1;
            }
            else if sum < 0 {
                left += 1;
            }
            else {
                right -= 1;
            }
        }
    }
    
    result
}

// [0,0,1,1,1,2,2,3,3,4] => [0,1,2,3,4,_,_,_,_,_]
// remove_duplicates(&mut [0, 0, 1, 1, 1, 2, 2, 3, 3, 4].to_vec());
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 1;
    }

    let mut k = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[k - 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }
    
    k as i32
}

// Two strings s and t are isomorphic if the characters in s can be replaced to get t.
// s = "egg", t = "add" => true
// s = "paper", t = "title" => true
// s = "foo", t = "bar" => false, as 'o' needs to be mapped to both 'a' and 'r'.

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();
    
    for (cs, ct) in s.chars().zip(t.chars()) {
        if let Some(&mapped) = map_s.get(&cs) {
            if mapped != ct {
                return false;
            }
        } else {
            map_s.insert(cs, ct);
        }

        if let Some(&mapped) = map_t.get(&ct) {
            if mapped != cs {
                return false;
            }
        } else {
            map_t.insert(ct, cs);
        }
    }

    true
}

// "()" => true
// "()[]{}" => true
// "(]" => false
// "([])" => true
// "([)]" => false
pub fn is_valid_parenthesis(s: String) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => if stack.pop() != Some('(') {  return false },
            ']' => if stack.pop() != Some('{') {  return false },
            '}' => if stack.pop() != Some('{') {  return false },
            _ => { return false },
        }
    }

    stack.is_empty()
}

// valid numbers:
// "2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789", "46.e3"
// invalid numbers:
// "abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", ".", "+.", "-.E3", "92e1740e91", "4e+"
//
// Formally, a valid number is:
// An integer number followed by an optional exponent.
// A decimal number followed by an optional exponent.
// An integer number is defined with an optional sign '-' or '+' followed by digits.
// A decimal number is defined with an optional sign '-' or '+' followed by one of the following definitions:
// Digits followed by a dot '.'.
// Digits followed by a dot '.' followed by digits.
// A dot '.' followed by digits.
// An exponent is defined with an exponent notation 'e' or 'E' followed by an integer number.
//
// The digits are defined as one or more digits.
pub fn is_number(s: String) -> bool {
    let len = s.len();
    let mut digit_found = false;
    let mut dot_found = false;
    let mut exp_found = false;
    let mut prev = ' ';
    for (i, c) in s.char_indices() {
        match c {
            c if c.is_ascii_digit() => { digit_found = true },
            '+' | '-' => {
                if (i == len -1) || (i != 0 && prev.to_ascii_lowercase() != 'e') { return false }
            },
            '.' => {
                if dot_found || exp_found { return false } else { dot_found = true }
            }
            'e' | 'E' => {
                if digit_found
                    && !exp_found
                    && (i + 1 < len)
                    && (prev.is_digit(10) || dot_found) {
                    exp_found = true
                } else {
                    return false
                }
            }
            _ => { return false }
        }

        prev = c;
    }

    digit_found
}


















