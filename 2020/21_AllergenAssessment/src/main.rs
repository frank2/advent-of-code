use std::io;
use std::collections::{HashMap, HashSet};

fn read_allergens() -> Result<(Vec<String>, HashMap<String, HashSet<String>>), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut ingredients = Vec::<String>::new();
    let mut allergens = HashMap::<String, HashSet<String>>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        let chunks: Vec<&str> = buffer.trim().split(" (contains ").collect();
        let ingredient_set: HashSet<String> = chunks[0].split(" ").map(|x| x.to_string()).collect();
        let allergen_vec: Vec<String> = chunks[1].split(", ").map(|x| x.replace(")", "")).collect();

        for allergen in allergen_vec {
            let entry = allergens.entry(allergen).or_insert(ingredient_set.clone());
            *entry = entry.intersection(&ingredient_set).cloned().collect::<HashSet<String>>();
        }

        ingredients.append(&mut ingredient_set.iter().cloned().collect::<Vec<String>>());

        buffer.clear();
    }

    if allergens.len() == 0 { Err(()) }
    else { Ok((ingredients, allergens)) }
}

fn main() {
    if let Ok((ingredients, allergens)) = read_allergens() {
        let allergen_set = allergens.values().cloned().reduce(|acc, x| acc.union(&x).cloned().collect::<HashSet<String>>()).unwrap();
        let full_ingredient_set: HashSet<String> = ingredients.iter().cloned().collect();
        let safe_set: HashSet<String> = full_ingredient_set.difference(&allergen_set).cloned().collect();

        println!("safe ingredients: {}", safe_set.iter().map(|x| ingredients.iter().filter(|&y| y == x).count()).sum::<usize>());

        let mut known_allergens = HashMap::<String, String>::new();

        loop {
            let solved_ingredients: HashSet<String> = known_allergens.values().cloned().collect();
            if solved_ingredients == allergen_set { break; }

            for (allergen, ingredient_set) in &allergens {
                if known_allergens.get(allergen).is_some() { continue; }

                let difference: HashSet<String> = ingredient_set.difference(&solved_ingredients).cloned().collect();

                if difference.len() == 1 {
                    known_allergens.insert(allergen.clone(), difference.iter().next().unwrap().clone());
                    break;
                }
            }
        }

        let mut sorted_allergens: Vec<(String, String)> = known_allergens.iter().map(|(k,v)| (k.clone(),v.clone())).collect();
        sorted_allergens.sort();

        let allergen_string: Vec<String> = sorted_allergens.iter().map(|(_,v)| v.clone()).collect();
        println!("{}", allergen_string.join(","));
    }
    else { panic!("couldn't read allergens!"); }
}
