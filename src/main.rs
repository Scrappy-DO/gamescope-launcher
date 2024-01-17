use std::process::{Command, exit};
use std::fmt;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow,
    Fixed, SpinButton, Adjustment, CheckButton, Label,
    Entry, EntryBuffer, Button};

fn main() {
    let app: Application = Application::builder()
        .application_id("com.scrappyDO.gamescope-launcher")
        .build();

    app.connect_activate(build_ui);
    app.run();
}


fn build_ui(app: &Application){
    // title
    let res_title_text: Option<&str> = Some("Set resolution");
    let res_title: Label = Label::new(res_title_text);

    // resolution
    let res_x: SpinButton = new_spin_button(0.0, 9999.0, 1920.0, 
        1.0, 0, 120, 30);
    let res_y: SpinButton = new_spin_button(0.0, 9999.0, 1080.0, 
        1.0, 0, 120, 30);

    let res_text_text: Option<&str> = Some("X");
    let res_text: Label = Label::new(res_text_text);

    // refresh rate
    let refresh_enable: CheckButton = new_checkbox(false, "refresh rate: ");
    let refresh_spin: SpinButton = new_spin_button(24.0, 999.0, 60.0,
         1.0, 0, 110, 30);

    // other checkbox
    let select_fulscreen: CheckButton = new_checkbox(true, "fulscreen");
    let select_steam_intigration: CheckButton = new_checkbox(true, "Steam intigration");
    
    // fsr
    let fsr_checkbox: CheckButton = new_checkbox(false, "fsr");
    let fsr_strengt: SpinButton = new_spin_button(0.0, 20.0, 5.0, 
        1.0, 0, 100, 30);
    
    let fsr_render_resolution_text: Option<&str> = Some("render resolution (fsr only):");
    let fsr_render_resolution: Label = Label::new(fsr_render_resolution_text);

    let fsr_render_text_text: Option<&str> = Some("X");
    let fsr_render_text: Label = Label::new(fsr_render_text_text);

    let fsr_render_x: SpinButton = new_spin_button(0.0, 9999.0, 1477.0,
         1.0, 0, 120, 30);
    let fsr_render_y: SpinButton = new_spin_button(0.0, 9999.0, 831.0,
        1.0, 0, 120, 30);

    // exec
    let exec_title_text: Option<&str> = Some("command or path to executable");
    let exec_title: Label = Label::new(exec_title_text);

    let exec_entry: Entry = new_entry("steam -gamepadui", 320, 30);

    // launch button
    let launch_button: Button = new_button("launch gamescope");

    // arguments
    let select_steam_intigration_copy = &select_steam_intigration;
    let select_fulscreen_copy = &select_fulscreen;
    let fsr_checkbox_copy = &fsr_checkbox;

    let fsr_strengt_copy = &fsr_strengt;

    let fsr_render_x_copy = &fsr_render_x;
    let fsr_render_y_copy = &fsr_render_y;

    let res_x_copy = &res_x;
    let res_y_copy = &res_y;

    let refresh_enable_copy = &refresh_enable;
    let refresh_spin_copy = &refresh_spin;

    let exec_entry_copy = &exec_entry;

    let fulscreen = select_fulscreen_copy.is_active();
    let steam_inigration = select_steam_intigration_copy.is_active();

    let fsr = fsr_checkbox_copy.is_active();
    let fsr_strengt_value = fsr_strengt_copy.value_as_int();
    let fsr_res_x = fsr_render_x_copy.value_as_int();
    let fsr_res_y = fsr_render_y_copy.value_as_int();

    let res_x_value = res_x_copy.value_as_int();
    let res_y_value = res_y_copy.value_as_int();

    let refresh = refresh_enable_copy.is_active();
    let refresh_rate = refresh_spin_copy.value_as_int();

    let command = exec_entry_copy.text();

    // arguments cast 2
    let fsr_strengt_value = fsr_strengt_value.to_string();
    let fsr_res_x = fsr_res_x.to_string();
    let fsr_res_y = fsr_res_y.to_string();

    let res_x_value = res_x_value.to_string();
    let res_y_value = res_y_value.to_string();

    let refresh_rate = refresh_rate.to_string();

    let command = command.as_str().to_owned();
    

     
    launch_button.connect_clicked(move |_|{
        let mut launch_args = vec![];
        
        if steam_inigration{
            launch_args.push("-e");
        }

        if fulscreen {
            launch_args.push("-f");
        }

        if fsr {
            launch_args.push("-F");
            launch_args.push("fsr");

            launch_args.push("--fsr-sharpness");
            launch_args.push(&fsr_strengt_value);

            launch_args.push("-w");
            launch_args.push(&fsr_res_x);

            launch_args.push("-h");
            launch_args.push(&fsr_res_y);

            launch_args.push("-W");
            launch_args.push(&res_x_value);

            launch_args.push("-H");
            launch_args.push(&res_y_value);
            
        } else {
            launch_args.push("-w");
            launch_args.push(&res_x_value);

            launch_args.push("-h");
            launch_args.push(&res_y_value);        
        }

        if refresh {
            launch_args.push("-r");
            launch_args.push(&refresh_rate);
        }

        launch_args.push("--");
        for flag in command.split_whitespace() {
            launch_args.push(flag)
        }


        for arg in launch_args.clone() {
            print!(" {}", arg);
        }

        print!("\n");
        let _ = Command::new("gamescope")
            .args(launch_args)
            .spawn();
        
        exit(0);
     });

    // final window
    let fixed: Fixed = Fixed::new();
    fixed.put(&res_title, 30.0, 30.0);
    fixed.put(&res_x, 65.0, 60.0);
    fixed.put(&res_text, 196.0, 67.0);
    fixed.put(&res_y, 215.0, 60.0);

    fixed.put(&refresh_enable, 30.0, 114.0);
    fixed.put(&refresh_spin, 150.0, 110.0);
    
    fixed.put(&select_fulscreen, 30.0, 143.0);
    fixed.put(&select_steam_intigration, 30.0, 170.0);

    fixed.put(&fsr_checkbox, 30.0, 198.0);
    fixed.put(&fsr_strengt, 100.0, 195.0);
    fixed.put(&fsr_render_resolution, 30.0, 240.0);
    fixed.put(&fsr_render_x, 65.0, 265.0);
    fixed.put(&fsr_render_text, 196.0, 272.0);
    fixed.put(&fsr_render_y, 215.0, 265.0);

    fixed.put(&exec_title, 30.0, 310.0);
    fixed.put(&exec_entry, 40.0, 335.0);

    fixed.put(&launch_button, 110.0, 410.0);
    

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
