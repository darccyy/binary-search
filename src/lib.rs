#[cfg(test)]
mod tests;

use std::cmp::Ordering;

// Recursively search *sorted* array for target
//
// Returns index of target, and amount of steps taken (recusive function calls)
//
// Returns `None` if target not found in array
pub fn binary_search<T: Ord>(array: &[T], target: &T) -> Option<(usize, usize)> {
    // Set initial step count to 1
    // Set initial amount of items removed to left, to 0
    binary_search_step(array, target, 1, 0)
}

// Recursively search part of sorted array
fn binary_search_step<T: Ord>(
    array: &[T],
    target: &T,
    step_count: usize,
    // Amount of items removed to left of guess
    // To return correct index, for entire array
    left_removed_count: usize,
) -> Option<(usize, usize)> {
    // Array only has one item
    if array.len() < 2 {
        // If array has 1 item, and item matches target
        // Return correct index and step count
        if let Some(only_item) = array.first() {
            if only_item == target {
                return Some((left_removed_count, step_count));
            }
        }
        // Item not found
        return None;
    }

    // Get item at middle of array
    let half_index = array.len() / 2;
    let guess = array.get(half_index)?;

    // Split array in halves (former and latter)
    let (former, latter) = array.split_at(half_index);

    // Compare target with guess
    let (half_array, new_left_removed) = match target.cmp(guess) {
        // Target matches!
        // Return correct index and step count
        Ordering::Equal => return Some((left_removed_count + half_index, step_count)),

        // Target is 'less' than guess
        // It will be found to the left (before guess in array)
        Ordering::Less => (former, 0),

        // Target is 'greater' than guess
        // It will be found to the right (after guess in array)
        Ordering::Greater => (latter, half_index),
    };

    // Recurse function
    binary_search_step(
        // Use chosen half of array
        half_array,
        target,
        // Increase step count
        step_count + 1,
        // Add length of removed items left of guess
        left_removed_count + new_left_removed,
    )
}
