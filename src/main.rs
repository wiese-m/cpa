use std::io;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn main() {
    let stdin = io::read_to_string(io::stdin()).unwrap();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(stdin.to_owned()).unwrap();
    ctx.get_contents().unwrap();
}
