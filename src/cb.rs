use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use _unavailable::*;

pub struct CbAntiflickerPlugin { ptr: *mut c_void }
impl TCbAntiflickerPlugin for CbAntiflickerPlugin {}
impl TCbPluginBase for CbAntiflickerPlugin {}
impl TEvtHandler for CbAntiflickerPlugin {}
impl TObject for CbAntiflickerPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbAntiflickerPlugin {
    pub fn from(ptr: *mut c_void) -> CbAntiflickerPlugin { CbAntiflickerPlugin { ptr: ptr } }
    pub fn null() -> CbAntiflickerPlugin { CbAntiflickerPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbAntiflickerPlugin : TCbPluginBase {
}

pub struct CbBarDragPlugin { ptr: *mut c_void }
impl TCbBarDragPlugin for CbBarDragPlugin {}
impl TCbPluginBase for CbBarDragPlugin {}
impl TEvtHandler for CbBarDragPlugin {}
impl TObject for CbBarDragPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarDragPlugin {
    pub fn from(ptr: *mut c_void) -> CbBarDragPlugin { CbBarDragPlugin { ptr: ptr } }
    pub fn null() -> CbBarDragPlugin { CbBarDragPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbBarDragPlugin : TCbPluginBase {
}

pub struct CbBarHintsPlugin { ptr: *mut c_void }
impl TCbBarHintsPlugin for CbBarHintsPlugin {}
impl TCbPluginBase for CbBarHintsPlugin {}
impl TEvtHandler for CbBarHintsPlugin {}
impl TObject for CbBarHintsPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarHintsPlugin {
    pub fn from(ptr: *mut c_void) -> CbBarHintsPlugin { CbBarHintsPlugin { ptr: ptr } }
    pub fn null() -> CbBarHintsPlugin { CbBarHintsPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbBarHintsPlugin : TCbPluginBase {
}

pub struct CbBarInfo { ptr: *mut c_void }
impl TCbBarInfo for CbBarInfo {}
impl TObject for CbBarInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarInfo {
    pub fn from(ptr: *mut c_void) -> CbBarInfo { CbBarInfo { ptr: ptr } }
    pub fn null() -> CbBarInfo { CbBarInfo::from(0 as *mut c_void) }
    
}

pub trait TCbBarInfo : TObject {
}

pub struct CbBarSpy { ptr: *mut c_void }
impl TCbBarSpy for CbBarSpy {}
impl TEvtHandler for CbBarSpy {}
impl TObject for CbBarSpy { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarSpy {
    pub fn from(ptr: *mut c_void) -> CbBarSpy { CbBarSpy { ptr: ptr } }
    pub fn null() -> CbBarSpy { CbBarSpy::from(0 as *mut c_void) }
    
}

pub trait TCbBarSpy : TEvtHandler {
}

pub struct CbCloseBox { ptr: *mut c_void }
impl TCbCloseBox for CbCloseBox {}
impl TCbMiniButton for CbCloseBox {}
impl TObject for CbCloseBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCloseBox {
    pub fn from(ptr: *mut c_void) -> CbCloseBox { CbCloseBox { ptr: ptr } }
    pub fn null() -> CbCloseBox { CbCloseBox::from(0 as *mut c_void) }
    
}

pub trait TCbCloseBox : TCbMiniButton {
}

pub struct CbCollapseBox { ptr: *mut c_void }
impl TCbCollapseBox for CbCollapseBox {}
impl TCbMiniButton for CbCollapseBox {}
impl TObject for CbCollapseBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCollapseBox {
    pub fn from(ptr: *mut c_void) -> CbCollapseBox { CbCollapseBox { ptr: ptr } }
    pub fn null() -> CbCollapseBox { CbCollapseBox::from(0 as *mut c_void) }
    
}

pub trait TCbCollapseBox : TCbMiniButton {
}

pub struct CbCommonPaneProperties { ptr: *mut c_void }
impl TCbCommonPaneProperties for CbCommonPaneProperties {}
impl TObject for CbCommonPaneProperties { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCommonPaneProperties {
    pub fn from(ptr: *mut c_void) -> CbCommonPaneProperties { CbCommonPaneProperties { ptr: ptr } }
    pub fn null() -> CbCommonPaneProperties { CbCommonPaneProperties::from(0 as *mut c_void) }
    
}

pub trait TCbCommonPaneProperties : TObject {
}

pub struct CbCustomizeBarEvent { ptr: *mut c_void }
impl TCbCustomizeBarEvent for CbCustomizeBarEvent {}
impl TCbPluginEvent for CbCustomizeBarEvent {}
impl TEvent for CbCustomizeBarEvent {}
impl TObject for CbCustomizeBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCustomizeBarEvent {
    pub fn from(ptr: *mut c_void) -> CbCustomizeBarEvent { CbCustomizeBarEvent { ptr: ptr } }
    pub fn null() -> CbCustomizeBarEvent { CbCustomizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait TCbCustomizeBarEvent : TCbPluginEvent {
}

pub struct CbCustomizeLayoutEvent { ptr: *mut c_void }
impl TCbCustomizeLayoutEvent for CbCustomizeLayoutEvent {}
impl TCbPluginEvent for CbCustomizeLayoutEvent {}
impl TEvent for CbCustomizeLayoutEvent {}
impl TObject for CbCustomizeLayoutEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCustomizeLayoutEvent {
    pub fn from(ptr: *mut c_void) -> CbCustomizeLayoutEvent { CbCustomizeLayoutEvent { ptr: ptr } }
    pub fn null() -> CbCustomizeLayoutEvent { CbCustomizeLayoutEvent::from(0 as *mut c_void) }
    
}

pub trait TCbCustomizeLayoutEvent : TCbPluginEvent {
}

pub struct CbDimHandlerBase { ptr: *mut c_void }
impl TCbDimHandlerBase for CbDimHandlerBase {}
impl TObject for CbDimHandlerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDimHandlerBase {
    pub fn from(ptr: *mut c_void) -> CbDimHandlerBase { CbDimHandlerBase { ptr: ptr } }
    pub fn null() -> CbDimHandlerBase { CbDimHandlerBase::from(0 as *mut c_void) }
    
}

pub trait TCbDimHandlerBase : TObject {
}

pub struct CbDimInfo { ptr: *mut c_void }
impl TCbDimInfo for CbDimInfo {}
impl TObject for CbDimInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDimInfo {
    pub fn from(ptr: *mut c_void) -> CbDimInfo { CbDimInfo { ptr: ptr } }
    pub fn null() -> CbDimInfo { CbDimInfo::from(0 as *mut c_void) }
    
}

pub trait TCbDimInfo : TObject {
}

pub struct CbDockBox { ptr: *mut c_void }
impl TCbDockBox for CbDockBox {}
impl TCbMiniButton for CbDockBox {}
impl TObject for CbDockBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDockBox {
    pub fn from(ptr: *mut c_void) -> CbDockBox { CbDockBox { ptr: ptr } }
    pub fn null() -> CbDockBox { CbDockBox::from(0 as *mut c_void) }
    
}

pub trait TCbDockBox : TCbMiniButton {
}

pub struct CbDockPane { ptr: *mut c_void }
impl TCbDockPane for CbDockPane {}
impl TObject for CbDockPane { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDockPane {
    pub fn from(ptr: *mut c_void) -> CbDockPane { CbDockPane { ptr: ptr } }
    pub fn null() -> CbDockPane { CbDockPane::from(0 as *mut c_void) }
    
}

pub trait TCbDockPane : TObject {
}

pub struct CbDrawBarDecorEvent { ptr: *mut c_void }
impl TCbDrawBarDecorEvent for CbDrawBarDecorEvent {}
impl TCbPluginEvent for CbDrawBarDecorEvent {}
impl TEvent for CbDrawBarDecorEvent {}
impl TObject for CbDrawBarDecorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawBarDecorEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawBarDecorEvent { CbDrawBarDecorEvent { ptr: ptr } }
    pub fn null() -> CbDrawBarDecorEvent { CbDrawBarDecorEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawBarDecorEvent : TCbPluginEvent {
}

pub struct CbDrawBarHandlesEvent { ptr: *mut c_void }
impl TCbDrawBarHandlesEvent for CbDrawBarHandlesEvent {}
impl TCbPluginEvent for CbDrawBarHandlesEvent {}
impl TEvent for CbDrawBarHandlesEvent {}
impl TObject for CbDrawBarHandlesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawBarHandlesEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawBarHandlesEvent { CbDrawBarHandlesEvent { ptr: ptr } }
    pub fn null() -> CbDrawBarHandlesEvent { CbDrawBarHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawBarHandlesEvent : TCbPluginEvent {
}

pub struct CbDrawHintRectEvent { ptr: *mut c_void }
impl TCbDrawHintRectEvent for CbDrawHintRectEvent {}
impl TCbPluginEvent for CbDrawHintRectEvent {}
impl TEvent for CbDrawHintRectEvent {}
impl TObject for CbDrawHintRectEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawHintRectEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawHintRectEvent { CbDrawHintRectEvent { ptr: ptr } }
    pub fn null() -> CbDrawHintRectEvent { CbDrawHintRectEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawHintRectEvent : TCbPluginEvent {
}

pub struct CbDrawPaneBkGroundEvent { ptr: *mut c_void }
impl TCbDrawPaneBkGroundEvent for CbDrawPaneBkGroundEvent {}
impl TCbPluginEvent for CbDrawPaneBkGroundEvent {}
impl TEvent for CbDrawPaneBkGroundEvent {}
impl TObject for CbDrawPaneBkGroundEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawPaneBkGroundEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawPaneBkGroundEvent { CbDrawPaneBkGroundEvent { ptr: ptr } }
    pub fn null() -> CbDrawPaneBkGroundEvent { CbDrawPaneBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawPaneBkGroundEvent : TCbPluginEvent {
}

pub struct CbDrawPaneDecorEvent { ptr: *mut c_void }
impl TCbDrawPaneDecorEvent for CbDrawPaneDecorEvent {}
impl TCbPluginEvent for CbDrawPaneDecorEvent {}
impl TEvent for CbDrawPaneDecorEvent {}
impl TObject for CbDrawPaneDecorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawPaneDecorEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawPaneDecorEvent { CbDrawPaneDecorEvent { ptr: ptr } }
    pub fn null() -> CbDrawPaneDecorEvent { CbDrawPaneDecorEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawPaneDecorEvent : TCbPluginEvent {
}

pub struct CbDrawRowBkGroundEvent { ptr: *mut c_void }
impl TCbDrawRowBkGroundEvent for CbDrawRowBkGroundEvent {}
impl TCbPluginEvent for CbDrawRowBkGroundEvent {}
impl TEvent for CbDrawRowBkGroundEvent {}
impl TObject for CbDrawRowBkGroundEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawRowBkGroundEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawRowBkGroundEvent { CbDrawRowBkGroundEvent { ptr: ptr } }
    pub fn null() -> CbDrawRowBkGroundEvent { CbDrawRowBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawRowBkGroundEvent : TCbPluginEvent {
}

pub struct CbDrawRowDecorEvent { ptr: *mut c_void }
impl TCbDrawRowDecorEvent for CbDrawRowDecorEvent {}
impl TCbPluginEvent for CbDrawRowDecorEvent {}
impl TEvent for CbDrawRowDecorEvent {}
impl TObject for CbDrawRowDecorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawRowDecorEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawRowDecorEvent { CbDrawRowDecorEvent { ptr: ptr } }
    pub fn null() -> CbDrawRowDecorEvent { CbDrawRowDecorEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawRowDecorEvent : TCbPluginEvent {
}

pub struct CbDrawRowHandlesEvent { ptr: *mut c_void }
impl TCbDrawRowHandlesEvent for CbDrawRowHandlesEvent {}
impl TCbPluginEvent for CbDrawRowHandlesEvent {}
impl TEvent for CbDrawRowHandlesEvent {}
impl TObject for CbDrawRowHandlesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawRowHandlesEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawRowHandlesEvent { CbDrawRowHandlesEvent { ptr: ptr } }
    pub fn null() -> CbDrawRowHandlesEvent { CbDrawRowHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawRowHandlesEvent : TCbPluginEvent {
}

pub struct CbDynToolBarDimHandler { ptr: *mut c_void }
impl TCbDynToolBarDimHandler for CbDynToolBarDimHandler {}
impl TCbDimHandlerBase for CbDynToolBarDimHandler {}
impl TObject for CbDynToolBarDimHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDynToolBarDimHandler {
    pub fn from(ptr: *mut c_void) -> CbDynToolBarDimHandler { CbDynToolBarDimHandler { ptr: ptr } }
    pub fn null() -> CbDynToolBarDimHandler { CbDynToolBarDimHandler::from(0 as *mut c_void) }
    
}

pub trait TCbDynToolBarDimHandler : TCbDimHandlerBase {
}

pub struct CbFinishDrawInAreaEvent { ptr: *mut c_void }
impl TCbFinishDrawInAreaEvent for CbFinishDrawInAreaEvent {}
impl TCbPluginEvent for CbFinishDrawInAreaEvent {}
impl TEvent for CbFinishDrawInAreaEvent {}
impl TObject for CbFinishDrawInAreaEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbFinishDrawInAreaEvent {
    pub fn from(ptr: *mut c_void) -> CbFinishDrawInAreaEvent { CbFinishDrawInAreaEvent { ptr: ptr } }
    pub fn null() -> CbFinishDrawInAreaEvent { CbFinishDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait TCbFinishDrawInAreaEvent : TCbPluginEvent {
}

pub struct CbFloatedBarWindow { ptr: *mut c_void }
impl TCbFloatedBarWindow for CbFloatedBarWindow {}
impl TToolWindow for CbFloatedBarWindow {}
impl TFrame for CbFloatedBarWindow {}
impl TTopLevelWindow for CbFloatedBarWindow {}
impl TWindow for CbFloatedBarWindow {}
impl TEvtHandler for CbFloatedBarWindow {}
impl TObject for CbFloatedBarWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbFloatedBarWindow {
    pub fn from(ptr: *mut c_void) -> CbFloatedBarWindow { CbFloatedBarWindow { ptr: ptr } }
    pub fn null() -> CbFloatedBarWindow { CbFloatedBarWindow::from(0 as *mut c_void) }
    
}

pub trait TCbFloatedBarWindow : TToolWindow {
}

pub struct CbGCUpdatesMgr { ptr: *mut c_void }
impl TCbGCUpdatesMgr for CbGCUpdatesMgr {}
impl TCbSimpleUpdatesMgr for CbGCUpdatesMgr {}
impl TCbUpdatesManagerBase for CbGCUpdatesMgr {}
impl TObject for CbGCUpdatesMgr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbGCUpdatesMgr {
    pub fn from(ptr: *mut c_void) -> CbGCUpdatesMgr { CbGCUpdatesMgr { ptr: ptr } }
    pub fn null() -> CbGCUpdatesMgr { CbGCUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait TCbGCUpdatesMgr : TCbSimpleUpdatesMgr {
}

pub struct CbHintAnimationPlugin { ptr: *mut c_void }
impl TCbHintAnimationPlugin for CbHintAnimationPlugin {}
impl TCbPluginBase for CbHintAnimationPlugin {}
impl TEvtHandler for CbHintAnimationPlugin {}
impl TObject for CbHintAnimationPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbHintAnimationPlugin {
    pub fn from(ptr: *mut c_void) -> CbHintAnimationPlugin { CbHintAnimationPlugin { ptr: ptr } }
    pub fn null() -> CbHintAnimationPlugin { CbHintAnimationPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbHintAnimationPlugin : TCbPluginBase {
}

pub struct CbInsertBarEvent { ptr: *mut c_void }
impl TCbInsertBarEvent for CbInsertBarEvent {}
impl TCbPluginEvent for CbInsertBarEvent {}
impl TEvent for CbInsertBarEvent {}
impl TObject for CbInsertBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbInsertBarEvent {
    pub fn from(ptr: *mut c_void) -> CbInsertBarEvent { CbInsertBarEvent { ptr: ptr } }
    pub fn null() -> CbInsertBarEvent { CbInsertBarEvent::from(0 as *mut c_void) }
    
}

pub trait TCbInsertBarEvent : TCbPluginEvent {
}

pub struct CbLayoutRowEvent { ptr: *mut c_void }
impl TCbLayoutRowEvent for CbLayoutRowEvent {}
impl TCbPluginEvent for CbLayoutRowEvent {}
impl TEvent for CbLayoutRowEvent {}
impl TObject for CbLayoutRowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLayoutRowEvent {
    pub fn from(ptr: *mut c_void) -> CbLayoutRowEvent { CbLayoutRowEvent { ptr: ptr } }
    pub fn null() -> CbLayoutRowEvent { CbLayoutRowEvent::from(0 as *mut c_void) }
    
}

pub trait TCbLayoutRowEvent : TCbPluginEvent {
}

pub struct CbLeftDClickEvent { ptr: *mut c_void }
impl TCbLeftDClickEvent for CbLeftDClickEvent {}
impl TCbPluginEvent for CbLeftDClickEvent {}
impl TEvent for CbLeftDClickEvent {}
impl TObject for CbLeftDClickEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLeftDClickEvent {
    pub fn from(ptr: *mut c_void) -> CbLeftDClickEvent { CbLeftDClickEvent { ptr: ptr } }
    pub fn null() -> CbLeftDClickEvent { CbLeftDClickEvent::from(0 as *mut c_void) }
    
}

pub trait TCbLeftDClickEvent : TCbPluginEvent {
}

pub struct CbLeftDownEvent { ptr: *mut c_void }
impl TCbLeftDownEvent for CbLeftDownEvent {}
impl TCbPluginEvent for CbLeftDownEvent {}
impl TEvent for CbLeftDownEvent {}
impl TObject for CbLeftDownEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLeftDownEvent {
    pub fn from(ptr: *mut c_void) -> CbLeftDownEvent { CbLeftDownEvent { ptr: ptr } }
    pub fn null() -> CbLeftDownEvent { CbLeftDownEvent::from(0 as *mut c_void) }
    
}

pub trait TCbLeftDownEvent : TCbPluginEvent {
}

pub struct CbLeftUpEvent { ptr: *mut c_void }
impl TCbLeftUpEvent for CbLeftUpEvent {}
impl TCbPluginEvent for CbLeftUpEvent {}
impl TEvent for CbLeftUpEvent {}
impl TObject for CbLeftUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLeftUpEvent {
    pub fn from(ptr: *mut c_void) -> CbLeftUpEvent { CbLeftUpEvent { ptr: ptr } }
    pub fn null() -> CbLeftUpEvent { CbLeftUpEvent::from(0 as *mut c_void) }
    
}

pub trait TCbLeftUpEvent : TCbPluginEvent {
}

pub struct CbMiniButton { ptr: *mut c_void }
impl TCbMiniButton for CbMiniButton {}
impl TObject for CbMiniButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbMiniButton {
    pub fn from(ptr: *mut c_void) -> CbMiniButton { CbMiniButton { ptr: ptr } }
    pub fn null() -> CbMiniButton { CbMiniButton::from(0 as *mut c_void) }
    
}

pub trait TCbMiniButton : TObject {
}

pub struct CbMotionEvent { ptr: *mut c_void }
impl TCbMotionEvent for CbMotionEvent {}
impl TCbPluginEvent for CbMotionEvent {}
impl TEvent for CbMotionEvent {}
impl TObject for CbMotionEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbMotionEvent {
    pub fn from(ptr: *mut c_void) -> CbMotionEvent { CbMotionEvent { ptr: ptr } }
    pub fn null() -> CbMotionEvent { CbMotionEvent::from(0 as *mut c_void) }
    
}

pub trait TCbMotionEvent : TCbPluginEvent {
}

pub struct CbPaneDrawPlugin { ptr: *mut c_void }
impl TCbPaneDrawPlugin for CbPaneDrawPlugin {}
impl TCbPluginBase for CbPaneDrawPlugin {}
impl TEvtHandler for CbPaneDrawPlugin {}
impl TObject for CbPaneDrawPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbPaneDrawPlugin {
    pub fn from(ptr: *mut c_void) -> CbPaneDrawPlugin { CbPaneDrawPlugin { ptr: ptr } }
    pub fn null() -> CbPaneDrawPlugin { CbPaneDrawPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbPaneDrawPlugin : TCbPluginBase {
}

pub struct CbPluginBase { ptr: *mut c_void }
impl TCbPluginBase for CbPluginBase {}
impl TEvtHandler for CbPluginBase {}
impl TObject for CbPluginBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbPluginBase {
    pub fn from(ptr: *mut c_void) -> CbPluginBase { CbPluginBase { ptr: ptr } }
    pub fn null() -> CbPluginBase { CbPluginBase::from(0 as *mut c_void) }
    
}

pub trait TCbPluginBase : TEvtHandler {
}

pub struct CbPluginEvent { ptr: *mut c_void }
impl TCbPluginEvent for CbPluginEvent {}
impl TEvent for CbPluginEvent {}
impl TObject for CbPluginEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbPluginEvent {
    pub fn from(ptr: *mut c_void) -> CbPluginEvent { CbPluginEvent { ptr: ptr } }
    pub fn null() -> CbPluginEvent { CbPluginEvent::from(0 as *mut c_void) }
    
}

pub trait TCbPluginEvent : TEvent {
}

pub struct CbRemoveBarEvent { ptr: *mut c_void }
impl TCbRemoveBarEvent for CbRemoveBarEvent {}
impl TCbPluginEvent for CbRemoveBarEvent {}
impl TEvent for CbRemoveBarEvent {}
impl TObject for CbRemoveBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRemoveBarEvent {
    pub fn from(ptr: *mut c_void) -> CbRemoveBarEvent { CbRemoveBarEvent { ptr: ptr } }
    pub fn null() -> CbRemoveBarEvent { CbRemoveBarEvent::from(0 as *mut c_void) }
    
}

pub trait TCbRemoveBarEvent : TCbPluginEvent {
}

pub struct CbResizeBarEvent { ptr: *mut c_void }
impl TCbResizeBarEvent for CbResizeBarEvent {}
impl TCbPluginEvent for CbResizeBarEvent {}
impl TEvent for CbResizeBarEvent {}
impl TObject for CbResizeBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbResizeBarEvent {
    pub fn from(ptr: *mut c_void) -> CbResizeBarEvent { CbResizeBarEvent { ptr: ptr } }
    pub fn null() -> CbResizeBarEvent { CbResizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait TCbResizeBarEvent : TCbPluginEvent {
}

pub struct CbResizeRowEvent { ptr: *mut c_void }
impl TCbResizeRowEvent for CbResizeRowEvent {}
impl TCbPluginEvent for CbResizeRowEvent {}
impl TEvent for CbResizeRowEvent {}
impl TObject for CbResizeRowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbResizeRowEvent {
    pub fn from(ptr: *mut c_void) -> CbResizeRowEvent { CbResizeRowEvent { ptr: ptr } }
    pub fn null() -> CbResizeRowEvent { CbResizeRowEvent::from(0 as *mut c_void) }
    
}

pub trait TCbResizeRowEvent : TCbPluginEvent {
}

pub struct CbRightDownEvent { ptr: *mut c_void }
impl TCbRightDownEvent for CbRightDownEvent {}
impl TCbPluginEvent for CbRightDownEvent {}
impl TEvent for CbRightDownEvent {}
impl TObject for CbRightDownEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRightDownEvent {
    pub fn from(ptr: *mut c_void) -> CbRightDownEvent { CbRightDownEvent { ptr: ptr } }
    pub fn null() -> CbRightDownEvent { CbRightDownEvent::from(0 as *mut c_void) }
    
}

pub trait TCbRightDownEvent : TCbPluginEvent {
}

pub struct CbRightUpEvent { ptr: *mut c_void }
impl TCbRightUpEvent for CbRightUpEvent {}
impl TCbPluginEvent for CbRightUpEvent {}
impl TEvent for CbRightUpEvent {}
impl TObject for CbRightUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRightUpEvent {
    pub fn from(ptr: *mut c_void) -> CbRightUpEvent { CbRightUpEvent { ptr: ptr } }
    pub fn null() -> CbRightUpEvent { CbRightUpEvent::from(0 as *mut c_void) }
    
}

pub trait TCbRightUpEvent : TCbPluginEvent {
}

pub struct CbRowDragPlugin { ptr: *mut c_void }
impl TCbRowDragPlugin for CbRowDragPlugin {}
impl TCbPluginBase for CbRowDragPlugin {}
impl TEvtHandler for CbRowDragPlugin {}
impl TObject for CbRowDragPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRowDragPlugin {
    pub fn from(ptr: *mut c_void) -> CbRowDragPlugin { CbRowDragPlugin { ptr: ptr } }
    pub fn null() -> CbRowDragPlugin { CbRowDragPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbRowDragPlugin : TCbPluginBase {
}

pub struct CbRowInfo { ptr: *mut c_void }
impl TCbRowInfo for CbRowInfo {}
impl TObject for CbRowInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRowInfo {
    pub fn from(ptr: *mut c_void) -> CbRowInfo { CbRowInfo { ptr: ptr } }
    pub fn null() -> CbRowInfo { CbRowInfo::from(0 as *mut c_void) }
    
}

pub trait TCbRowInfo : TObject {
}

pub struct CbRowLayoutPlugin { ptr: *mut c_void }
impl TCbRowLayoutPlugin for CbRowLayoutPlugin {}
impl TCbPluginBase for CbRowLayoutPlugin {}
impl TEvtHandler for CbRowLayoutPlugin {}
impl TObject for CbRowLayoutPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRowLayoutPlugin {
    pub fn from(ptr: *mut c_void) -> CbRowLayoutPlugin { CbRowLayoutPlugin { ptr: ptr } }
    pub fn null() -> CbRowLayoutPlugin { CbRowLayoutPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbRowLayoutPlugin : TCbPluginBase {
}

pub struct CbSimpleCustomizationPlugin { ptr: *mut c_void }
impl TCbSimpleCustomizationPlugin for CbSimpleCustomizationPlugin {}
impl TCbPluginBase for CbSimpleCustomizationPlugin {}
impl TEvtHandler for CbSimpleCustomizationPlugin {}
impl TObject for CbSimpleCustomizationPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbSimpleCustomizationPlugin {
    pub fn from(ptr: *mut c_void) -> CbSimpleCustomizationPlugin { CbSimpleCustomizationPlugin { ptr: ptr } }
    pub fn null() -> CbSimpleCustomizationPlugin { CbSimpleCustomizationPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbSimpleCustomizationPlugin : TCbPluginBase {
}

pub struct CbSimpleUpdatesMgr { ptr: *mut c_void }
impl TCbSimpleUpdatesMgr for CbSimpleUpdatesMgr {}
impl TCbUpdatesManagerBase for CbSimpleUpdatesMgr {}
impl TObject for CbSimpleUpdatesMgr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbSimpleUpdatesMgr {
    pub fn from(ptr: *mut c_void) -> CbSimpleUpdatesMgr { CbSimpleUpdatesMgr { ptr: ptr } }
    pub fn null() -> CbSimpleUpdatesMgr { CbSimpleUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait TCbSimpleUpdatesMgr : TCbUpdatesManagerBase {
}

pub struct CbSizeBarWndEvent { ptr: *mut c_void }
impl TCbSizeBarWndEvent for CbSizeBarWndEvent {}
impl TCbPluginEvent for CbSizeBarWndEvent {}
impl TEvent for CbSizeBarWndEvent {}
impl TObject for CbSizeBarWndEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbSizeBarWndEvent {
    pub fn from(ptr: *mut c_void) -> CbSizeBarWndEvent { CbSizeBarWndEvent { ptr: ptr } }
    pub fn null() -> CbSizeBarWndEvent { CbSizeBarWndEvent::from(0 as *mut c_void) }
    
}

pub trait TCbSizeBarWndEvent : TCbPluginEvent {
}

pub struct CbStartBarDraggingEvent { ptr: *mut c_void }
impl TCbStartBarDraggingEvent for CbStartBarDraggingEvent {}
impl TCbPluginEvent for CbStartBarDraggingEvent {}
impl TEvent for CbStartBarDraggingEvent {}
impl TObject for CbStartBarDraggingEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbStartBarDraggingEvent {
    pub fn from(ptr: *mut c_void) -> CbStartBarDraggingEvent { CbStartBarDraggingEvent { ptr: ptr } }
    pub fn null() -> CbStartBarDraggingEvent { CbStartBarDraggingEvent::from(0 as *mut c_void) }
    
}

pub trait TCbStartBarDraggingEvent : TCbPluginEvent {
}

pub struct CbStartDrawInAreaEvent { ptr: *mut c_void }
impl TCbStartDrawInAreaEvent for CbStartDrawInAreaEvent {}
impl TCbPluginEvent for CbStartDrawInAreaEvent {}
impl TEvent for CbStartDrawInAreaEvent {}
impl TObject for CbStartDrawInAreaEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbStartDrawInAreaEvent {
    pub fn from(ptr: *mut c_void) -> CbStartDrawInAreaEvent { CbStartDrawInAreaEvent { ptr: ptr } }
    pub fn null() -> CbStartDrawInAreaEvent { CbStartDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait TCbStartDrawInAreaEvent : TCbPluginEvent {
}

pub struct CbUpdatesManagerBase { ptr: *mut c_void }
impl TCbUpdatesManagerBase for CbUpdatesManagerBase {}
impl TObject for CbUpdatesManagerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbUpdatesManagerBase {
    pub fn from(ptr: *mut c_void) -> CbUpdatesManagerBase { CbUpdatesManagerBase { ptr: ptr } }
    pub fn null() -> CbUpdatesManagerBase { CbUpdatesManagerBase::from(0 as *mut c_void) }
    
}

pub trait TCbUpdatesManagerBase : TObject {
}

