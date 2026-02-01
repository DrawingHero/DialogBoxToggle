use unity::prelude::*;

use engage::menu::{
    BasicMenuResult,
    config::{
        ConfigBasicMenuItem,
        ConfigBasicMenuItemSwitchMethods
    }
};
use crate::{
    utils::{localize, off_str, on_str, save_config}
};
pub static mut SWITCH_PRESSED: bool = false;
//Making a struct to impl for ConfigBasicMenuItemSwitchMethods
pub struct TalkSetting;

// pub trait ConfigBasicMenuItemSwitchMethods {
//     fn init_content(_this: &mut ConfigBasicMenuItem) {}
//     extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult;
//     extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
//     extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
//     extern "C" fn a_call(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult {
//         BasicMenuResult::new()
//     }
//     extern "C" fn build_attributes(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuItemAttribute {
//         BasicMenuItemAttribute::Enable
//     }
// }

impl ConfigBasicMenuItemSwitchMethods for TalkSetting {
    extern "C" fn custom_call(
        this: &mut ConfigBasicMenuItem,
        _method_info: OptionalMethod,
    ) -> BasicMenuResult {
        let switchpressed: bool = unsafe { SWITCH_PRESSED };

        let result = ConfigBasicMenuItem::change_key_value_b(switchpressed);

        if switchpressed != result {
            unsafe { SWITCH_PRESSED = result };
            save_config("talk", result);
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new() 
        }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        match unsafe { SWITCH_PRESSED } {
            true => this.command_text = on_str(),
            false => this.command_text = off_str(),
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        match unsafe { SWITCH_PRESSED } {
            true => this.help_text = localize("button_on").into(),
            false => this.help_text = localize("button_off").into(),
        }
        }
    }
#[no_mangle] // no_mangle is an attribute used to ask Rust not to modify your function name to facilitate communication with code from other sources.
pub extern "C" fn talk_settings_callback() -> &'static mut ConfigBasicMenuItem {
    // Your callback must return a ConfigBasicMenu, which you can acquire by using new_gauge or new_switch.
    ConfigBasicMenuItem::new_switch::<TalkSetting>(localize("talk_name"))
}