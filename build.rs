use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

extern crate gcc;
extern crate bindgen;

fn main() {
    update_submodules();
    patch_wxc_glue();
	build_libwxc();
    export_linker_flags();
	generate_unsafe_rs();
	generate_other_rs();
}

fn update_submodules() {
    println!("build.rs : update submodules");

    let curr_dir_str = env::var("CARGO_MANIFEST_DIR").unwrap();
    let curr_dir = Path::new(&curr_dir_str);
    Command::new("git").arg("submodule").arg("update").arg("--init")
        .current_dir(&curr_dir).status().unwrap();

    println!("build.rs : update submodules - done");
}

fn path_exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        Err(_) => false
    }
}

fn build_libwxc() {
    println!("build.rs : check if libwxc already exists");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("libwxc.a");
    if path_exists(&dest_path) {
        //no need to rebuild libwxc.a
        //just export linker flags
        println!("cargo:rustc-link-search=native={}", &out_dir);
        println!("cargo:rustc-link-lib=static=wxc");
        println!("build.rs : libwxc already exists");
        return;
    }

    println!("build.rs : build libwxc");

	let files = vec![
		"apppath.cpp", 
		"dragimage.cpp", 
		"eljaccelerator.cpp", 
		"eljartprov.cpp", 
		"eljbitmap.cpp", 
		"eljbrush.cpp", 
		"eljbusyinfo.cpp", 
		"eljbutton.cpp", 
		"eljcalendarctrl.cpp", 
		"eljcaret.cpp", 
		"eljcheckbox.cpp", 
		"eljchecklistbox.cpp", 
		"eljchoice.cpp", 
		"eljclipboard.cpp", 
		"eljcoldata.cpp", 
		"eljcolour.cpp", 
		"eljcolourdlg.cpp", 
		"eljcombobox.cpp", 
		"eljconfigbase.cpp", 
		"eljcontrol.cpp", 
		"eljctxhelp.cpp", 
		"eljcursor.cpp", 
		"eljdataformat.cpp", 
		"eljdatetime.cpp", 
		"eljdc.cpp", 
		"eljdcsvg.cpp", 
		"eljdialog.cpp", 
		"eljdirdlg.cpp", 
		"eljdnd.cpp", 
		"eljdrawing.cpp", 
		"eljevent.cpp", 
		"eljfiledialog.cpp", 
		"eljfilehist.cpp", 
		"eljfindrepldlg.cpp", 
		"eljfont.cpp", 
		"eljfontdata.cpp", 
		"eljfontdlg.cpp", 
		"eljframe.cpp", 
		"eljgauge.cpp", 
		"eljgrid.cpp", 
		"eljhelpcontroller.cpp", 
		"eljicnbndl.cpp", 
		"eljicon.cpp", 
		"eljimage.cpp", 
		"eljimagelist.cpp", 
		"eljlayoutconstraints.cpp", 
		"eljlistbox.cpp", 
		"eljlistctrl.cpp", 
		"eljlocale.cpp", 
		"eljlog.cpp", 
		"eljmask.cpp", 
		"eljmdi.cpp", 
		"eljmenu.cpp", 
		"eljmenubar.cpp", 
		"eljmessagedialog.cpp", 
		"eljmime.cpp", 
		"eljminiframe.cpp", 
		"eljnotebook.cpp", 
		"eljpalette.cpp", 
		"eljpanel.cpp", 
		"eljpen.cpp", 
		"eljprintdlg.cpp", 
		"eljprinting.cpp", 
		"eljprocess.cpp", 
		"eljradiobox.cpp", 
		"eljradiobutton.cpp", 
		"eljrc.cpp", 
		"eljregion.cpp", 
		"eljregioniter.cpp", 
		"eljsash.cpp", 
		"eljscrollbar.cpp", 
		"eljscrolledwindow.cpp", 
		"eljsingleinst.cpp", 
		"eljsizer.cpp", 
		"eljslider.cpp", 
		"eljspinctrl.cpp", 
		"eljsplitterwindow.cpp", 
		"eljstaticbox.cpp", 
		"eljstaticline.cpp", 
		"eljstatictext.cpp", 
		"eljstatusbar.cpp", 
		"eljsystemsettings.cpp", 
		"eljtextctrl.cpp", 
		"eljtimer.cpp", 
		"eljtipwnd.cpp", 
		"eljtglbtn.cpp", 
		"eljtoolbar.cpp", 
		"eljvalidator.cpp", 
		"eljwindow.cpp", 
		"eljwizard.cpp", 
		"ewxw_main.cpp", 
		"extra.cpp", 
		"glcanvas.cpp", 
		"graphicscontext.cpp", 
		"image.cpp", 
		"managed.cpp", 
		"mediactrl.cpp", 
		"previewframe.cpp", 
		"printout.cpp", 
		"sckaddr.cpp", 
		"socket.cpp", 
		"sound.cpp", 
		"stc.cpp", 
		"std.cpp", 
		"taskbaricon.cpp", 
		"textstream.cpp", 
		"treectrl.cpp", 
		"wrapper.cpp", 
	];

	let mut config = gcc::Config::new();
	for flag in wx_config(&["--cxxflags"]).split_whitespace() {
		config.flag(flag);
	}
	for file in files {
		config.file(format!("wxHaskell/wxc/src/cpp/{}", file));
	}
	config
		.include("wxHaskell/wxc/src/include")
		.compile("libwxc.a");

    println!("build.rs : build libwxc - done");
}

