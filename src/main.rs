use binary_search::binary_search;

fn main() {
    let array = [
        "Agapi", "Andrew", "Bonnie", "Cathy", "Dominic", "Edward", "Huang", "Jordon", "Joshua",
        "Le", "Mason", "Ngor", "Oscar", "Tim", "Ursula", "Vicky", "Vinnie",
    ];

    let target = "Vinnie";

    let (index, step_count) = binary_search(&array, &target).expect("Not found in array");

    let item = array.get(index).expect("Index not in array");

    println!("Found '{}' (index {}) in {} steps", item, index, step_count,);
    assert_eq!(item, &target, "Incorrect item index found");
}
