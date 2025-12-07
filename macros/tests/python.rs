use macros::{comp};

#[test]
fn comp_work() {
    let base = vec![1, 2, 3];
    let a: Vec<_> = comp![x*2 for x in base if x > 1].collect();
    assert_eq!(a, vec![4, 6]);

    let base = vec![1, 2, 3];
    let a: Vec<_> = comp![x + 2 for x in base if x > 2].collect();
    assert_eq!(a, vec![5]);

    let base = vec![1, 2, 3];
    let result: Vec<_> = comp![x for x in base].collect();
    assert_eq!(result, vec![1, 2, 3]);

    let base = vec![1, 2, 3];
    let result: Vec<_> = comp![x for x in base].collect();
    assert_eq!(result, vec![1, 2, 3]);

    let base = vec![1, 2, 3];
    let a: Vec<_> = comp![x + 2 for x in base if false].collect();
    assert_eq!(a, vec![]);

    let base = vec![1, 2, 3];
    let a: Vec<_> = comp![x + 2 for x in base if x > 1 if x < 3].collect();
    assert_eq!(a, vec![4]);

    let base = vec![1, 2, 3];
    let a: Vec<_> = comp![x + 2 for x in base if x > 1 && x < 2].collect();
    assert_eq!(a, vec![]);
}
