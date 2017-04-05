extern crate pancurses;
use pancurses::Input;

pub fn do_the_thing() {
    let window = pancurses::initscr();
    window.printw("Press enter to select an item or 'q' to quit.\n");

    if pancurses::has_colors() {
        pancurses::start_color();
    }

    pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
    window.bkgd(pancurses::ColorPair(1));

    pancurses::init_pair(2, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    let attr_selected = pancurses::ColorPair(2);

    window.refresh();
    window.keypad(true);
    pancurses::noecho();

    let max_y = window.get_max_y();

    let choices = [
        "milk",
        "bread",
        "cookies",
    ];

    let choices_strings = choices.iter().map(|s| format!("{:<15}", s)).collect::<Vec<_>>();

    let mut selected_idx = 0;

    let mut explicit_quit = false;

    loop {
        for (i, s) in choices_strings.iter().enumerate() {
            let i = i as i32;
            if i == selected_idx {
                window.attron(attr_selected);
            }

            window.mvprintw(i + 1, 0, &s);
            window.attroff(attr_selected);
        }

        window.mvprintw(max_y - 1, 0, "[todo: statusbar here]");

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
            Some(Input::Character('q')) => {
                explicit_quit = true;
                break
            },
            _ => (),
        }

        window.refresh();
    }

    if !explicit_quit {
        window.mvprintw(4, 0, &format!("Your choice was: {} (press any key to exit)", choices[selected_idx as usize]));
        window.getch();
    }

    pancurses::endwin();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
