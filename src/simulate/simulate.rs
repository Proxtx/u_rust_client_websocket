use enigo::Enigo;
use enigo::Key;
use enigo::KeyboardControllable;
use enigo::MouseControllable;

pub struct Simulate {
    enigo: Enigo,
}

impl Simulate {
    pub fn new() -> Self {
        Simulate {
            enigo: Enigo::new(),
        }
    }

    #[async_recursion::async_recursion]
    pub async fn keys(&mut self, sequence: &str) -> bool {
        let split = sequence.split("{");
        for section in split {
            if section.starts_with("+") || section.starts_with("-") {
                let mut section_split = section.split("}");
                let command = section_split.nth(0).unwrap();
                match command {
                    "+WAIT" => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(
                            u64::from_str_radix(section_split.nth(0).unwrap(), 10).unwrap(),
                        ))
                        .await;
                    }
                    "+MOUSE" => {
                        let mut coords_split = section_split.nth(0).unwrap().split(",");
                        self.enigo.mouse_move_to(
                            i32::from_str_radix(coords_split.nth(0).unwrap(), 10).unwrap(),
                            i32::from_str_radix(coords_split.nth(0).unwrap(), 10).unwrap(),
                        )
                    }
                    "-MOUSE" => {}
                    "-WAIT" => {}
                    "+RIGHTMOUSE" => self.enigo.mouse_down(enigo::MouseButton::Right),
                    "-RIGHTMOUSE" => self.enigo.mouse_up(enigo::MouseButton::Right),
                    "+LEFTMOUSE" => self.enigo.mouse_down(enigo::MouseButton::Left),
                    "-LEFTMOUSE" => self.enigo.mouse_up(enigo::MouseButton::Left),
                    "+SHIFT" => self.enigo.key_down(Key::Shift),
                    "-SHIFT" => self.enigo.key_up(Key::Shift),
                    "+CTRL" => self.enigo.key_down(Key::Control),
                    "-CTRL" => self.enigo.key_up(Key::Control),
                    "+META" => self.enigo.key_down(Key::Meta),
                    "-META" => self.enigo.key_up(Key::Meta),
                    "+ALT" => self.enigo.key_down(Key::Alt),
                    "-ALT" => self.enigo.key_up(Key::Alt),
                    "+TAB" => self.enigo.key_down(Key::Tab),
                    "-TAB" => self.enigo.key_up(Key::Tab),
                    "+BACKSPACE" => self.enigo.key_down(Key::Backspace),
                    "-BACKSPACE" => self.enigo.key_up(Key::Backspace),
                    "+CAPSLOCK" => self.enigo.key_down(Key::CapsLock),
                    "-CAPSLOCK" => self.enigo.key_up(Key::CapsLock),
                    "+CONTROL" => self.enigo.key_down(Key::Control),
                    "-CONTROL" => self.enigo.key_up(Key::Control),
                    "+DELETE" => self.enigo.key_down(Key::Delete),
                    "-DELETE" => self.enigo.key_up(Key::Delete),
                    "+DEL" => self.enigo.key_down(Key::Delete),
                    "-DEL" => self.enigo.key_up(Key::Delete),
                    "+DOWNARROW" => self.enigo.key_down(Key::DownArrow),
                    "-DOWNARROW" => self.enigo.key_up(Key::DownArrow),
                    "+END" => self.enigo.key_down(Key::End),
                    "-END" => self.enigo.key_up(Key::End),
                    "+ESCAPE" => self.enigo.key_down(Key::Escape),
                    "-ESCAPE" => self.enigo.key_up(Key::Escape),
                    "+F1" => self.enigo.key_down(Key::F1),
                    "-F1" => self.enigo.key_up(Key::F1),
                    "+F2" => self.enigo.key_down(Key::F2),
                    "-F2" => self.enigo.key_up(Key::F2),
                    "+F3" => self.enigo.key_down(Key::F3),
                    "-F3" => self.enigo.key_up(Key::F3),
                    "+F4" => self.enigo.key_down(Key::F4),
                    "-F4" => self.enigo.key_up(Key::F4),
                    "+F5" => self.enigo.key_down(Key::F5),
                    "-F5" => self.enigo.key_up(Key::F5),
                    "+F6" => self.enigo.key_down(Key::F6),
                    "-F6" => self.enigo.key_up(Key::F6),
                    "+F7" => self.enigo.key_down(Key::F7),
                    "-F7" => self.enigo.key_up(Key::F7),
                    "+F8" => self.enigo.key_down(Key::F8),
                    "-F8" => self.enigo.key_up(Key::F8),
                    "+F9" => self.enigo.key_down(Key::F9),
                    "-F9" => self.enigo.key_up(Key::F9),
                    "+F10" => self.enigo.key_down(Key::F10),
                    "-F10" => self.enigo.key_up(Key::F10),
                    "+F11" => self.enigo.key_down(Key::F11),
                    "-F11" => self.enigo.key_up(Key::F11),
                    "+F12" => self.enigo.key_down(Key::F12),
                    "-F12" => self.enigo.key_up(Key::F12),
                    "+HOME" => self.enigo.key_down(Key::Home),
                    "-HOME" => self.enigo.key_up(Key::Home),
                    "+LEFTARROW" => self.enigo.key_down(Key::LeftArrow),
                    "-LEFTARROW" => self.enigo.key_up(Key::LeftArrow),
                    "+OPTION" => self.enigo.key_down(Key::Option),
                    "-OPTION" => self.enigo.key_up(Key::Option),
                    "+PAGEDOWN" => self.enigo.key_down(Key::PageDown),
                    "-PAGEDOWN" => self.enigo.key_up(Key::PageDown),
                    "+PAGEUP" => self.enigo.key_down(Key::PageUp),
                    "-PAGEUP" => self.enigo.key_up(Key::PageUp),
                    "+RETURN" => self.enigo.key_down(Key::Return),
                    "-RETURN" => self.enigo.key_up(Key::Return),
                    "+RIGHTARROW" => self.enigo.key_down(Key::RightArrow),
                    "-RIGHTARROW" => self.enigo.key_up(Key::RightArrow),
                    "+UPARROW" => self.enigo.key_down(Key::UpArrow),
                    "-UPARROW" => self.enigo.key_up(Key::UpArrow),
                    _ => {
                        println!("Did not find {}", command)
                    }
                }
                match section_split.nth(0) {
                    Some(sequence) => self.keys(sequence).await,
                    None => true,
                };
            } else {
                let section_split = section.chars();
                for char in section_split {
                    self.enigo.key_click(Key::Layout(char));
                }
            }
        }

        return true;
    }
}
