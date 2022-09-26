use chrono::{Datelike, Timelike};
use device_query::{DeviceState, DeviceEvents, DeviceQuery, MouseState, Keycode};

fn get_current_time() -> String {
    let time = chrono::offset::Local::now();
    let year = time.year();
    let month = time.month();
    let day = time.day();
    let hour = time.hour().to_string();
    let minute = time.minute().to_string();
    let second = time.second().to_string();

    format!("Date - {}/{}/{} Time - {}:{}:{}", year, month, day, hour, minute, second)
}

fn get_key(key_list: &mut Vec<Keycode>) -> Option<Keycode>{
    let device = DeviceState::new();
    let key = device.get_keys().into_iter().filter(|item| !key_list.contains(item)).collect::<Vec<Keycode>>().pop();
    *key_list = device.get_keys();
    key
}

fn get_mouse_pos() -> String {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();
    let mouse_coordinates = mouse.coords;

    format!("X:{:?} Y:{:?}", mouse_coordinates.0, mouse_coordinates.1)
}

fn main() {
    let mut key_list = Vec::new();
    let device_state = DeviceState::new();

    loop {
        let _guard = device_state.on_mouse_down(|button| {
            println!("Mouse button {} pressed on {}\n", button, get_mouse_pos())
         });


        if let Some(key) = get_key(&mut key_list){
            println!(
                "{}\n Key pressed: {}\n",
                get_current_time(),
                key
            );
        }
    }
}