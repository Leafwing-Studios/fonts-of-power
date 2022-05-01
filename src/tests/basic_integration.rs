#[test]
/// Verifies that the app runs without setup-time crashes
fn basic_headless() {
    use crate::headless_app;

    let mut app = headless_app();

    for _ in 1..3 {
        app.update();
    }
}