fn wx_config(args: &[&str]) -> String {
	let output = Command::new("wx-config")
		.args(args)
		.output()
		.unwrap();
	let flags = String::from_utf8_lossy(&output.stdout);
	flags.to_string()
}

fn export_linker_flags() {
    println!("build.rs : export global linker flags");

    //returns some like 
    //-L/usr/lib64 -pthread   -lwx_gtk2u_xrc-3.0 -lwx_gtk2u_webview-3.0 -lwx_gtk2u_stc-3.0
    //-lwx_gtk2u_richtext-3.0 -lwx_gtk2u_ribbon-3.0 -lwx_gtk2u_propgrid-3.0 -lwx_gtk2u_aui-3.0
    //-lwx_gtk2u_gl-3.0 -lwx_gtk2u_media-3.0 -lwx_gtk2u_html-3.0 -lwx_gtk2u_qa-3.0
    //-lwx_gtk2u_adv-3.0 -lwx_gtk2u_core-3.0 -lwx_baseu_xml-3.0 -lwx_baseu_net-3.0 -lwx_baseu-3.0
	for flag in wx_config(&["--libs", "all"]).split_whitespace() {
		if flag.starts_with("-L") {
            println!("cargo:rustc-link-search=native={}", &flag[2..]);
        } else if flag.starts_with("-l") {
            println!("cargo:rustc-link-lib=dylib={}", &flag[2..]);
        }
	}
    println!("cargo:rustc-link-lib=dylib=stdc++");

    println!("build.rs : export global linker flags - done");
}


struct BindgenLogger;

impl bindgen::Logger for BindgenLogger {
    fn error(&self, msg: &str) {
        println!("bindgen error:  {}", msg);
    }

    fn warn(&self, msg: &str) {
        println!("bindgen warning: {}", msg);
    }
}

fn generate_unsafe_rs() {
    println!("build.rs : generate unsafe rs");

    let logger = BindgenLogger;
    let mut bindings = bindgen::builder();
    bindings.forbid_unknown_types();
    bindings.log(&logger);

    
	for flag in wx_config(&["--cxxflags"]).split_whitespace() {
		bindings.clang_arg(flag);
	}

    let args = [
        "-x", "c++", 
        "--include", "stdint.h",
        "--include", "time.h",
        "wxHaskell/wxc/src/include/wxc.h"
    ];

	for flag in args.iter() {
		bindings.clang_arg(*flag);
	}

    match bindgen::get_include_dir() {
        Some(path) => {
            bindings.clang_arg("-I");
            bindings.clang_arg(path);
        }
        None => (),
    }

    bindings.match_pat("wxc");

    let binding = bindings.generate().unwrap();

	let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let unsafe_rs = Path::new(&root_dir).join("src").join("_unsafe.rs");
	let mut file = File::create(&unsafe_rs).unwrap();

    file.write_all("/* added by build.rs */\n".as_bytes()).unwrap();
    file.write_all("use libc::*;\n\n".as_bytes()).unwrap();

    binding.write(Box::new(file)).unwrap();
    
    println!("build.rs : generate unsafe rs - done");
}

fn patch_wxc_glue() {
    println!("build.rs : patching wxHaskell/wxc/src/include/wxc_glue.h");

    //patch -N -p1 -i ../wxc/wxc_glue_h.patch
	let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let wxhaskel_dir = Path::new(&root_dir).join("wxHaskell");

	Command::new("patch")
        .current_dir(wxhaskel_dir)
		.args(&["-N", "-p1", "-i", "../wxc/wxc_glue_h.patch"])
		.status()
		.unwrap();

    println!("build.rs : patching wxHaskell/wxc/src/include/wxc_glue.h - done");
}

fn generate_other_rs() {
    println!("build.rs : running src/codegen.py");

	Command::new("python")
		.args(&["src/codegen.py", "wxHaskell/wxc/src/include/wxc.h"])
		.status()
		.unwrap();

    println!("build.rs : running src/codegen.py - done");
}
