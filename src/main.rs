extern crate gtk;

use std::io::prelude::*;
use std::fs::File;
use std::thread::spawn;
use gtk::traits::*;

use gtk::signal::Inhibit;

enum Register {
    BIT_8(u8),
    BIT_16(u16),
    None,
}

enum ValueType {
    IMM(u8),
    ADDR(u16),
}

use Register::{BIT_8,BIT_16};

enum Opcodes {
    ACI(u8),
    ADC_A(u8),
    ADC_B(u8),
    ADC_C(u8),
    ADC_D(u8),
    ADC_E(u8),
    ADC_H(u8),
    ADC_L(u8),
}

struct CPU {
    A:Register,
    BC:(Register,Register),
    DE:(Register,Register),
    HL:(Register,Register),
    PC:Register,
    SP:Register,
}

struct Flag {
    S:u8,
    Z:u8,
    AC:u8,
    P:u8,
    C:u8,
}

impl CPU {
    fn new() -> Self {
        CPU {A:BIT_8(0),BC:(BIT_8(0),BIT_8(0)),DE:(BIT_8(0),BIT_8(0)),HL:(BIT_8(0),BIT_8(0)),PC:BIT_16(0),SP:BIT_16(0)}
    }
}

fn main() {
    let cpu = CPU::new();
    println!("{:X}",0xabc);
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    window.set_title("Rustasm - 8085 Emulator in Rust 0.0.1");
    window.set_default_size(800,200);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let button = gtk::Button::new_with_label("Click me!").unwrap();
    let button2 = gtk::Button::new_with_label("Click me!").unwrap();
    window.add(&button);
    window.add(&button2);
    window.show_all();
    gtk::main();
}