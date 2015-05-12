use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

extern crate gcc;

fn main() {
	build_libwxc();
	build_bindgen();
	generate_unsafe_rs();
}

fn build_libwxc() {
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
	for flag in wx_config("--cxxflags").split_whitespace() {
		config.flag(flag);
	}
	for file in files {
		config.file(format!("wxHaskell/wxc/src/cpp/{}", file));
	}
	config
		.include("wxHaskell/wxc/src/include")
		.compile("libwxc.a")
}

fn wx_config(config: &str) -> String {
	let output = Command::new("wx-config")
		.arg(config)
		.output()
		.unwrap();
	let flags = String::from_utf8_lossy(&output.stdout);
	flags.to_string()
}

fn build_bindgen() {
	Command::new("cargo")
		.current_dir("rust-bindgen/")
		.arg("build")
		.status()
		.unwrap_or_else(|e| {
		    panic!("failed to build bindgen: {}", e)
		});
}

fn generate_unsafe_rs() {
	let mut command = Command::new("rust-bindgen/target/debug/bindgen");
	command
		.args(&[
			"-allow-unknown-types",
			"-x", "c++",
		]);
	for flag in wx_config("--cppflags").split_whitespace() {
		command.arg(flag);
	}
	let output = command
		.args(&[
			"--include", "stdint.h",
			"--include", "time.h",
			"wxHaskell/wxc/src/include/wxc.h"
		])
		.output()
		.unwrap();
	let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let unsafe_rs = Path::new(&root_dir).join("_unsafe.rs");
	let mut file = File::create(&unsafe_rs).unwrap();
	
	file.write_all(&output.stdout).unwrap();
}