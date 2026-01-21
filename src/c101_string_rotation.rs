
pub fn group_rotations(strs: Vec<String>) -> Vec<Vec<String>> {
    if strs.len() == 0 {
        return vec![];
    };

    let mut not_yet_sorted: Vec<String> = strs.iter().cloned().collect();
    let mut result: Vec<Vec<String>> = Vec::new();

    let mut group: Vec<String> = Vec::new();
    let mut check_fixes: Vec<String> = Vec::new();
    let mut check_full: Vec<String> = Vec::new();

    while not_yet_sorted.len() > 0 {

        for strg in not_yet_sorted.iter() {
            let mut it = strg.chars();
            it.next_back();
            let prefix: String = it.collect();
            let suffix: String = strg.chars().skip(1).collect();

            if group.len() == 0 {
                group.push(strg.clone());
                check_fixes.push(prefix.clone());
                check_fixes.push(suffix.clone());
                check_full.push(strg.clone());
            } else {
                if check_fixes.contains(&prefix) && prefix.len() > 0 {
                    group.push(strg.clone());
                    if !check_fixes.contains(&suffix) && suffix.len() > 0 {
                        check_fixes.push(suffix.clone());
                    };
                } else if check_fixes.contains(&suffix) && suffix.len() > 0 {
                    group.push(strg.clone());
                    if !check_fixes.contains(&prefix) && prefix.len() > 0 {
                        check_fixes.push(prefix.clone());
                    };
                } else if check_full.contains(&strg.clone()) {
                    group.push(strg.clone());
                };
            };
            
        };

        not_yet_sorted.retain(|s| !group.contains(s));
        result.push(group);

        check_fixes = Vec::new();
        check_full = Vec::new();
        group = Vec::new();

    };

    return result;
}



#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_groups(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in groups.iter_mut() {
            group.sort();
        }
        groups.sort_by_key(|g| g[0].clone());
        groups
    }

    // --- given tests (keep yours as-is) ---

    #[test]
    fn test_basic_rotations() {
        let input = vec![
            "abc".to_string(),
            "bca".to_string(),
            "cab".to_string(),
            "xyz".to_string(),
            "yzx".to_string(),
            "zxy".to_string(),
        ];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![
            vec!["abc".to_string(), "bca".to_string(), "cab".to_string()],
            vec!["xyz".to_string(), "yzx".to_string(), "zxy".to_string()],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_rotations() {
        let input = vec!["hello".to_string(), "world".to_string()];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![vec!["hello".to_string()], vec!["world".to_string()]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_duplicates() {
        let input = vec!["abc".to_string(), "abc".to_string(), "bca".to_string()];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![vec![
            "abc".to_string(),
            "abc".to_string(),
            "bca".to_string(),
        ]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_char() {
        let input = vec!["a".to_string(), "a".to_string(), "b".to_string()];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![vec!["a".to_string(), "a".to_string()], vec!["b".to_string()]]);
        assert_eq!(result, expected);
    }

    // --- additional tests ---

    #[test]
    fn test_empty_input() {
        let input: Vec<String> = vec![];
        let result = group_rotations(input);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_string() {
        let input = vec!["rotation".to_string()];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![vec!["rotation".to_string()]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_in_one_group_with_repeats() {
        let input = vec![
            "aaaa".to_string(),
            "aaaa".to_string(),
            "aaaa".to_string(),
        ];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![vec![
            "aaaa".to_string(),
            "aaaa".to_string(),
            "aaaa".to_string(),
        ]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_lengths_should_not_merge_groups() {
        // Rotations must be same length; these should not be grouped together.
        let input = vec![
            "ab".to_string(),
            "ba".to_string(),   // rotation of "ab"
            "aba".to_string(),
            "baa".to_string(),  // rotation of "aba"? ("aba" rotations: "aba","baa","aab") so yes "baa" is a rotation
        ];
        let mut result = group_rotations(input);
        result = sorted_groups(result);

        let expected = sorted_groups(vec![
            vec!["ab".to_string(), "ba".to_string()],
            vec!["aba".to_string(), "baa".to_string()],
        ]);

        assert_eq!(result, expected);
    }


    #[test]
    fn test_two_separate_rotation_classes_same_length() {
        let input = vec![
            "abcd".to_string(),
            "bcda".to_string(),
            "cdab".to_string(),
            "dabc".to_string(),
            "abdc".to_string(),
            "bdca".to_string(),
            "dcab".to_string(),
            "cabd".to_string(),
        ];
        // "abcd" group = all its rotations
        // "abdc" group = all its rotations (different class)
        let mut result = group_rotations(input);
        result = sorted_groups(result);

        let expected = sorted_groups(vec![
            vec![
                "abcd".to_string(),
                "bcda".to_string(),
                "cdab".to_string(),
                "dabc".to_string(),
            ],
            vec![
                "abdc".to_string(),
                "bdca".to_string(),
                "dcab".to_string(),
                "cabd".to_string(),
            ],
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rotation_wraparound() {
        // Ensure wrap-around shifts are handled (last chars to front).
        let input = vec![
            "zzza".to_string(),
            "azzz".to_string(),
            "zzaz".to_string(),
            "zazz".to_string(),
        ];
        let mut result = group_rotations(input);
        result = sorted_groups(result);

        let expected = sorted_groups(vec![vec![
            "zzza".to_string(),
            "azzz".to_string(),
            "zzaz".to_string(),
            "zazz".to_string(),
        ]]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_many_groups_with_noise_order() {
        let input = vec![
            "bca".to_string(),
            "world".to_string(),
            "cab".to_string(),
            "hello".to_string(),
            "abc".to_string(),
            "dlrow".to_string(), // NOT a rotation of "world" (it's reversed)
        ];
        let mut result = group_rotations(input);
        result = sorted_groups(result);

        let expected = sorted_groups(vec![
            vec!["abc".to_string(), "bca".to_string(), "cab".to_string()],
            vec!["hello".to_string()],
            vec!["world".to_string()],
            vec!["dlrow".to_string()],
        ]);

        assert_eq!(result, expected);
    }
}
