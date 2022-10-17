pub struct Screen {}

impl Screen {
    pub fn new() -> Self {
        Screen {}
    }

    pub fn screenshot(&self, screen_index: usize) -> Option<Vec<u8>> {
        let screens = screenshots::Screen::all().unwrap();

        if screens.len() <= screen_index {
            return Option::None;
        }

        let mut t = 0;
        let mut screen_option: Option<screenshots::Screen> = Option::None;
        for local_screen in screens {
            if t == screen_index {
                screen_option = Option::Some(local_screen);
                break;
            }

            t += 1;
        }

        let screen;

        match screen_option {
            Some(local_screen) => screen = local_screen,
            None => return Option::None,
        }

        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        return Option::Some(buffer.to_owned());
    }

    pub fn screens(&self) -> Vec<String> {
        let screens = screenshots::Screen::all().unwrap();
        let mut res: Vec<String> = Vec::new();
        for screen in screens {
            res.push(format!("{:?}", screen.display_info));
        }
        return res;
    }
}
