// Rust Bytes Challenge Issue #100 Merging Magical Portals

pub fn merge_portals(input: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    for &(x, y) in &input{
        assert!(x < y, "tuples not ordered");
    }

    if input.is_empty() {
        return input;
    };


    let mut sorted_input: Vec<&(i64, i64)> = input.iter().collect();
    sorted_input.sort_by_key(|&&(x, _)| x);
    let mut result = Vec::new();

    let mut prev = *sorted_input[0];
    let mut building_tuple: bool = false;

    for (i, &&(x, y)) in sorted_input.iter().enumerate() {
        match sorted_input.get(i + 1) {
            Some(&&(next_x, next_y)) => {
                if y < next_x {
                    result.push(prev);
                    prev.0 = next_x;
                    prev.1 = next_y;
                    building_tuple = false;
                    continue;
                } else {
                    if !building_tuple {
                        prev.0 = x;
                    }
                    if prev.1 < next_y {
                        prev.1 = next_y;
                    };
                    building_tuple = true;
                }
            }
            None => {
                result.push(prev);
                return result
            }
        }
    }

    return result;
}


#[test]
fn test_no_portals() {
    assert_eq!(merge_portals(vec![]), vec![]);
}

#[test]
fn test_single_portal() {
    assert_eq!(merge_portals(vec![(5, 10)]), vec![(5, 10)]);
}

#[test]
fn test_no_overlap() {
    let input = vec![(1, 2), (3, 4), (5, 6)];
    let expected = vec![(1, 2), (3, 4), (5, 6)];
    assert_eq!(merge_portals(input), expected);
}

#[test]
fn test_simple_overlap() {
    let input = vec![(1, 3), (2, 4)];
    let expected = vec![(1, 4)];
    assert_eq!(merge_portals(input), expected);
}

#[test]
fn test_touching_edges() {
    let input = vec![(1, 3), (3, 5), (5, 7)];
    let expected = vec![(1, 7)];
    assert_eq!(merge_portals(input), expected);
}

#[test]
fn test_multiple_merges() {
    let input = vec![(6, 8), (1, 5), (2, 4), (7, 9), (10, 12)];
    let expected = vec![(1, 5), (6, 9), (10, 12)];
    assert_eq!(merge_portals(input), expected);
}

#[test]
fn test_all_overlap() {
    let input = vec![(1, 10), (2, 9), (3, 8), (4, 7)];
    let expected = vec![(1, 10)];
    assert_eq!(merge_portals(input), expected);
}

#[test]
fn test_unsorted_input() {
    let input = vec![(5, 6), (1, 3), (2, 4)];
    let expected = vec![(1, 4), (5, 6)];
    assert_eq!(merge_portals(input), expected);
}

