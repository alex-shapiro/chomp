// error-pattern:error: unexpected token: `@`

#[macro_use]
extern crate chomp;

use chomp::{Input, ParseResult};

fn main() {
    fn f(i: Input<u8>) -> ParseResult<u8, u8, ()> {
        i.ret(3)
    }

    let r: ParseResult<u8, u8, ()> = parse!{Input::new(b"5"); let x = f()};
}
