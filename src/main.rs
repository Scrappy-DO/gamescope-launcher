use std::process::{Command, exit};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow,
    Fixed, SpinButton, Adjustment, CheckButton, Label,
    Entry, EntryBuffer, Button};

#[derive(Clone)]
struct MyApp{
    // title
    res_title: Label,

    //resolution
    res_x: SpinButton,
    res_y: SpinButton,

    res_text: Label,

    //refresh rate
    refresh_enable: CheckButton,
    refresh_spin: SpinButton,

    // other
    select_fullscreen: CheckButton,
    select_steam_intigration: CheckButton,
    
    //fsr
    fsr_checkbox: CheckButton,
    fsr_strengt: SpinButton,
    fsr_render_res_text: Label,
    fsr_render_text: Label,
    fsr_render_x: SpinButton,
    fsr_render_y: SpinButton,

    // exec
    exec_title: Label,
    exec_entry: Entry,

    launch_button: Button
}

impl MyApp {
    fn new(
        // title
        res_title: Label,

        //resolution
        res_x: SpinButton,
        res_y: SpinButton,

        res_text: Label,

        //refresh rate
        refresh_enable: CheckButton,
        refresh_spin: SpinButton,

        // other
        select_fullscreen: CheckButton,
        select_steam_intigration: CheckButton,
        
        //fsr
        fsr_checkbox: CheckButton,
        fsr_strengt: SpinButton,
        fsr_render_res_text: Label,
        fsr_render_text: Label,
        fsr_render_x: SpinButton,
        fsr_render_y: SpinButton,

        // exec
        exec_title: Label,
        exec_entry: Entry,

        launch_button: Button
    ) -> MyApp {
        return MyApp{
            res_title,
            res_x,
            res_y,
            res_text,
            refresh_enable,
            refresh_spin,
            select_fullscreen,
            select_steam_intigration,
            fsr_checkbox,
            fsr_strengt,
            fsr_render_res_text,
            fsr_render_text,
            fsr_render_x,
            fsr_render_y,
            exec_title,
            exec_entry,
            launch_button
        };
    }

    fn launch_cb(&self, _: &Button){

        let fsr_strengt = self.fsr_strengt.value_as_int().to_string();
        let fsr_x = self.fsr_render_x.value_as_int().to_string();
        let fsr_y = self.fsr_render_y.value_as_int().to_string();
        let res_x = self.res_x.value_as_int().to_string();
        let res_y = self.res_y.value_as_int().to_string();
        let refresh_rate = self.refresh_spin.value_as_int().to_string();
        let command = self.exec_entry.text().to_string();


        let mut launch_args = vec![];

        if self.select_steam_intigration.is_active(){
            launch_args.push("-e");
        }

        if self.select_fullscreen.is_active() {
            launch_args.push("-f");
        }

        if self.fsr_checkbox.is_active() {
            launch_args.push("-F");
            launch_args.push("fsr");

            launch_args.push("--fsr-sharpness");
            launch_args.push(&fsr_strengt);

            launch_args.push("-w");
            launch_args.push(&fsr_x);

            launch_args.push("-h");
            launch_args.push(&fsr_y);

            launch_args.push("-W");
            launch_args.push(&res_x);

            launch_args.push("-H");
            launch_args.push(&res_y);
            
        } else {
            launch_args.push("-w");
            launch_args.push(&res_x);

            launch_args.push("-h");
            launch_args.push(&res_y);
        }

        if self.refresh_enable.is_active() {
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
        println!("");

        
        let _ = Command::new("gamescope")
            .args(launch_args)
            .spawn();
    
        exit(0);
    }
}

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
