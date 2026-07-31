// allowed character set is limited to what the linux kernel console font can render
// ref https://wiki.archlinux.org/title/Linux_console
// ref https://en.wikipedia.org/wiki/Code_page_437
pub fn is_safe(ch: char) -> bool {
    let code = ch as u32;

    if (0x20..=0x7e).contains(&code) {
        return true;
    }

    if (0xa0..=0xff).contains(&code) {
        return true;
    }

    if (0x2500..=0x259f).contains(&code) {
        return true;
    }

    (0x2190..=0x2193).contains(&code)
}

// returns a safe replacement if the input char is not in the allowed set
// replacement strategy
// unknown box drawing becomes ascii dash or pipe
// unknown block becomes space
// everything else becomes question mark
pub fn sanitize(ch: char) -> char {
    if is_safe(ch) {
        return ch;
    }

    let code = ch as u32;

    if (0x2500..=0x257f).contains(&code) {
        if code <= 0x250f || (0x2550..=0x255f).contains(&code) {
            return '-';
        }
        return '|';
    }

    if (0x2580..=0x259f).contains(&code) {
        return ' ';
    }

    '?'
}
