use std::collections::HashSet;

use crate::utils::{read_data_text, AocResult, AocError};

pub fn find_marker(marker_size: usize) -> AocResult<(usize, String)> {
    let data = read_data_text("day6.txt")?;
    let data_chars = data.chars().collect::<Vec<_>>();
    let pair = data_chars
        .windows(marker_size)
        .enumerate()
        .find(|(_, w)| w.iter()
            .collect::<HashSet<_>>().len() == marker_size)
        .ok_or(AocError::new("no matching code"))?;
    let chars = pair.1.iter().collect::<String>();
    Ok((pair.0 + marker_size, chars))
}

pub fn day6() {
    let (idx1, chrs1) = find_marker(4).unwrap();
    println!("First code: {}", chrs1);
    println!("Index past end of first code: {}", idx1);
    let (idx2, chrs2) = find_marker(14).unwrap();
    println!("Second code: {}", chrs2);
    println!("Index past end of second code: {}", idx2);
}