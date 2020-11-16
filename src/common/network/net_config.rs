use rua_value_list::Config;
use rua_utils::FileUtils;

pub struct NetConfig;

static mut el: *mut Config = 0 as *mut _;

impl NetConfig {
    pub fn instance() -> &'static mut Config {
        unsafe {
            if el == 0 as *mut _ {
                el = Box::into_raw(Box::new(Config::new_empty()))
            }

            &mut *el
        }
    }

    pub fn change_instance(field: &str, proto: &str) -> bool {
        let config = match Config::new(field, proto) {
            Some(c) => c,
            _ => {
                return false;
            }
        };

        unsafe {
            if el != 0 as *mut _ {
                let old = Box::from_raw(el);
                drop(old)
            }
            el = Box::into_raw(Box::new(config));
        }
        true
    }

    pub fn change_by_file(file_name: &str) -> bool {
        if let Ok(file_data) = FileUtils::get_file_data(file_name) {

            let file_data = match String::from_utf8(file_data) {
                Ok(f) => f,
                _ => {
                    return false;
                }
            };

            let config = match Config::new_by_full_str(&*file_data) {
                Some(c) => c,
                _ => {
                    return false;
                }
            };

            unsafe {
                if el != 0 as *mut _ {
                    let old = Box::from_raw(el);
                    drop(old);
                }
                el = Box::into_raw(Box::new(config));
            }
            return true;
        }
        false
    }
}