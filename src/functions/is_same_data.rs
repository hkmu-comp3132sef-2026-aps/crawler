use crate::structs::school::School;

/// Compare school data.
pub fn is_same_data(
    current: &School,
    next: &School,
) -> bool {
    match (
        serde_json::to_value(current),
        serde_json::to_value(next),
    ) {
        (Ok(cur), Ok(nex)) => {
            if cur != nex {
                print!("Different found in key:");
                println!("Current: {}", cur);
                println!("Next: {}", nex);
                false
            } else {
                true
            }
        },
        _ => false
    }
}
