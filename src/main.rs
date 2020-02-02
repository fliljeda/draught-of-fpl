extern crate reqwest;

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


mod fpl;

fn main() {
    fpl::start();
}
