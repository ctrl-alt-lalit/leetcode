use std::collections::HashSet;

fn find_all_recipes(
    recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
) -> Vec<String> {
    let avail: HashSet<_> = HashSet::from_iter(supplies.into_iter());

    let mut needed: Vec<_> = ingredients
        .into_iter()
        .map(|vi| vi.into_iter().filter(|ing| !avail.contains(ing)))
        .map(|it| HashSet::<_>::from_iter(it))
        .zip(0usize..)
        .collect();

    drop(avail);

    needed.sort_unstable_by_key(|(m, _)| -(m.len() as i32));
    let mut res = Vec::with_capacity(recipes.len());

    while !needed.is_empty() {
        let last_size = needed.last().unwrap().0.len();
        if last_size > 0 {
            break;
        }

        while let Some((m, i)) = needed.last() {
            if !m.is_empty() {
                break;
            }
            let recipe = &recipes[*i];
            needed.pop();

            for (mj, _) in needed.iter_mut().filter(|(mm, _)| !mm.is_empty()) {
                mj.remove(recipe);
            }

            res.push(recipe.clone());
        }

        needed.sort_unstable_by_key(|(m, _)| -(m.len() as i32));
    }

    return res;
}
