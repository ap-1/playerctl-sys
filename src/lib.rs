#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("types.rs");
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};

    use glib_sys::{
        g_error_free, g_list_find_custom, g_list_length, g_list_nth_data, gconstpointer,
    };

    use super::*;

    fn to_mut_c_string(s: &str) -> *mut gchar {
        CString::new(s).unwrap().into_raw()
    }

    #[test]
    fn list_players() {
        unsafe {
            // Get the list of all players that are currently running
            let mut error: *mut GError = std::ptr::null_mut();
            let players = playerctl_list_players(&mut error);

            if players.is_null() {
                println!("No players found");
            } else {
                let length = g_list_length(players);
                println!("Found {} player(s)", length);

                for n in 0..length {
                    // Get the name from the nth player in the list
                    let player = g_list_nth_data(players, n) as *mut PlayerctlPlayerName;
                    let name = CString::from_raw((*player).name);

                    // Output the player number and its name
                    println!("Player {}: {}", n, name.to_str().unwrap());
                }
            }

            if !error.is_null() {
                g_error_free(error);
            }
        }
    }

    #[test]
    fn play_pause_spotify() {
        unsafe {
            // Get the list of all players that are currently running
            let mut error: *mut GError = std::ptr::null_mut();
            let players = playerctl_list_players(&mut error);

            if players.is_null() {
                println!("No players are running");
            } else {
                let name_cstr = to_mut_c_string("spotify");
                let instance_cstr = to_mut_c_string("spotify");

                // Construct a PlayerctlPlayerName for the Spotify player
                let player_name = PlayerctlPlayerName {
                    name: name_cstr,
                    instance: instance_cstr,
                    source: PlayerctlSource_PLAYERCTL_SOURCE_DBUS_SESSION,
                };

                // Create a function that compares items in the *mut GList
                unsafe extern "C" fn compare_func(a: gconstpointer, b: gconstpointer) -> gint {
                    let a = (*(a as *const PlayerctlPlayerName)).name;
                    let b = (*(b as *const PlayerctlPlayerName)).name;

                    // Return 0 if the two strings are the same
                    CStr::from_ptr(a).cmp(CStr::from_ptr(b)) as gint
                }

                // Convert the player name to a *const c_void (gconstpointer) and search for it
                let data: gconstpointer = &player_name as *const _ as gconstpointer;
                let element = g_list_find_custom(players, data, Some(compare_func));

                if element.is_null() {
                    println!("Spotify is not running");
                } else {
                    // Free the strings by converting them back to CStrings
                    drop(CString::from_raw(name_cstr));
                    drop(CString::from_raw(instance_cstr));

                    let player_name = (*element).data as *mut PlayerctlPlayerName;

                    let mut player_error: *mut GError = std::ptr::null_mut();
                    let player = playerctl_player_new_from_name(player_name, &mut player_error);

                    if player.is_null() {
                        println!("Could not create player");
                    } else {
                        let mut play_pause_error: *mut GError = std::ptr::null_mut();
                        playerctl_player_play_pause(player, &mut play_pause_error);

                        if !play_pause_error.is_null() {
                            g_error_free(play_pause_error);
                        } else {
                            println!("Toggled spotify playing status");
                        }
                    }

                    if !player_error.is_null() {
                        g_error_free(player_error);
                    }
                }
            }

            if !error.is_null() {
                g_error_free(error);
            }
        }
    }
}
