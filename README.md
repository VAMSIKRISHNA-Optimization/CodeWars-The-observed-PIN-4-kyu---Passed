# CodeWars-The-observed-PIN-4-kyu---Passed
Alright, detective, one of our colleagues successfully observed our target person, Robby the robber. We followed him to a secret warehouse, where we assume to find all the stolen stuff. The door to this warehouse is secured by an electronic combination lock. Unfortunately our spy isn't sure about the PIN he saw, when Robby entered it.

The keypad has the following layout:

┌───┬───┬───┐
│ 1 │ 2 │ 3 │
├───┼───┼───┤
│ 4 │ 5 │ 6 │
├───┼───┼───┤
│ 7 │ 8 │ 9 │
└───┼───┼───┘
    │ 0 │
    └───┘
He noted the PIN 1357, but he also said, it is possible that each of the digits he saw could actually be another adjacent digit (horizontally or vertically, but not diagonally). E.g. instead of the 1 it could also be the 2 or 4. And instead of the 5 it could also be the 2, 4, 6 or 8.

He also mentioned, he knows this kind of locks. You can enter an unlimited amount of wrong PINs, they never finally lock the system or sound the alarm. That's why we can try out all possible (*) variations.

* possible in sense of: the observed PIN itself and all variations considering the adjacent digits

Can you help us to find all those variations? It would be nice to have a function, that returns an array (or a list in Java/Kotlin and C#) of all variations for an observed PIN with a length of 1 to 8 digits. We could name the function getPINs (get_pins in python, GetPINs in C#). But please note that all PINs, the observed one and also the results, must be strings, because of potentially leading '0's. We already prepared some test cases for you.

Detective, we are counting on you!


#[cfg(test)]
mod tests {
    use super::get_pins;
    use rand::{thread_rng, Rng};
    use itertools::Itertools;
    
    
    fn reference_solution(s: &str) -> Vec<String> {
        if s.len() == 1{
            match s {
                "1" => return vec![String::from("1"), String::from("2"), String::from("4")],
                "2" => return vec![String::from("2"), String::from("1"), String::from("3"), String::from("5")],
                "3" => return vec![String::from("3"), String::from("2"), String::from("6")],
                "4" => return vec![String::from("4"), String::from("1"), String::from("5"), String::from("7")],
                "5" => return vec![String::from("5"), String::from("2"), String::from("4"), String::from("6"), String::from("8")],
                "6" => return vec![String::from("6"), String::from("3"), String::from("5"), String::from("9")],
                "7" => return vec![String::from("7"), String::from("4"), String::from("8")],
                "8" => return vec![String::from("8"), String::from("0"), String::from("5"), String::from("7"), String::from("9")],
                "9" => return vec![String::from("9"), String::from("6"), String::from("8")],
                _ =>   return vec![String::from("0"), String::from("8")],
            }
        }
        let mut v = Vec::new();
        let p = get_pins(&s[1..][..]);
        reference_solution(&String::from(s.chars().next().unwrap()))
        .iter()
        .for_each(|x| {
            p.iter()
            .for_each(|e| {
                v.push(format!("{}{}", x, e))
            });
        });
        v
    }

    #[test]
    fn sample_tests() {
        assert_eq!(get_pins("8").iter().sorted().collect::<Vec<&String>>(), 
            vec!["0", "5", "7", "8", "9"]);
        assert_eq!(get_pins("11").iter().sorted().collect::<Vec<&String>>(), 
            vec!["11", "12", "14",  "21", "22", "24",  "41", "42", "44"]);
        assert_eq!(get_pins("369").iter().sorted().collect::<Vec<&String>>(), 
            vec!["236", "238", "239",  "256", "258", "259",  "266", "268", "269",  "296", "298", "299", 
                "336", "338", "339",  "356", "358", "359",  "366", "368", "369",  "396", "398", "399", 
                "636", "638", "639",  "656", "658", "659",  "666", "668", "669",  "696", "698", "699"]);
    }
    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        let digits = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for _ in 0..30 {
            let l = rng.gen_range(1..=8);
            let mut s = String::from("");
            for _ in 0..l {
                s.push(digits[rng.gen_range(0..10)])
            }
            assert_eq!(get_pins(&s).iter().sorted().collect::<Vec<&String>>(), 
                reference_solution(&s).iter().sorted().collect::<Vec<&String>>());
        }
    }
}
