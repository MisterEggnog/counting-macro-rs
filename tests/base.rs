use bump_counting_macro::*;

#[test]
fn shared_counter() {
    counter_create!(count);

    // Get the value of the counter & increment
    assert_eq!(counter_bump!(count), 0);

    // Get the value of the counter without incrementing
    assert_eq!(counter_peek!(count), 1);

    // Change the value of the counter
    counter_set!(count, 12);
    assert_eq!(counter_bump!(count), 12);
}
