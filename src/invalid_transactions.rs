use std::collections::{BTreeMap, HashMap};

pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
    let mut m: HashMap<String, BTreeMap<i32, Vec<String>>> = HashMap::new();
    let mut invalids: HashMap<String, i32> = HashMap::new();

    for s in transactions.iter() {
        let (name, time, _, city) = parse(s);
        let tree = m.entry(name).or_default();

        tree.entry(time)
            .and_modify(|v| v.push(city.clone()))
            .or_insert(vec![city]);
    }

    for s in transactions.into_iter() {
        let (name, time, amt, city) = parse(&s);
        let mut valid = (amt <= 1000);

        if valid {
            let tree = m.get(&name).unwrap();
            for (_, old_cities) in tree.range((time - 60)..=(time + 60)) {
                if old_cities.iter().any(|old_city| old_city != &city) {
                    valid = false;
                    break;
                }
            }
        }

        if !valid {
            invalids.entry(s).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    let mut res = Vec::with_capacity(invalids.len());
    for (s, x) in invalids.into_iter() {
        (0..(x - 1)).for_each(|_| res.push(s.clone()));
        res.push(s);
    }
    return res;
}

fn parse(s: &String) -> (String, i32, i32, String) {
    let mut it = s.split(',');
    let name = String::from(it.next().unwrap());
    let time: i32 = it.next().unwrap().parse().unwrap();
    let amt: i32 = it.next().unwrap().parse().unwrap();
    let city = String::from(it.next().unwrap());
    return (name, time, amt, city);
}
