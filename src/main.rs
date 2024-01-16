slint::include_modules!();




fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
// ui.on_divide_money();


    ui.run()
}
