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
