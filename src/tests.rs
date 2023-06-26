use super::binary_search;

#[test]
fn binary_search_works() {
    let array = [
        "Agapi", "Andrew", "Bonnie", "Cathy", "Dominic", "Edward", "Huang", "Jordon", "Joshua",
        "Le", "Mason", "Ngor", "Oscar", "Tim", "Ursula", "Vicky", "Vinnie",
    ];

    // First item
    let (index, step_count) =
        binary_search(&array, &"Agapi").expect("'Agapi' should be found in array");
    assert_eq!(
        array.get(index),
        Some(&"Agapi"),
        "Item at index should be 'Agapi'"
    );
    assert_eq!(index, 0, "Index of 'Agapi' should be 0");
    assert_eq!(step_count, 5, "Step count of 'Agapi' should be 5");

    // Arbitrary item
    let (index, step_count) =
        binary_search(&array, &"Huang").expect("'Huang' should be found in array");
    assert_eq!(
        array.get(index),
        Some(&"Huang"),
        "Item at index should be 'Huang'"
    );
    assert_eq!(index, 6, "Index of 'Huang' should be 6");
    assert_eq!(step_count, 3, "Step count of 'Huang' should be 3");

    // Middle item
    let (index, step_count) =
        binary_search(&array, &"Joshua").expect("'Joshua' should be found in array");
    assert_eq!(
        array.get(index),
        Some(&"Joshua"),
        "Item at index should be 'Joshua'"
    );
    assert_eq!(index, 8, "Index of 'Joshua' should be 8");
    assert_eq!(step_count, 1, "Step count of 'Joshua' should be 1");

    // Last item
    let (index, step_count) =
        binary_search(&array, &"Vinnie").expect("'Vinnie' should be found in array");
    assert_eq!(
        array.get(index),
        Some(&"Vinnie"),
        "Item at index should be 'Vinnie'"
    );
    assert_eq!(index, 16, "Index of 'Vinnie' should be 16");
    assert_eq!(step_count, 5, "Step count of 'Vinnie' should be 5");

    // Non-existant item
    assert_eq!(
        binary_search(&array, &"bruh"),
        None,
        "'bruh' should not be found in array"
    );
    // non-existant item (would be first)
    assert_eq!(
        binary_search(&array, &"aaa"),
        None,
        "'aaa' should not be found in array"
    );
    // non-existant item (would be last)
    assert_eq!(
        binary_search(&array, &"zzz"),
        None,
        "'zzz' should not be found in array"
    );
}
