// Currently needed because we use these functionality, they'll be removable when the Rust language stabilizes them
#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use dialog_config::talk_settings_callback;
use dialog_config::SWITCH_PRESSED;
use utils::get_config;

use engage::{
pad::Pad, util::get_instance,
};
use engage::proc::ProcInstFields;
pub mod utils;
pub mod dialog_config;
//Structs with the same fields as TalkUI and MyRoomRelianceSequence
pub struct MyRoomRelianceSequence {
    pub super_fields: ProcInstFields,
    pub maincontent: *mut u8,  
    pub root: *mut u8,         
    pub content: *mut u8,        
    pub subcontent: *mut u8,     
}
static mut CONVO: bool = false;
static mut MINUS_PRESSED: bool = false;
pub struct TalkUI {
    pub super_fields: ProcInstFields,
    pub m_system_objects: *mut u8,
    pub m_stand_objects: *mut u8,
    pub m_face_objects: *mut u8,
    pub m_focus_talk_objects: *mut u8,
    pub m_reserve_focus_window: *mut u8,
    pub m_event_picture_controller: *mut u8, 
}
//Function that closes dialog box 
#[skyline::from_offset(0x21dd070)]
pub fn talk_ui_hide(this : &TalkUI,  method_info: OptionalMethod);

//Function that opens dialog box
#[skyline::from_offset(0x21dd000)]
pub fn talk_ui_show(this : &TalkUI,  method_info: OptionalMethod);


#[skyline::hook(offset = 0x2396920)]
pub fn myroomreliancesequence_entry(event_name: &MyRoomRelianceSequence, method_info: OptionalMethod,) {
    unsafe {CONVO = true; }
    call_original!(event_name, method_info);
}
#[skyline::hook(offset = 0x2396d40)]
pub fn myroomreliancesequence_exit(event_name: &MyRoomRelianceSequence, method_info: OptionalMethod,) {
    unsafe { CONVO = false;}
    call_original!(event_name, method_info);
}

#[skyline::hook(offset = 0x21dcfe0)]
pub fn talkui_update(this: &TalkUI, method_info: OptionalMethod) { 
    //If the support convo hide dialogue by default button was pressed
    if unsafe { SWITCH_PRESSED == true } {
        //If a support conversation is happening
        if unsafe { CONVO == true } {
            //Hide the dialog box
            open_closer( unsafe {CONVO}, this, method_info);
            //To keep it from always hiding it every tick, we need to change the value of CONVO
            unsafe { CONVO = false };
        }
    }
    let pad_instance = get_instance::<Pad>();
    if pad_instance.npad_state.buttons.minus() {
        if !pad_instance.old_buttons.minus() {
            unsafe {
                //If the minus button was pressed, switch MINUS_PRESSED from false to true
                MINUS_PRESSED = !MINUS_PRESSED;
                //Function that will decide whether or not to open or close dialog window based on bool value of MINUS_PRESSED
                //True will close the box, False will open it again
                open_closer(MINUS_PRESSED, this, method_info);
            }
        }
    }
    call_original!(this, method_info);
}

//Function that will decide whether or not to open or close dialog window based on bool value of MINUS_PRESSED
pub fn open_closer(buttonpressed : bool, this : &TalkUI, method_info: OptionalMethod) {
    //Checks if the minus button was pressed 
    //If it was pressed (which will make the value True), hide the dialog box
        match { buttonpressed } {
        true => {
            unsafe { talk_ui_hide(this, method_info) }
        }
        false => {
            unsafe { talk_ui_show(this, method_info) }
        }
    }
}
/// The internal name of your plugin. This will show up in crash logs. Make it 8 characters long at max.
#[skyline::main(name = "dialoghs")]
pub fn main() {
    // Install a panic handler for your plugin, allowing you to customize what to do if there's an issue in your code.
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    cobapi::install_game_setting(talk_settings_callback);
    skyline::install_hooks!(talkui_update, myroomreliancesequence_entry, myroomreliancesequence_exit);
}