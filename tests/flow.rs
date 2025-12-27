use platform_data::Flow;
use std::ops::ControlFlow;

#[test]
fn basic() {
    let mut vec = vec![];

    for i in 0..20 {
        vec.push(i);
        let flow = if i == 10 { Flow::Break } else { Flow::Continue };
        if flow.is_break() {
            break;
        }
    }

    assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_flow_continue_with_control_flow() {
    let mut count = 0;
    let _ = (0..5).try_for_each(|_| {
        count += 1;
        Flow::Continue.into_control_flow()
    });
    assert_eq!(count, 5);
}

#[test]
fn test_flow_break_early_with_control_flow() {
    let mut count = 0;
    let _ = (0..100).try_for_each(|_| {
        count += 1;
        Flow::Break.into_control_flow()
    });
    assert_eq!(count, 1); // Should stop after first iteration
}

#[test]
fn test_from_control_flow_continue() {
    // Test conversion from ControlFlow::Continue
    let cf: ControlFlow<i32, ()> = ControlFlow::Continue(());
    let flow: Flow = cf.into();
    assert!(flow.is_continue());
}

#[test]
fn test_from_control_flow_break() {
    // Test conversion from ControlFlow::Break
    let cf: ControlFlow<(), i32> = ControlFlow::Break(());
    let flow: Flow = cf.into();
    assert!(flow.is_break());

    // Verify Break behavior
    let mut count = 0;
    let _ = (0..10).try_for_each(|_| {
        count += 1;
        Flow::Break.into_control_flow()
    });
    assert_eq!(count, 1);
}

#[test]
fn from_control_flow() {
    let continue_flow: Flow = ControlFlow::<(), ()>::Continue(()).into();
    assert!(continue_flow.is_continue());

    let break_flow: Flow = ControlFlow::<(), ()>::Break(()).into();
    assert!(break_flow.is_break());
}

#[test]
fn into_control_flow() {
    let cf: ControlFlow<()> = Flow::Continue.into();
    assert!(matches!(cf, ControlFlow::Continue(())));

    let cf: ControlFlow<()> = Flow::Break.into();
    assert!(matches!(cf, ControlFlow::Break(())));
}

#[test]
fn test_into_control_flow_method() {
    // Test the into_control_flow() method
    let continue_cf = Flow::Continue.into_control_flow();
    assert!(matches!(continue_cf, ControlFlow::Continue(())));

    let break_cf = Flow::Break.into_control_flow();
    assert!(matches!(break_cf, ControlFlow::Break(())));
}

#[test]
fn test_flow_with_iterator() {
    // Test using Flow with iterators via into_control_flow
    let mut collected = vec![];
    let _ = (0..20).try_for_each(|i| {
        collected.push(i);
        if i == 10 {
            Flow::Break.into_control_flow()
        } else {
            Flow::Continue.into_control_flow()
        }
    });
    assert_eq!(collected, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_is_continue() {
    assert!(Flow::Continue.is_continue());
    assert!(!Flow::Break.is_continue());
}

#[test]
fn test_is_break() {
    assert!(Flow::Break.is_break());
    assert!(!Flow::Continue.is_break());
}

#[test]
fn test_flow_copy() {
    let flow = Flow::Continue;
    let copied1 = flow;
    let copied2 = flow;
    assert_eq!(flow, copied1);
    assert_eq!(flow, copied2);
}

#[test]
fn test_flow_eq() {
    assert_eq!(Flow::Continue, Flow::Continue);
    assert_eq!(Flow::Break, Flow::Break);
    assert_ne!(Flow::Continue, Flow::Break);
}

#[test]
fn test_flow_debug() {
    assert_eq!(format!("{:?}", Flow::Continue), "Continue");
    assert_eq!(format!("{:?}", Flow::Break), "Break");
}
