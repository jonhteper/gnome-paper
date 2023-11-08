use chrono::Local;
use std::io;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

mod application;
mod config;
mod errors;
mod image;

#[cfg(test)]
mod tests;

pub fn main() {}
