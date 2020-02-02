#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate reqwest;

mod fpl;

fn main() {
    fpl::start();
}
