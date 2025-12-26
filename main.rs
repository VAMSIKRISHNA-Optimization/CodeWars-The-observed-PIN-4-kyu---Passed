fn get_pins(observed: &str) -> Vec<String> 
{
    // Immutable variables
    let mut ans      : Vec<String>  = Vec::new();

    // Mutable variables
    let mut all_combs: Vec<Vec<u8>> = Vec::new(); 
    let mut digits   : Vec<u8>      = Vec::new();

    // Convert characters from string into u8 and push it to vec
    for character in observed.chars().filter(|c| c.is_digit(10)) 
    {
        digits.push(character.to_digit(10).unwrap() as u8);
    }

    // Get the alternative numbers for each number and push it to a vec
    for digit in digits.clone()
    {
        all_combs.push(get_alternative_nums(digit));
    }

    // Map each sub-vector to its length and collect into a Vec<usize>
    let sub_lengths: Vec<usize> = all_combs.clone().iter()
                                  .map(|sub_vec| sub_vec.len())
                                  .collect();
    

    let max_combos = sub_lengths.clone().iter().product::<usize>();
    let mut sub_ind  = vec![0;all_combs.len()];

    let mut loop_count = 0;
    'outer: loop
    {

        //println!("Loop Count: {:?}", loop_count);
        // for ind_val in 0..1
        // {
            //println!("Sub-Ind: {:?}", sub_ind);
            let mut alt_pin_string = String::new();
            for nind_val in 0..all_combs.len()
            {
                let number_extracted = all_combs[nind_val][sub_ind[nind_val]];
                alt_pin_string.push_str(&number_extracted.to_string());
            }

            if !alt_pin_string.is_empty()
            {
                ans.push(alt_pin_string);
            }

        // }

        let mut incremented = false;
        for ind_val in (0..=all_combs.len() - 1).rev() 
        {
            if !incremented
            {
                sub_ind[ind_val] += 1;
                incremented = true;

                if sub_ind[ind_val] >= all_combs[ind_val].len()
                {
                    sub_ind[ind_val] = 0;
                    incremented = false;
                }
                

            }

            // Condition to break the loop
            if sub_ind[0] >= all_combs[0].len()
            {
                break 'outer;
            }
        }

        loop_count += 1;

        if loop_count >= max_combos as i32
        {
            break 'outer;
        }


    }

    return ans;
}


fn get_alternative_nums(cur_num: u8)->Vec<u8>
{
    let mut alternative_nums: Vec<u8> = Vec::new();
    match cur_num
    {
        1 => alternative_nums.extend(&[1,2,4]),
        2 => alternative_nums.extend(&[1,2,3,5]),
        3 => alternative_nums.extend(&[2,3,6]),
        4 => alternative_nums.extend(&[1,4,5,7]),
        5 => alternative_nums.extend(&[2,4,5,6,8]),
        6 => alternative_nums.extend(&[3,5,6,9]),
        7 => alternative_nums.extend(&[4,7,8]),
        8 => alternative_nums.extend(&[0,5,7,8,9]),
        9 => alternative_nums.extend(&[6,8,9]),
        0 => alternative_nums.extend(&[0,8]),
        _ => alternative_nums.push(111),
    }
    alternative_nums
}



fn main() 
{
    //println!("{:?}", get_alternative_nums(8));
    println!("{:?}", get_pins("369"));
}


