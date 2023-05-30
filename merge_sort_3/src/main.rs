// iFarbod 2023

use std::io;

fn merge_sort(arr: &mut [i32], print_steps: bool)
{
    let n = arr.len();
    if n > 1
    {
        let mid1 = n / 3;
        let mid2 = 2 * n / 3;

        if print_steps
        {
            println!("First third: {:?}", &arr[..mid1]);
        }
        merge_sort(&mut arr[..mid1], print_steps);
        if print_steps
        {
            println!("First third sorted: {:?}", &arr[..mid1]);
        }

        if print_steps
        {
            // println!("Second third: {:?}", &arr[mid1..mid2]);
        }
        merge_sort(&mut arr[mid1..mid2], print_steps);
        if print_steps
        {
            println!("Second third sorted: {:?}", &arr[mid1..mid2]);
        }

        if print_steps
        {
            // println!("Third third: {:?}", &arr[mid2..]);
        }
        merge_sort(&mut arr[mid2..], print_steps);
        if print_steps
        {
            println!("Third third sorted: {:?}", &arr[mid2..]);
        }

        let mut i = 0;
        let mut j = mid1;
        let mut k = mid2;
        let mut tmp = Vec::with_capacity(n);

        while i < mid1 && j < mid2 && k < n
        {
            if arr[i] < arr[j]
            {
                if arr[i] < arr[k]
                {
                    tmp.push(arr[i]);
                    i += 1;
                }
                else
                {
                    tmp.push(arr[k]);
                    k += 1;
                }
            }
            else
            {
                if arr[j] < arr[k]
                {
                    tmp.push(arr[j]);
                    j += 1;
                }
                else
                {
                    tmp.push(arr[k]);
                    k += 1;
                }
            }
        }

        while i < mid1 && j < mid2
        {
            if arr[i] < arr[j]
            {
                tmp.push(arr[i]);
                i += 1;
            }
            else
            {
                tmp.push(arr[j]);
                j += 1;
            }
        }

        while j < mid2 && k < n
        {
            if arr[j] < arr[k]
            {
                tmp.push(arr[j]);
                j += 1;
            }
            else
            {
                tmp.push(arr[k]);
                k += 1;
            }
        }

        while i < mid1 && k < n
        {
            if arr[i] < arr[k]
            {
                tmp.push(arr[i]);
                i += 1;
            }
            else
            {
                tmp.push(arr[k]);
                k += 1;
            }
        }

        while i < mid1
        {
            tmp.push(arr[i]);
            i += 1;
        }

        while j < mid2
        {
            tmp.push(arr[j]);
            j += 1;
        }

        while k < n
        {
            tmp.push(arr[k]);
            k += 1;
        }

        for i in 0..n
        {
            arr[i] = tmp[i];
        }

        if print_steps
        {
            println!("Merged: {:?}", &arr);
        }
    }
}

pub fn main()
{
    println!("Enter an array of numbers:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // let mut arr = vec![104, 60, 78, 99, 12, 15, 7, 8, 9, 1, 5, 90];
    // 9 4 7 2 8 5 1 3 6

    let mut arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Original array: {:?}", &arr);
    merge_sort(&mut arr, true);
    println!("Sorted array: {:?}", &arr);
}
