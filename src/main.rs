use gtk::prelude::*;
use gtk::{Application, ApplicationWindow,
    Fixed, SpinButton, Adjustment, CheckButton, Label,
    Entry, EntryBuffer, Button};

use gamescope_launcher::MyApp;

fn main() {
    let app: Application = Application::builder()
        .application_id("com.scrappyDO.gamescope-launcher")
        .build();

    app.connect_activate(build_ui);
    app.run();
}


fn build_ui(app: &Application){

    let my_app: MyApp = MyApp::new(
        Label::new(Some("Set Resolution")), 
        new_spin_button(0.0, 9999.0, 1920.0, 1.0, 0, 120, 30), 
        new_spin_button(0.0, 9999.0, 1080.0, 1.0, 0, 120, 30), 
        Label::new(Some("X")), 
        new_checkbox(false, "refresh rate: "), 
        new_spin_button(24.0, 999.0, 60.0, 1.0, 0, 110, 30), 
        new_checkbox(true, "fulscreen"), 
        new_checkbox(true, "Steam intigration"), 
        new_checkbox(false, "fsr"), 
        new_spin_button(0.0, 20.0, 5.0, 1.0, 0, 100, 30), 
        Label::new(Some("render resolution (fsr only):")), 
        Label::new(Some("X")), 
        new_spin_button(0.0, 9999.0, 1477.0, 1.0, 0, 120, 30), 
        new_spin_button(0.0, 9999.0, 831.0, 1.0, 0, 120, 30), 
        Label::new(Some("command or path to executable")), 
        new_entry("steam -gamepadui", 320, 30),
        new_button("launch gamescope") 
    );


    let my_app_ref = my_app.clone();
    my_app.launch_button.connect_clicked(move |button|{
        my_app_ref.launch_cb(button);
    });

    // final window
    let fixed: Fixed = Fixed::new();
    fixed.put(&my_app.res_title, 30.0, 30.0);
    fixed.put(&my_app.res_x, 65.0, 60.0);
    fixed.put(&my_app.res_text, 196.0, 67.0);
    fixed.put(&my_app.res_y, 215.0, 60.0);

    fixed.put(&my_app.refresh_enable, 30.0, 114.0);
    fixed.put(&my_app.refresh_spin, 150.0, 110.0);
    
    fixed.put(&my_app.select_fullscreen, 30.0, 143.0);
    fixed.put(&my_app.select_steam_intigration, 30.0, 170.0);

    fixed.put(&my_app.fsr_checkbox, 30.0, 198.0);
    fixed.put(&my_app.fsr_strengt, 100.0, 195.0);
    fixed.put(&my_app.fsr_render_res_text, 30.0, 240.0);
    fixed.put(&my_app.fsr_render_x, 65.0, 265.0);
    fixed.put(&my_app.fsr_render_text, 196.0, 272.0);
    fixed.put(&my_app.fsr_render_y, 215.0, 265.0);

    fixed.put(&my_app.exec_title, 30.0, 310.0);
    fixed.put(&my_app.exec_entry, 40.0, 335.0);

    fixed.put(&my_app.launch_button, 110.0, 410.0);
    

    let window: ApplicationWindow = ApplicationWindow::builder()
        .title("gamescope launcher")
        .application(app)
        .default_width(400)
        .default_height(550)
        .resizable(false)
        .child(&fixed)
        .build();

    window.show();

}


fn new_button(label: &str) -> Button{
    let button: Button = Button::builder()
        .label(label)
        .build();


    return button;
}

fn new_entry(text: &str, width: i32, height: i32) -> Entry{
    let buffer: EntryBuffer = EntryBuffer::builder()
        .text(text)
        .build();

    let input: Entry = Entry::builder()
        .buffer(&buffer)
        .editable(true)
        .width_request(width)
        .height_request(height)
        .build();

    return input;
}

fn new_checkbox(is_active: bool, label: &str) -> CheckButton{
    let button: CheckButton = CheckButton::builder()
        .active(is_active)
        .label(label)
        .build();

    return button;
}

fn new_spin_button(lower: f64, upper: f64, value: f64, climb_rate: f64, digits: u32, width: i32, height: i32) -> SpinButton{
    let adj: Adjustment = Adjustment::builder()
        .lower(lower)
        .upper(upper)
        .value(value)
        .page_increment(1.0)
        .step_increment(climb_rate)
        .page_size(0.0)
        .build();

    let button: SpinButton = SpinButton::builder()
        .adjustment(&adj)
        .climb_rate(climb_rate)
        .digits(digits)
        .width_request(width)
        .height_request(height)
        .build();

    return button;
}
