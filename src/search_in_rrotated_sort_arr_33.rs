/// There is an integer array nums sorted in ascending order (with distinct values).
///
/// Prior to being passed to your function, nums is possibly rotated at an unknown pivot
/// index k (1 <= k < nums.length) such that the resulting array is
/// [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed).
///
/// For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
/// Given the array nums after the possible rotation and an integer target,
/// return the index of target if it is in nums, or -1 if it is not in nums.
///
/// You must write an algorithm with O(log n) runtime complexity.
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    _search(&nums, target).map(|n| n as i32).unwrap_or(-1)
}

fn _search(nums: &[i32], target: i32) -> Option<usize> {
    match find_pivot(nums) {
        Some(pivot) => {
            let left = &nums[..pivot];
            let right = &nums[pivot..];

            target_index_in_sorted_slice(left, target)
                .or_else(|| target_index_in_sorted_slice(right, target).map(|r| r + left.len()))
        }
        None => target_index_in_sorted_slice(nums, target),
    }
}

pub fn find_pivot(nums: &[i32]) -> Option<usize> {
    _find_pivot(nums, 0)
}

fn _find_pivot(nums: &[i32], start: usize) -> Option<usize> {
    let len = nums.len();
    match len {
        0 | 1 => None,
        2.. if nums[0] <= nums[len - 1] => None,
        _ => {
            // len >= 2 and 100% have pivot somewhere in the middle
            let mid = nums.len() / 2;
            let (left, right) = (&nums[..mid], &nums[mid..]);

            // pivot is right at the junction
            if left[left.len() - 1] > right[0] {
                return Some(start + left.len());
            }

            _find_pivot(left, start).or(_find_pivot(right, start + mid))
        }
    }
}

fn target_index_in_sorted_slice(nums: &[i32], target: i32) -> Option<usize> {
    nums.binary_search(&target).ok()
}

#[cfg(test)]
mod test {
    use super::*;

    const EMPTY: &[i32] = &[];
    const SINGLE: &[i32] = &[1];
    const DOUBLE: &[i32] = &[1, 3];
    const DOUBLE_REV: &[i32] = &[3, 1];
    const WITHOUT_PIVOT: &[i32] = &[0, 1, 2, 4, 5, 6, 7];
    const WITH_PIVOT: &[i32] = &[4, 5, 6, 7, 0, 1, 2];

    #[test]
    fn empty() {
        assert_eq!(_search(EMPTY, 0), None);
    }

    #[test]
    fn single_none() {
        assert_eq!(_search(SINGLE, 0), None);
    }

    #[test]
    fn single() {
        assert_eq!(_search(SINGLE, 1), Some(0));
    }

    #[test]
    fn double_none() {
        assert_eq!(_search(DOUBLE, 2), None);
    }

    #[test]
    fn double1() {
        assert_eq!(_search(DOUBLE, 1), Some(0));
    }

    #[test]
    fn double2() {
        assert_eq!(_search(DOUBLE, 3), Some(1));
    }

    #[test]
    fn double_none_rev() {
        assert_eq!(_search(DOUBLE_REV, 2), None);
    }

    #[test]
    fn double1_rev() {
        assert_eq!(_search(DOUBLE_REV, 1), Some(1));
    }

    #[test]
    fn double2_rev() {
        assert_eq!(_search(DOUBLE_REV, 3), Some(0));
    }

    #[test]
    fn with_pivot_none() {
        assert_eq!(_search(WITH_PIVOT, 3), None);
    }

    #[test]
    fn with_pivot() {
        for (u, i) in WITH_PIVOT.iter().enumerate() {
            assert_eq!(_search(WITH_PIVOT, *i), Some(u));
        }
    }
}
