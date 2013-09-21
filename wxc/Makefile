CXX		= clang++
CXXFLAGS	= `wx-config --cppflags` -I ../wxHaskell/wxc/src/include -DwxcREFUSE_MEDIACTRL
LD		= $(CXX)
LDFLAGS		= `wx-config --libs all`
SRCDIR		= ../wxHaskell/wxc/src/cpp/

OBJS		= \
	$(SRCDIR)apppath.o \
	$(SRCDIR)dragimage.o \
	$(SRCDIR)eljaccelerator.o \
	$(SRCDIR)eljartprov.o \
	$(SRCDIR)eljbitmap.o \
	$(SRCDIR)eljbrush.o \
	$(SRCDIR)eljbusyinfo.o \
	$(SRCDIR)eljbutton.o \
	$(SRCDIR)eljcalendarctrl.o \
	$(SRCDIR)eljcaret.o \
	$(SRCDIR)eljcheckbox.o \
	$(SRCDIR)eljchecklistbox.o \
	$(SRCDIR)eljchoice.o \
	$(SRCDIR)eljclipboard.o \
	$(SRCDIR)eljcoldata.o \
	$(SRCDIR)eljcolour.o \
	$(SRCDIR)eljcolourdlg.o \
	$(SRCDIR)eljcombobox.o \
	$(SRCDIR)eljconfigbase.o \
	$(SRCDIR)eljcontrol.o \
	$(SRCDIR)eljctxhelp.o \
	$(SRCDIR)eljcursor.o \
	$(SRCDIR)eljdataformat.o \
	$(SRCDIR)eljdatetime.o \
	$(SRCDIR)eljdc.o \
	$(SRCDIR)eljdcsvg.o \
	$(SRCDIR)eljdialog.o \
	$(SRCDIR)eljdirdlg.o \
	$(SRCDIR)eljdnd.o \
	$(SRCDIR)eljdrawing.o \
	$(SRCDIR)eljevent.o \
	$(SRCDIR)eljfiledialog.o \
	$(SRCDIR)eljfilehist.o \
	$(SRCDIR)eljfindrepldlg.o \
	$(SRCDIR)eljfont.o \
	$(SRCDIR)eljfontdata.o \
	$(SRCDIR)eljfontdlg.o \
	$(SRCDIR)eljframe.o \
	$(SRCDIR)eljgauge.o \
	$(SRCDIR)eljgrid.o \
	$(SRCDIR)eljhelpcontroller.o \
	$(SRCDIR)eljicnbndl.o \
	$(SRCDIR)eljicon.o \
	$(SRCDIR)eljimage.o \
	$(SRCDIR)eljimagelist.o \
	$(SRCDIR)eljlayoutconstraints.o \
	$(SRCDIR)eljlistbox.o \
	$(SRCDIR)eljlistctrl.o \
	$(SRCDIR)eljlocale.o \
	$(SRCDIR)eljlog.o \
	$(SRCDIR)eljmask.o \
	$(SRCDIR)eljmdi.o \
	$(SRCDIR)eljmenu.o \
	$(SRCDIR)eljmenubar.o \
	$(SRCDIR)eljmessagedialog.o \
	$(SRCDIR)eljmime.o \
	$(SRCDIR)eljminiframe.o \
	$(SRCDIR)eljnotebook.o \
	$(SRCDIR)eljpalette.o \
	$(SRCDIR)eljpanel.o \
	$(SRCDIR)eljpen.o \
	$(SRCDIR)eljprintdlg.o \
	$(SRCDIR)eljprinting.o \
	$(SRCDIR)eljprocess.o \
	$(SRCDIR)eljradiobox.o \
	$(SRCDIR)eljradiobutton.o \
	$(SRCDIR)eljrc.o \
	$(SRCDIR)eljregion.o \
	$(SRCDIR)eljregioniter.o \
	$(SRCDIR)eljsash.o \
	$(SRCDIR)eljscrollbar.o \
	$(SRCDIR)eljscrolledwindow.o \
	$(SRCDIR)eljsingleinst.o \
	$(SRCDIR)eljsizer.o \
	$(SRCDIR)eljslider.o \
	$(SRCDIR)eljspinctrl.o \
	$(SRCDIR)eljsplitterwindow.o \
	$(SRCDIR)eljstaticbox.o \
	$(SRCDIR)eljstaticline.o \
	$(SRCDIR)eljstatictext.o \
	$(SRCDIR)eljstatusbar.o \
	$(SRCDIR)eljsystemsettings.o \
	$(SRCDIR)eljtextctrl.o \
	$(SRCDIR)eljtimer.o \
	$(SRCDIR)eljtipwnd.o \
	$(SRCDIR)eljtglbtn.o \
	$(SRCDIR)eljtoolbar.o \
	$(SRCDIR)eljvalidator.o \
	$(SRCDIR)eljwindow.o \
	$(SRCDIR)eljwizard.o \
	$(SRCDIR)ewxw_main.o \
	$(SRCDIR)extra.o \
	$(SRCDIR)glcanvas.o \
	$(SRCDIR)graphicscontext.o \
	$(SRCDIR)image.o \
	$(SRCDIR)managed.o \
	$(SRCDIR)mediactrl.o \
	$(SRCDIR)previewframe.o \
	$(SRCDIR)printout.o \
	$(SRCDIR)sckaddr.o \
	$(SRCDIR)socket.o \
	$(SRCDIR)sound.o \
	$(SRCDIR)stc.o \
	$(SRCDIR)std.o \
	$(SRCDIR)taskbaricon.o \
	$(SRCDIR)textstream.o \
	$(SRCDIR)treectrl.o \
	$(SRCDIR)wrapper.o

libwxc.dylib: $(OBJS)
	$(LD) -shared $(LDFLAGS) -o $@ $(OBJS)

.cpp.o:
	$(CXX) -c $(CXXFLAGS) $< -o $@

.PHONY: clean
clean:
	rm -rf $(SRCDIR)*.o

