extern crate pancurses;
use pancurses::Input;

pub fn do_the_thing() {
    let window = pancurses::initscr();
    window.printw("Press enter to select an item.\n");
    window.refresh();
    window.keypad(true);
    pancurses::noecho();

    if pancurses::has_colors() {
        pancurses::start_color();
    }

    let choices = ["milk", "bread", "cookies"];

    let mut selected_idx = 0;

    loop {
        for (i, s) in choices.iter().enumerate() {
            let i = i as i32;
            if i == selected_idx {
                window.attron(pancurses::A_REVERSE);
            }

            window.mvprintw(i + 1, 0, s);
            window.attroff(pancurses::A_REVERSE);
        }

        match window.getch() {
            Some(Input::KeyDown) => {
                selected_idx += 1;
                if selected_idx as usize > choices.len() {
                    selected_idx = choices.len() as i32 - 1;
                }

                ()
            }
            Some(Input::KeyUp) => {
                selected_idx -= 1;
                if selected_idx < 0 {
                    selected_idx = 0;
                }

                ()
            }
            Some(Input::Character('\n')) => break,
            _ => (),
        }
    }

    window.mvprintw(4, 0, &format!("Your choice was: {} (press any key to exit)", choices[selected_idx as usize]));
    window.getch();

    pancurses::endwin();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
