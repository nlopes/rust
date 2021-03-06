// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
// ignore-emscripten no threads support

use std::thread;
use std::sync::mpsc::{channel, Sender};

pub fn main() {
    let (tx, rx) = channel();
    let t = thread::spawn(move|| { child(&tx) });
    let y = rx.recv().unwrap();
    println!("received");
    println!("{}", y);
    assert_eq!(y, 10);
    t.join();
}

fn child(c: &Sender<isize>) {
    println!("sending");
    c.send(10).unwrap();
    println!("value sent");
}
