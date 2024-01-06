pub fn length_of_last_word(s: String) -> usize {
    let s = s.trim();

    let v: Vec<&str> = s.split(' ').collect();

    v.iter().for_each(|x| println!("{}", x));

    v.last().unwrap().len()
}
