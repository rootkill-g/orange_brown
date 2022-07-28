fn main() {
    for c in "SurpRIsE InbOuND"
        .chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase())
    {
        println!("{}", c);
    }
}
