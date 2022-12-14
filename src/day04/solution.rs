use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn main() {
    let example = r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "#
    .trim();

    assert_eq!(camp_cleanup_reconsideration_count(example), 2);

    let path = Path::new("src/day04/input");
    assert!(path.exists());

    let contents = fs::read_to_string(path).unwrap().trim().to_owned();
    let total_reconsideration_count = camp_cleanup_reconsideration_count(&contents);

    println!("Total reconsideration count: {total_reconsideration_count}");
    // regression test for refactoring
    assert_eq!(total_reconsideration_count, 483);

    // part 2
    assert_eq!(camp_cleanup_overlap_count(example), 4);

    let total_overlap_count = camp_cleanup_overlap_count(&contents);
    println!("Total overlap count: {total_overlap_count}");
    // regression test for refactoring
    assert_eq!(total_overlap_count, 874);
}

fn camp_cleanup_overlap_count(contents: &str) -> i32 {
    let mut total_overlap_count: i32 = 0;
    for line_wspace in contents.lines() {
        let (srng1, srng2) = get_range_sets_from_line(line_wspace);
        if !srng1.is_disjoint(&srng2) {
            total_overlap_count += 1;
        }
    }
    total_overlap_count
}

fn camp_cleanup_reconsideration_count(contents: &str) -> i32 {
    let mut total_score: i32 = 0;
    for line_wspace in contents.lines() {
        let (srng1, srng2) = get_range_sets_from_line(line_wspace);
        if srng1.is_subset(&srng2) | srng2.is_subset(&srng1) {
            total_score += 1;
        }
    }
    total_score
}

fn get_range_sets_from_line(line_wspace: &str) -> (HashSet<i32>, HashSet<i32>) {
    let line = line_wspace.trim();
    let parts: Vec<&str> = line.split(",").collect();
    assert_eq!(parts.len(), 2);
    // range_from_part
    let srng1 = set_from_range_str(parts[0]);
    let srng2 = set_from_range_str(parts[1]);
    (srng1, srng2)
}

fn set_from_range_str(range_str: &str) -> HashSet<i32> {
    let start_stop: Vec<i32> = range_str
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    assert_eq!(start_stop.len(), 2);
    let range: HashSet<i32> = (start_stop[0]..=start_stop[1]).collect();
    range
}
