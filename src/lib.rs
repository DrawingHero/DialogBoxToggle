// Currently needed because we use these functionality, they'll be removable when the Rust language stabilizes them
#![feature(lazy_cell, ptr_sub_ptr)]

use engage::{
pad::Pad, util::get_instance,
};
static mut MINUS_PRESSED: bool = false;

use engage::proc::ProcInstFields;

//Making a struct with the same fields as TalkUI

pub struct TalkUI {
    pub super_fields: ProcInstFields,
    pub m_system_objects: *mut u8,
    pub m_stand_objects: *mut u8,
    pub m_face_objects: *mut u8,
    pub m_focus_talk_objects: *mut u8,
    pub m_reserve_focus_window: *mut u8,
    pub m_event_picture_controller: *mut u8, 
}
/// This is called a proc(edural) macro. You use this to indicate that a function will be used as a hook.
///
/// Pay attention to the argument, offset.
/// This is the address of the start of the function you would like to hook.
/// This address has to be relative to the .text section of the game.
/// If you do not know what any of this means, take the address in Ghidra and remove the starting ``71`` and the zeroes that follow it.
/// Do not forget the 0x indicator, as it denotates that you are providing a hexadecimal value.

//Function that closes dialog box 
#[skyline::from_offset(0x21dd070)]
pub fn talk_ui_hide(this : &TalkUI,  method_info: OptionalMethod);

//Function that opens dialog box
#[skyline::from_offset(0x21dd000)]
pub fn talk_ui_show(this : &TalkUI,  method_info: OptionalMethod);

#[skyline::hook(offset = 0x21dcfe0)]
pub fn talkui_update(this: &TalkUI, method_info: OptionalMethod) { 
    // This tick is for TalkSequence, which is every frame the dialog box is displayed
    // Since the function is run every frame, it'll always be available to update a bool value that can be used to turn the 
    // dialog box off and on again.
    println!("Tick is working!");
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
    skyline::install_hooks!(talkui_update);
}