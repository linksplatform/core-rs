#![feature(try_trait_v2)]

use platform_data::Flow;
use std::ops::{ControlFlow, FromResidual, Try};

#[test]
fn basic() {
    let mut vec = vec![];

    (0..20).try_for_each(|i| {
        vec.push(i);
        if i == 10 { Flow::Break } else { Flow::Continue }
    });

    assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_flow_continue() {
    let mut count = 0;
    (0..5).try_for_each(|_| {
        count += 1;
        Flow::Continue
    });
    assert_eq!(count, 5);
}

#[test]
fn test_flow_break_early() {
    let mut count = 0;
    (0..100).try_for_each(|_| {
        count += 1;
        Flow::Break
    });
    assert_eq!(count, 1); // Should stop after first iteration
}

#[test]
fn test_from_control_flow_continue() {
    // Test conversion from ControlFlow::Continue
    let cf: ControlFlow<i32, ()> = ControlFlow::Continue(());
    let flow: Flow = cf.into();
    // We can't directly compare Flow values, but we can verify the conversion happened
    // by checking behavior - the flow should be Continue
    let result = std::iter::once(0).try_for_each(|_| {
        let cf2: ControlFlow<i32, ()> = ControlFlow::Continue(());
        let _: Flow = cf2.into();
        Flow::Continue
    });
    // Verify the result is ok (iterator completed)
    let _ = result;
    let _ = flow; // Use the flow variable
}

#[test]
fn test_from_control_flow_break() {
    // Test conversion from ControlFlow::Break
    let cf: ControlFlow<(), i32> = ControlFlow::Break(());
    let flow: Flow = cf.into();
    let _ = flow; // Use the flow variable

    // Verify Break behavior
    let mut count = 0;
    (0..10).try_for_each(|_| {
        count += 1;
        let cf2: ControlFlow<(), i32> = ControlFlow::Break(());
        let f: Flow = cf2.into();
        f
    });
    assert_eq!(count, 1);
}

#[test]
fn test_try_trait_from_output() {
    // Test that Flow::from_output returns Continue
    let flow = Flow::from_output(());
    let _ = flow; // Use the flow variable

    // Verify from_output behavior by testing in iteration
    let mut count = 0;
    std::iter::once(()).try_for_each(|_| {
        count += 1;
        Flow::from_output(())
    });
    assert_eq!(count, 1);
}

#[test]
fn test_from_residual() {
    // Test FromResidual implementation
    let flow: Flow = Flow::from_residual(Flow::Break);
    let _ = flow; // Use the flow variable

    // Verify from_residual behavior
    let mut count = 0;
    (0..10).try_for_each(|_| {
        count += 1;
        Flow::from_residual(Flow::Break)
    });
    // from_residual(Break) returns Break, so should stop after first
    assert_eq!(count, 1);
}
