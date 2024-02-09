use std::process::{Command, exit};
use gtk::prelude::*;
use gtk::{
    SpinButton, CheckButton, Label,
    Entry, Button};

#[derive(Clone)]
pub struct MyApp{
     // title
    pub res_title: Label,
 
     //resolution
    pub res_x: SpinButton,
    pub res_y: SpinButton,
 
    pub res_text: Label,
 
     //refresh rate
    pub refresh_enable: CheckButton,
    pub refresh_spin: SpinButton,
 
     // other
    pub select_fullscreen: CheckButton,
    pub select_steam_intigration: CheckButton,
     
     //fsr
    pub fsr_checkbox: CheckButton,
    pub fsr_strengt: SpinButton,
    pub fsr_render_res_text: Label,
    pub fsr_render_text: Label,
    pub fsr_render_x: SpinButton,
    pub fsr_render_y: SpinButton,
 
     // exec
    pub exec_title: Label,
    pub exec_entry: Entry,
 
    pub launch_button: Button
}

impl MyApp {
    pub fn new(
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

    pub fn launch_cb(&self, _: &Button){

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
