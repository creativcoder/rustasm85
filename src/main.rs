extern crate gtk;
extern crate regex;

//use std::io::prelude::*;
use std::thread::spawn;
use gtk::traits::*;

use std::collections::HashMap;
use std::io::BufWriter;
use std::io::BufReader;
use gtk::{Orientation};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

// trait imports 
use std::io::BufRead;
use std::borrow::ToOwned;
use gtk::signal::Inhibit;
use std::io::Read;
//use gtk::{WindowTrait, ContainerTrait, WidgetTrait, ButtonSignals, BoxTrait};

// external lib imports
use regex::Regex;

const STATUS_SRC_VALID : u8 = 0;
const STATUS_SRC_INVALID : u8 = 1;

struct Cpu {
	a:u8,
	bc:(u8,u8),
	de:(u8,u8),
	hl:(u8,u8),
	sp:u16,
	pc:u16,
	flag:(FL,FL,FL,FL,FL,FL,FL,FL),
}

enum FL {
	T=1,
	F=0,
}
use FL::*;

impl Cpu {
	fn init() -> Self {
		Cpu {
			a:0,
			bc:(0,0),
			de:(0,0),
			hl:(0,0),
			sp:0xff,
			pc:0,
			flag:(F,F,F,F,F,F,F,F),
		}
	}
    fn execute(&mut self,code_line:&str) {
    }
}

enum TYPE {
	BYTE_1,
	BYTE_2,
	BYTE_3,
	INVALID
}

use TYPE::*;

fn hex_code(line:Line) -> TYPE {
	match line.len {
		1 => BYTE_1,
		2 => BYTE_2,
		3 => BYTE_3,
		_ => {INVALID},
	}
}

struct Line {
    len:u8,
}

fn incr_addr(addr:&u16) -> u16 {
	addr+1
}

fn read_asm() {
	// just for test
	let opcode_list = vec!["LXI H","MVI A","MVI B","ADD B","HLT"];

	fn asm_linter(line:&str) -> bool {
			// early return
			if line.len() < 2 {return false;}
			let ins_format:Regex = if line.len() > 5 {
						// matches opcodes with an explicit data
            			Regex::new(r"^\w+\s[ABCDEHLM],\s?((\d{2})|(\d{4}))h$").unwrap()
            		} else {
            			// matches opcodes without an explicit data
            			Regex::new(r"^(\w{1,4}\s[A-Z])|(\w{1,4})$").unwrap()
            		};
            		if ins_format.is_match(line) { true} else { false}
		}

	match File::open("src/tests/src.asm") {
		Ok(handle) => {
			let mut addr = 0u16;
			let mut code_map = HashMap::new();
			let mut status = STATUS_SRC_VALID;
			let mut line_count = 0;
			let mut reader = BufReader::new(handle);
			let mut line = String::new();
            reader.read_line(&mut line);
            line_count+=1;
            while line.trim() != ""{
            	
            	// inner block
            	{	
            		if asm_linter(line.trim()) && opcode_list.contains(&line.trim().split(",").nth(0).unwrap()) {
            			println!("ASM:Linter Status for line {} : Pass",line_count);
            			// Todo
            			code_map.insert(line_count,line.trim().to_string());
            		} else {
            			println!("Syntax Error at line {}",line_count);
            			status = STATUS_SRC_INVALID;
            		}
            		line_count+=1;
            	}
            	// clear line buffer and read next line
            	line.clear();
            	reader.read_line(&mut line);
            }
            // checking if source file is syntactically clean
            if status == STATUS_SRC_INVALID {
            	println!("Code contains syntax Errors.");
            	code_map.clear();
            }
            else {
            	let mut cpu = Cpu::init();
            }
		},
		Err(why) => println!("{:?}",why),
	}
}

fn main() {

    // gtk routines
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    window.set_title("Rustasm - 8085 Emulator in Rust 0.0.1");
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(800,400);

    window.connect_delete_event(|_, _| {
        let dialog = gtk::Dialog::with_buttons(
            "Quit !", None, gtk::DIALOG_MODAL,
            [("Yes", 1), ("No", 0)]);
        // ret has return value from dialog
        
        let ret = dialog.run();
        dialog.destroy();
        if ret==1 {gtk::main_quit();}
        Inhibit(false)
    });

    let outer_frame = gtk::Frame::new(Some("Rustasm 0.0.1")).unwrap();

    /*let code_frame = gtk::Frame::new(Some("Code")).unwrap();
    code_frame.set_border_width(7);
    let register_frame = gtk::Frame::new(Some("Registers")).unwrap();
    register_frame.set_border_width(7);*/

    // toolbar contains the basic functions for running Rustasm
    let toolbar = gtk::Toolbar::new().unwrap();
    let open_icon = gtk::Image::new_from_icon_name("document-open",
                                                   gtk::IconSize::SmallToolbar as i32).unwrap();
    let open_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Open asm Source")).unwrap();
    let compile_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Compile")).unwrap();
    let debug_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Debug")).unwrap();
    let run_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Run")).unwrap();
    let button = gtk::Button::new_with_label("Whattttt a button !").unwrap();
    run_button.set_is_important(true);
    open_button.set_is_important(true);
    compile_button.set_is_important(true);
    debug_button.set_is_important(true);
    
    toolbar.add(&open_button);
    toolbar.add(&compile_button);
    toolbar.add(&debug_button);
    toolbar.add(&run_button);
    toolbar.add(&button);

    // Text view which contains asm source file
    // This is passed as a reference to the ScrolledWindow
    let text_view = gtk::TextView::new().unwrap();
    


    let scroll = gtk::ScrolledWindow::new(None, None).unwrap();
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.add(&text_view);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10).unwrap();
    vbox.add(&toolbar);
    vbox.pack_start(&scroll, true, true, 0);

   /* run_button.connect_clicked(move || {
    	
    });*/

    run_button.connect_clicked(move |_| {
        let dialog = gtk::Dialog::with_buttons(
            "Run without Saving !", None, gtk::DIALOG_MODAL,
            [("Yes", 1), ("No", 0)]);
        // ret has return value from dialog
        let ret = dialog.run();
        dialog.destroy();
    });

    open_button.connect_clicked(move |_| {
    	let file_chooser = gtk::FileChooserDialog::new("Open File", None,gtk::FileChooserAction::Open,
    		[("Open",gtk::ResponseType::Ok),("Cancel",gtk::ResponseType::Cancel)]
    		);
    	if file_chooser.run() == gtk::ResponseType::Ok as i32 {
    		let filename = file_chooser.get_filename().unwrap();
    		let file = File::open(&filename).unwrap();
    		let mut reader = BufReader::new(file);
    		let mut contents = String::new();
    		let _ = reader.read_to_string(&mut contents);
    		text_view.get_buffer().unwrap().set_text(&contents);
            read_asm();
    	}
    	file_chooser.destroy();
    });

    // setting outer frame's child widgets
    outer_frame.add(&vbox);
    // adding frame to parent window
    window.add(&outer_frame);
    window.set_decorated(true);
    window.show_all();
    gtk::main();
    /*read_asm();*/
}
