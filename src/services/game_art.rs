use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::text::Text;
use crate::enums::field::FieldMark;

const YOUR_TURN: [&str; 5] = [
    "__   __                 _                    ",
    "\\ \\ / /__  _   _ _ __  | |_ _   _ _ __ _ __  ",
    " \\ V / _ \\| | | | '__| | __| | | | '__| '_ \\ ",
    "  | | (_) | |_| | |    | |_| |_| | |  | | | |",
    "  |_|\\___/ \\__,_|_|     \\__|\\__,_|_|  |_| |_|",
];

pub fn your_turn() -> Text<'static>{
    Text::from_iter(YOUR_TURN)
}

const ENEMY_TURN: [&str; 6] = [
    " _____                              _                    ",
    "| ____|_ __   ___ _ __ ___  _   _  | |_ _   _ _ __ _ __  ",
    "|  _| | '_ \\ / _ \\ '_ ` _ \\| | | | | __| | | | '__| '_ \\ ",
    "| |___| | | |  __/ | | | | | |_| | | |_| |_| | |  | | | |",
    "|_____|_| |_|\\___|_| |_| |_|\\__, |  \\__|\\__,_|_|  |_| |_|",
    "                            |___/                        "
];

pub fn enemy_turn() -> Text<'static>{
    Text::from_iter(ENEMY_TURN)
}

const CROSS_TURN: [&str; 5] = [
    "  ____                     _____                 ",
    " / ___|_ __ ___  ___ ___  |_   _|   _ _ __ _ __  ",
    "| |   | '__/ _ \\/ __/ __|   | || | | | '__| '_ \\ ",
    "| |___| | | (_) \\__ \\__ \\   | || |_| | |  | | | |",
    " \\____|_|  \\___/|___/___/   |_| \\__,_|_|  |_| |_|"
];

pub fn cross_turn() -> Text<'static>{
    Text::from_iter(CROSS_TURN)
}

const CIRCLE_TURN: [&str; 5] = [
    "  ____ _          _        _____                 ",
    " / ___(_)_ __ ___| | ___  |_   _|   _ _ __ _ __  ",
    "| |   | | '__/ __| |/ _ \\   | || | | | '__| '_ \\ ",
    "| |___| | | | (__| |  __/   | || |_| | |  | | | |",
    " \\____|_|_|  \\___|_|\\___|   |_| \\__,_|_|  |_| |_|"
];

pub fn circle_turn() -> Text<'static>{
    Text::from_iter(CIRCLE_TURN)
}

const YOU_WON: [&str; 5] = [
    "__   __                                ",
    "\\ \\ / /__  _   _  __      _____  _ __  ",
    " \\ V / _ \\| | | | \\ \\ /\\ / / _ \\| '_ \\ ",
    "  | | (_) | |_| |  \\ V  V / (_) | | | |",
    "  |_|\\___/ \\__,_|   \\_/\\_/ \\___/|_| |_|",
];

pub fn you_won() -> Text<'static>{
    Text::from_iter(YOU_WON)
}

const YOU_LOST: [&str; 5] = [
    "__   __            _           _   ",
    "\\ \\ / /__  _   _  | | ___  ___| |_ ",
    " \\ V / _ \\| | | | | |/ _ \\/ __| __|",
    "  | | (_) | |_| | | | (_) \\__ \\ |_ ",
    "  |_|\\___/ \\__,_| |_|\\___/|___/\\__|",
];

pub fn you_lost() -> Text<'static>{
    Text::from_iter(YOU_LOST)
}

const CIRCLE_WON: [&str; 5] = [
    "  ____ _          _       __        __          ",
    " / ___(_)_ __ ___| | ___  \\ \\      / /__  _ __  ",
    "| |   | | '__/ __| |/ _ \\  \\ \\ /\\ / / _ \\| '_ \\ ",
    "| |___| | | | (__| |  __/   \\ V  V / (_) | | | |",
    " \\____|_|_|  \\___|_|\\___|    \\_/\\_/ \\___/|_| |_|"
];

pub fn circle_won() -> Text<'static>{
    Text::from_iter(CIRCLE_WON)
}

const CROSS_WON: [&str; 5] = [
    "  ____                    __        __          ",
    " / ___|_ __ ___  ___ ___  \\ \\      / /__  _ __  ",
    "| |   | '__/ _ \\/ __/ __|  \\ \\ /\\ / / _ \\| '_ \\ ",
    "| |___| | | (_) \\__ \\__ \\   \\ V  V / (_) | | | |",
    " \\____|_|  \\___/|___/___/    \\_/\\_/ \\___/|_| |_|"
];

pub fn cross_won() -> Text<'static>{
    Text::from_iter(CROSS_WON)
}

const DRAW: [&str; 5] = [
    " ____  ____      ___        __",
    "|  _ \\|  _ \\    / \\ \\      / /",
    "| | | | |_) |  / _ \\ \\ /\\ / / ",
    "| |_| |  _ <  / ___ \\ V  V /",
    "|____/|_| \\_\\/_/   \\_\\_/\\_/   "
];

pub fn draw() -> Text<'static>{
    Text::from_iter(DRAW)
}


const X: [&str; 4] = [
    "__  __",
    "\\ \\/ /",
    " >  < ",
    "/_/\\_\\",
];

pub fn x() -> Text<'static>{
    Text::from_iter(X)
}

const O: [&str; 4] = [
    "  ___  ",
    " / _ \\ ",
    "| (_) |",
    " \\___/ ",
];

pub fn o() -> Text<'static>{
    Text::from_iter(O)
}

const ANGRY_FACE: [&str; 7] = [
    "    █      █    ",
    "     █    █     ",
    "   ██ █  █ ██   ",
    "   ██      ██   ",
    "                ",
    " ████ ██████  ██",
    "██  █████  ████ ",
];

pub fn angry_face() -> Text<'static> {
    Text::from_iter(ANGRY_FACE)
}

const SMILEY_FACE: [&str; 6] = [
    "     ██      ██     ",
    "     ██      ██     ",
    "                    ",
    "██                ██",
    " ██              ██ ",
    "  ████████████████  "
];

pub fn smiley_face() -> Text<'static>{
    Text::from_iter(SMILEY_FACE)
}

const THINKING_FACE: [&str; 7] = [
    "                 ███",
    "                   █",
    "    ██   ██      ███",
    "    ██   ██      █  ",
    "                    ",
    "                 █  ",
    "  ████████████      "
];

pub fn thinking_face() -> Text<'static>{
    Text::from_iter(THINKING_FACE)
}

const HAPPY_FACE: [&str; 6] = [
    "   ███    ███   ",
    "   ███    ███   ",
    "                ",
    "█              █",
    "███          ███",
    "  ████████████  "
];

pub fn happy_face() -> Text<'static>{
    Text::from_iter(HAPPY_FACE)
}
