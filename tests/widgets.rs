#[feature(globs, macro_rules, managed_boxes)];

extern mod wx;

use wx::core::*;
use wx::defs::*;

mod macros;

wxApp!(wx_main)

enum ControlIDs {
    Widgets_ClearLog = 100,
    Widgets_Quit,

    Widgets_BookCtrl,

    Widgets_SetTooltip,
    Widgets_SetFgColour,
    Widgets_SetBgColour,
    Widgets_SetPageBg,
    Widgets_SetFont,
    Widgets_Enable,

    Widgets_BorderNone,
    Widgets_BorderStatic,
    Widgets_BorderSimple,
    Widgets_BorderRaised,
    Widgets_BorderSunken,
    Widgets_BorderDouble,
    Widgets_BorderDefault,

    Widgets_LayoutDirection,

    Widgets_GlobalBusyCursor,
    Widgets_BusyCursor,

    Widgets_GoToPage,
//    Widgets_GoToPageLast = Widgets_GoToPage + 100,
}
enum ControlIDs2 {
    TextEntry_DisableAutoComplete = 200,
    TextEntry_AutoCompleteFixed,
    TextEntry_AutoCompleteFilenames,
    TextEntry_AutoCompleteDirectories,
    TextEntry_AutoCompleteCustom,

    TextEntry_SetHint,
    TextEntry_End
}

struct WidgetsFrame {
    base: @wxFrame
}
impl WidgetsFrame {
    fn new() -> @WidgetsFrame {
        let frame = wxFrame::new(wxWindow::null(), wxID_ANY, "wxWidgets",
                -1, -1, -1, -1, wxDEFAULT_FRAME_STYLE);

        // TODO: set icon (sample.xpm)
        // frame.setIcon();
        
        let mbar = wxMenuBar::new(0);
        let menuWidget = wxMenu::new("", 0);
        menuWidget.append(Widgets_SetTooltip, "Set &tooltip...\tCtrl-T", "", 0);
        menuWidget.appendSeparator();
        menuWidget.append(Widgets_SetFgColour, "Set &foreground...\tCtrl-F", "", 0);
        menuWidget.append(Widgets_SetBgColour, "Set &background...\tCtrl-B", "", 0);
        menuWidget.append(Widgets_SetPageBg,   "Set &page background...\tShift-Ctrl-B", "", 0);
        menuWidget.append(Widgets_SetFont,     "Set f&ont...\tCtrl-O", "", 0);
        menuWidget.appendCheckItem(Widgets_Enable,  "&Enable/disable\tCtrl-E");

        let menuBorders = wxMenu::new("", 0);
        menuBorders.appendRadioItem(Widgets_BorderDefault, "De&fault\tCtrl-Shift-9");
        menuBorders.appendRadioItem(Widgets_BorderNone,   "&None\tCtrl-Shift-0");
        menuBorders.appendRadioItem(Widgets_BorderSimple, "&Simple\tCtrl-Shift-1");
        menuBorders.appendRadioItem(Widgets_BorderDouble, "&Double\tCtrl-Shift-2");
        menuBorders.appendRadioItem(Widgets_BorderStatic, "Stati&c\tCtrl-Shift-3");
        menuBorders.appendRadioItem(Widgets_BorderRaised, "&Raised\tCtrl-Shift-4");
        menuBorders.appendRadioItem(Widgets_BorderSunken, "S&unken\tCtrl-Shift-5");
        menuWidget.appendSubMenu(menuBorders, "Set &border");

        menuWidget.appendSeparator();
        menuWidget.appendCheckItem(Widgets_LayoutDirection,
                                    "Toggle &layout direction\tCtrl-L");

        menuWidget.appendSeparator();
        menuWidget.appendCheckItem(Widgets_GlobalBusyCursor,
                                    "Toggle &global busy cursor\tCtrl-Shift-U");
        menuWidget.appendCheckItem(Widgets_BusyCursor,
                                    "Toggle b&usy cursor\tCtrl-U");

        menuWidget.appendSeparator();
        menuWidget.append(wxID_EXIT, "&Quit\tCtrl-Q", "", 0);
        mbar.append(menuWidget, "&Widget", "", 0);

        let menuTextEntry = wxMenu::new("", 0);
        menuTextEntry.appendRadioItem(TextEntry_DisableAutoComplete,
                                       "&Disable auto-completion");
        menuTextEntry.appendRadioItem(TextEntry_AutoCompleteFixed,
                                       "Fixed-&list auto-completion");
        menuTextEntry.appendRadioItem(TextEntry_AutoCompleteFilenames,
                                       "&Files names auto-completion");
        menuTextEntry.appendRadioItem(TextEntry_AutoCompleteDirectories,
                                       "&Directories names auto-completion");
        menuTextEntry.appendRadioItem(TextEntry_AutoCompleteCustom,
                                       "&Custom auto-completion");
        menuTextEntry.appendSeparator();
        menuTextEntry.append(TextEntry_SetHint, "Set help &hint", "", 0);

        mbar.append(menuTextEntry, "&Text", "", 0);

        frame.setMenuBar(mbar);

        mbar.check(Widgets_Enable, true);

        @WidgetsFrame {
            base: frame,
        }
    }
}

fn wx_main() {
    let frame = WidgetsFrame::new();
    let f = frame.base;
    f.show();
    f.raise();
}
