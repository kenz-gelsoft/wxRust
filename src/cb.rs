use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use _unavailable::*;

pub struct CbAntiflickerPlugin { ptr: *mut c_void }
impl TCbAntiflickerPlugin for CbAntiflickerPlugin {}
impl TCbPluginBase for CbAntiflickerPlugin {}
impl TWxEvtHandler for CbAntiflickerPlugin {}
impl TWxObject for CbAntiflickerPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbAntiflickerPlugin {
    pub fn from(ptr: *mut c_void) -> CbAntiflickerPlugin { CbAntiflickerPlugin { ptr: ptr } }
    pub fn null() -> CbAntiflickerPlugin { CbAntiflickerPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbAntiflickerPlugin : TCbPluginBase {
}

pub struct CbBarDragPlugin { ptr: *mut c_void }
impl TCbBarDragPlugin for CbBarDragPlugin {}
impl TCbPluginBase for CbBarDragPlugin {}
impl TWxEvtHandler for CbBarDragPlugin {}
impl TWxObject for CbBarDragPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarDragPlugin {
    pub fn from(ptr: *mut c_void) -> CbBarDragPlugin { CbBarDragPlugin { ptr: ptr } }
    pub fn null() -> CbBarDragPlugin { CbBarDragPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbBarDragPlugin : TCbPluginBase {
}

pub struct CbBarHintsPlugin { ptr: *mut c_void }
impl TCbBarHintsPlugin for CbBarHintsPlugin {}
impl TCbPluginBase for CbBarHintsPlugin {}
impl TWxEvtHandler for CbBarHintsPlugin {}
impl TWxObject for CbBarHintsPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarHintsPlugin {
    pub fn from(ptr: *mut c_void) -> CbBarHintsPlugin { CbBarHintsPlugin { ptr: ptr } }
    pub fn null() -> CbBarHintsPlugin { CbBarHintsPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbBarHintsPlugin : TCbPluginBase {
}

pub struct CbBarInfo { ptr: *mut c_void }
impl TCbBarInfo for CbBarInfo {}
impl TWxObject for CbBarInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarInfo {
    pub fn from(ptr: *mut c_void) -> CbBarInfo { CbBarInfo { ptr: ptr } }
    pub fn null() -> CbBarInfo { CbBarInfo::from(0 as *mut c_void) }
    
}

pub trait TCbBarInfo : TWxObject {
}

pub struct CbBarSpy { ptr: *mut c_void }
impl TCbBarSpy for CbBarSpy {}
impl TWxEvtHandler for CbBarSpy {}
impl TWxObject for CbBarSpy { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarSpy {
    pub fn from(ptr: *mut c_void) -> CbBarSpy { CbBarSpy { ptr: ptr } }
    pub fn null() -> CbBarSpy { CbBarSpy::from(0 as *mut c_void) }
    
}

pub trait TCbBarSpy : TWxEvtHandler {
}

pub struct CbCloseBox { ptr: *mut c_void }
impl TCbCloseBox for CbCloseBox {}
impl TCbMiniButton for CbCloseBox {}
impl TWxObject for CbCloseBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCloseBox {
    pub fn from(ptr: *mut c_void) -> CbCloseBox { CbCloseBox { ptr: ptr } }
    pub fn null() -> CbCloseBox { CbCloseBox::from(0 as *mut c_void) }
    
}

pub trait TCbCloseBox : TCbMiniButton {
}

pub struct CbCollapseBox { ptr: *mut c_void }
impl TCbCollapseBox for CbCollapseBox {}
impl TCbMiniButton for CbCollapseBox {}
impl TWxObject for CbCollapseBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCollapseBox {
    pub fn from(ptr: *mut c_void) -> CbCollapseBox { CbCollapseBox { ptr: ptr } }
    pub fn null() -> CbCollapseBox { CbCollapseBox::from(0 as *mut c_void) }
    
}

pub trait TCbCollapseBox : TCbMiniButton {
}

pub struct CbCommonPaneProperties { ptr: *mut c_void }
impl TCbCommonPaneProperties for CbCommonPaneProperties {}
impl TWxObject for CbCommonPaneProperties { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCommonPaneProperties {
    pub fn from(ptr: *mut c_void) -> CbCommonPaneProperties { CbCommonPaneProperties { ptr: ptr } }
    pub fn null() -> CbCommonPaneProperties { CbCommonPaneProperties::from(0 as *mut c_void) }
    
}

pub trait TCbCommonPaneProperties : TWxObject {
}

pub struct CbCustomizeBarEvent { ptr: *mut c_void }
impl TCbCustomizeBarEvent for CbCustomizeBarEvent {}
impl TCbPluginEvent for CbCustomizeBarEvent {}
impl TWxEvent for CbCustomizeBarEvent {}
impl TWxObject for CbCustomizeBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCustomizeBarEvent {
    pub fn from(ptr: *mut c_void) -> CbCustomizeBarEvent { CbCustomizeBarEvent { ptr: ptr } }
    pub fn null() -> CbCustomizeBarEvent { CbCustomizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait TCbCustomizeBarEvent : TCbPluginEvent {
}

pub struct CbCustomizeLayoutEvent { ptr: *mut c_void }
impl TCbCustomizeLayoutEvent for CbCustomizeLayoutEvent {}
impl TCbPluginEvent for CbCustomizeLayoutEvent {}
impl TWxEvent for CbCustomizeLayoutEvent {}
impl TWxObject for CbCustomizeLayoutEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCustomizeLayoutEvent {
    pub fn from(ptr: *mut c_void) -> CbCustomizeLayoutEvent { CbCustomizeLayoutEvent { ptr: ptr } }
    pub fn null() -> CbCustomizeLayoutEvent { CbCustomizeLayoutEvent::from(0 as *mut c_void) }
    
}

pub trait TCbCustomizeLayoutEvent : TCbPluginEvent {
}

pub struct CbDimHandlerBase { ptr: *mut c_void }
impl TCbDimHandlerBase for CbDimHandlerBase {}
impl TWxObject for CbDimHandlerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDimHandlerBase {
    pub fn from(ptr: *mut c_void) -> CbDimHandlerBase { CbDimHandlerBase { ptr: ptr } }
    pub fn null() -> CbDimHandlerBase { CbDimHandlerBase::from(0 as *mut c_void) }
    
}

pub trait TCbDimHandlerBase : TWxObject {
}

pub struct CbDimInfo { ptr: *mut c_void }
impl TCbDimInfo for CbDimInfo {}
impl TWxObject for CbDimInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDimInfo {
    pub fn from(ptr: *mut c_void) -> CbDimInfo { CbDimInfo { ptr: ptr } }
    pub fn null() -> CbDimInfo { CbDimInfo::from(0 as *mut c_void) }
    
}

pub trait TCbDimInfo : TWxObject {
}

pub struct CbDockBox { ptr: *mut c_void }
impl TCbDockBox for CbDockBox {}
impl TCbMiniButton for CbDockBox {}
impl TWxObject for CbDockBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDockBox {
    pub fn from(ptr: *mut c_void) -> CbDockBox { CbDockBox { ptr: ptr } }
    pub fn null() -> CbDockBox { CbDockBox::from(0 as *mut c_void) }
    
}

pub trait TCbDockBox : TCbMiniButton {
}

pub struct CbDockPane { ptr: *mut c_void }
impl TCbDockPane for CbDockPane {}
impl TWxObject for CbDockPane { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDockPane {
    pub fn from(ptr: *mut c_void) -> CbDockPane { CbDockPane { ptr: ptr } }
    pub fn null() -> CbDockPane { CbDockPane::from(0 as *mut c_void) }
    
}

pub trait TCbDockPane : TWxObject {
}

pub struct CbDrawBarDecorEvent { ptr: *mut c_void }
impl TCbDrawBarDecorEvent for CbDrawBarDecorEvent {}
impl TCbPluginEvent for CbDrawBarDecorEvent {}
impl TWxEvent for CbDrawBarDecorEvent {}
impl TWxObject for CbDrawBarDecorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawBarDecorEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawBarDecorEvent { CbDrawBarDecorEvent { ptr: ptr } }
    pub fn null() -> CbDrawBarDecorEvent { CbDrawBarDecorEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawBarDecorEvent : TCbPluginEvent {
}

pub struct CbDrawBarHandlesEvent { ptr: *mut c_void }
impl TCbDrawBarHandlesEvent for CbDrawBarHandlesEvent {}
impl TCbPluginEvent for CbDrawBarHandlesEvent {}
impl TWxEvent for CbDrawBarHandlesEvent {}
impl TWxObject for CbDrawBarHandlesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawBarHandlesEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawBarHandlesEvent { CbDrawBarHandlesEvent { ptr: ptr } }
    pub fn null() -> CbDrawBarHandlesEvent { CbDrawBarHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawBarHandlesEvent : TCbPluginEvent {
}

pub struct CbDrawHintRectEvent { ptr: *mut c_void }
impl TCbDrawHintRectEvent for CbDrawHintRectEvent {}
impl TCbPluginEvent for CbDrawHintRectEvent {}
impl TWxEvent for CbDrawHintRectEvent {}
impl TWxObject for CbDrawHintRectEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawHintRectEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawHintRectEvent { CbDrawHintRectEvent { ptr: ptr } }
    pub fn null() -> CbDrawHintRectEvent { CbDrawHintRectEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawHintRectEvent : TCbPluginEvent {
}

pub struct CbDrawPaneBkGroundEvent { ptr: *mut c_void }
impl TCbDrawPaneBkGroundEvent for CbDrawPaneBkGroundEvent {}
impl TCbPluginEvent for CbDrawPaneBkGroundEvent {}
impl TWxEvent for CbDrawPaneBkGroundEvent {}
impl TWxObject for CbDrawPaneBkGroundEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawPaneBkGroundEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawPaneBkGroundEvent { CbDrawPaneBkGroundEvent { ptr: ptr } }
    pub fn null() -> CbDrawPaneBkGroundEvent { CbDrawPaneBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawPaneBkGroundEvent : TCbPluginEvent {
}

pub struct CbDrawPaneDecorEvent { ptr: *mut c_void }
impl TCbDrawPaneDecorEvent for CbDrawPaneDecorEvent {}
impl TCbPluginEvent for CbDrawPaneDecorEvent {}
impl TWxEvent for CbDrawPaneDecorEvent {}
impl TWxObject for CbDrawPaneDecorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawPaneDecorEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawPaneDecorEvent { CbDrawPaneDecorEvent { ptr: ptr } }
    pub fn null() -> CbDrawPaneDecorEvent { CbDrawPaneDecorEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawPaneDecorEvent : TCbPluginEvent {
}

pub struct CbDrawRowBkGroundEvent { ptr: *mut c_void }
impl TCbDrawRowBkGroundEvent for CbDrawRowBkGroundEvent {}
impl TCbPluginEvent for CbDrawRowBkGroundEvent {}
impl TWxEvent for CbDrawRowBkGroundEvent {}
impl TWxObject for CbDrawRowBkGroundEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawRowBkGroundEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawRowBkGroundEvent { CbDrawRowBkGroundEvent { ptr: ptr } }
    pub fn null() -> CbDrawRowBkGroundEvent { CbDrawRowBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawRowBkGroundEvent : TCbPluginEvent {
}

pub struct CbDrawRowDecorEvent { ptr: *mut c_void }
impl TCbDrawRowDecorEvent for CbDrawRowDecorEvent {}
impl TCbPluginEvent for CbDrawRowDecorEvent {}
impl TWxEvent for CbDrawRowDecorEvent {}
impl TWxObject for CbDrawRowDecorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawRowDecorEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawRowDecorEvent { CbDrawRowDecorEvent { ptr: ptr } }
    pub fn null() -> CbDrawRowDecorEvent { CbDrawRowDecorEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawRowDecorEvent : TCbPluginEvent {
}

pub struct CbDrawRowHandlesEvent { ptr: *mut c_void }
impl TCbDrawRowHandlesEvent for CbDrawRowHandlesEvent {}
impl TCbPluginEvent for CbDrawRowHandlesEvent {}
impl TWxEvent for CbDrawRowHandlesEvent {}
impl TWxObject for CbDrawRowHandlesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawRowHandlesEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawRowHandlesEvent { CbDrawRowHandlesEvent { ptr: ptr } }
    pub fn null() -> CbDrawRowHandlesEvent { CbDrawRowHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait TCbDrawRowHandlesEvent : TCbPluginEvent {
}

pub struct CbDynToolBarDimHandler { ptr: *mut c_void }
impl TCbDynToolBarDimHandler for CbDynToolBarDimHandler {}
impl TCbDimHandlerBase for CbDynToolBarDimHandler {}
impl TWxObject for CbDynToolBarDimHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDynToolBarDimHandler {
    pub fn from(ptr: *mut c_void) -> CbDynToolBarDimHandler { CbDynToolBarDimHandler { ptr: ptr } }
    pub fn null() -> CbDynToolBarDimHandler { CbDynToolBarDimHandler::from(0 as *mut c_void) }
    
}

pub trait TCbDynToolBarDimHandler : TCbDimHandlerBase {
}

pub struct CbFinishDrawInAreaEvent { ptr: *mut c_void }
impl TCbFinishDrawInAreaEvent for CbFinishDrawInAreaEvent {}
impl TCbPluginEvent for CbFinishDrawInAreaEvent {}
impl TWxEvent for CbFinishDrawInAreaEvent {}
impl TWxObject for CbFinishDrawInAreaEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbFinishDrawInAreaEvent {
    pub fn from(ptr: *mut c_void) -> CbFinishDrawInAreaEvent { CbFinishDrawInAreaEvent { ptr: ptr } }
    pub fn null() -> CbFinishDrawInAreaEvent { CbFinishDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait TCbFinishDrawInAreaEvent : TCbPluginEvent {
}

pub struct CbFloatedBarWindow { ptr: *mut c_void }
impl TCbFloatedBarWindow for CbFloatedBarWindow {}
impl TWxToolWindow for CbFloatedBarWindow {}
impl TWxFrame for CbFloatedBarWindow {}
impl TWxTopLevelWindow for CbFloatedBarWindow {}
impl TWxWindow for CbFloatedBarWindow {}
impl TWxEvtHandler for CbFloatedBarWindow {}
impl TWxObject for CbFloatedBarWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbFloatedBarWindow {
    pub fn from(ptr: *mut c_void) -> CbFloatedBarWindow { CbFloatedBarWindow { ptr: ptr } }
    pub fn null() -> CbFloatedBarWindow { CbFloatedBarWindow::from(0 as *mut c_void) }
    
}

pub trait TCbFloatedBarWindow : TWxToolWindow {
}

pub struct CbGCUpdatesMgr { ptr: *mut c_void }
impl TCbGCUpdatesMgr for CbGCUpdatesMgr {}
impl TCbSimpleUpdatesMgr for CbGCUpdatesMgr {}
impl TCbUpdatesManagerBase for CbGCUpdatesMgr {}
impl TWxObject for CbGCUpdatesMgr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbGCUpdatesMgr {
    pub fn from(ptr: *mut c_void) -> CbGCUpdatesMgr { CbGCUpdatesMgr { ptr: ptr } }
    pub fn null() -> CbGCUpdatesMgr { CbGCUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait TCbGCUpdatesMgr : TCbSimpleUpdatesMgr {
}

pub struct CbHintAnimationPlugin { ptr: *mut c_void }
impl TCbHintAnimationPlugin for CbHintAnimationPlugin {}
impl TCbPluginBase for CbHintAnimationPlugin {}
impl TWxEvtHandler for CbHintAnimationPlugin {}
impl TWxObject for CbHintAnimationPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbHintAnimationPlugin {
    pub fn from(ptr: *mut c_void) -> CbHintAnimationPlugin { CbHintAnimationPlugin { ptr: ptr } }
    pub fn null() -> CbHintAnimationPlugin { CbHintAnimationPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbHintAnimationPlugin : TCbPluginBase {
}

pub struct CbInsertBarEvent { ptr: *mut c_void }
impl TCbInsertBarEvent for CbInsertBarEvent {}
impl TCbPluginEvent for CbInsertBarEvent {}
impl TWxEvent for CbInsertBarEvent {}
impl TWxObject for CbInsertBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbInsertBarEvent {
    pub fn from(ptr: *mut c_void) -> CbInsertBarEvent { CbInsertBarEvent { ptr: ptr } }
    pub fn null() -> CbInsertBarEvent { CbInsertBarEvent::from(0 as *mut c_void) }
    
}

pub trait TCbInsertBarEvent : TCbPluginEvent {
}

pub struct CbLayoutRowEvent { ptr: *mut c_void }
impl TCbLayoutRowEvent for CbLayoutRowEvent {}
impl TCbPluginEvent for CbLayoutRowEvent {}
impl TWxEvent for CbLayoutRowEvent {}
impl TWxObject for CbLayoutRowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLayoutRowEvent {
    pub fn from(ptr: *mut c_void) -> CbLayoutRowEvent { CbLayoutRowEvent { ptr: ptr } }
    pub fn null() -> CbLayoutRowEvent { CbLayoutRowEvent::from(0 as *mut c_void) }
    
}

pub trait TCbLayoutRowEvent : TCbPluginEvent {
}

pub struct CbLeftDClickEvent { ptr: *mut c_void }
impl TCbLeftDClickEvent for CbLeftDClickEvent {}
impl TCbPluginEvent for CbLeftDClickEvent {}
impl TWxEvent for CbLeftDClickEvent {}
impl TWxObject for CbLeftDClickEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLeftDClickEvent {
    pub fn from(ptr: *mut c_void) -> CbLeftDClickEvent { CbLeftDClickEvent { ptr: ptr } }
    pub fn null() -> CbLeftDClickEvent { CbLeftDClickEvent::from(0 as *mut c_void) }
    
}

pub trait TCbLeftDClickEvent : TCbPluginEvent {
}

pub struct CbLeftDownEvent { ptr: *mut c_void }
impl TCbLeftDownEvent for CbLeftDownEvent {}
impl TCbPluginEvent for CbLeftDownEvent {}
impl TWxEvent for CbLeftDownEvent {}
impl TWxObject for CbLeftDownEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLeftDownEvent {
    pub fn from(ptr: *mut c_void) -> CbLeftDownEvent { CbLeftDownEvent { ptr: ptr } }
    pub fn null() -> CbLeftDownEvent { CbLeftDownEvent::from(0 as *mut c_void) }
    
}

pub trait TCbLeftDownEvent : TCbPluginEvent {
}

pub struct CbLeftUpEvent { ptr: *mut c_void }
impl TCbLeftUpEvent for CbLeftUpEvent {}
impl TCbPluginEvent for CbLeftUpEvent {}
impl TWxEvent for CbLeftUpEvent {}
impl TWxObject for CbLeftUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLeftUpEvent {
    pub fn from(ptr: *mut c_void) -> CbLeftUpEvent { CbLeftUpEvent { ptr: ptr } }
    pub fn null() -> CbLeftUpEvent { CbLeftUpEvent::from(0 as *mut c_void) }
    
}

pub trait TCbLeftUpEvent : TCbPluginEvent {
}

pub struct CbMiniButton { ptr: *mut c_void }
impl TCbMiniButton for CbMiniButton {}
impl TWxObject for CbMiniButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbMiniButton {
    pub fn from(ptr: *mut c_void) -> CbMiniButton { CbMiniButton { ptr: ptr } }
    pub fn null() -> CbMiniButton { CbMiniButton::from(0 as *mut c_void) }
    
}

pub trait TCbMiniButton : TWxObject {
}

pub struct CbMotionEvent { ptr: *mut c_void }
impl TCbMotionEvent for CbMotionEvent {}
impl TCbPluginEvent for CbMotionEvent {}
impl TWxEvent for CbMotionEvent {}
impl TWxObject for CbMotionEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbMotionEvent {
    pub fn from(ptr: *mut c_void) -> CbMotionEvent { CbMotionEvent { ptr: ptr } }
    pub fn null() -> CbMotionEvent { CbMotionEvent::from(0 as *mut c_void) }
    
}

pub trait TCbMotionEvent : TCbPluginEvent {
}

pub struct CbPaneDrawPlugin { ptr: *mut c_void }
impl TCbPaneDrawPlugin for CbPaneDrawPlugin {}
impl TCbPluginBase for CbPaneDrawPlugin {}
impl TWxEvtHandler for CbPaneDrawPlugin {}
impl TWxObject for CbPaneDrawPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbPaneDrawPlugin {
    pub fn from(ptr: *mut c_void) -> CbPaneDrawPlugin { CbPaneDrawPlugin { ptr: ptr } }
    pub fn null() -> CbPaneDrawPlugin { CbPaneDrawPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbPaneDrawPlugin : TCbPluginBase {
}

pub struct CbPluginBase { ptr: *mut c_void }
impl TCbPluginBase for CbPluginBase {}
impl TWxEvtHandler for CbPluginBase {}
impl TWxObject for CbPluginBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbPluginBase {
    pub fn from(ptr: *mut c_void) -> CbPluginBase { CbPluginBase { ptr: ptr } }
    pub fn null() -> CbPluginBase { CbPluginBase::from(0 as *mut c_void) }
    
}

pub trait TCbPluginBase : TWxEvtHandler {
}

pub struct CbPluginEvent { ptr: *mut c_void }
impl TCbPluginEvent for CbPluginEvent {}
impl TWxEvent for CbPluginEvent {}
impl TWxObject for CbPluginEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbPluginEvent {
    pub fn from(ptr: *mut c_void) -> CbPluginEvent { CbPluginEvent { ptr: ptr } }
    pub fn null() -> CbPluginEvent { CbPluginEvent::from(0 as *mut c_void) }
    
}

pub trait TCbPluginEvent : TWxEvent {
}

pub struct CbRemoveBarEvent { ptr: *mut c_void }
impl TCbRemoveBarEvent for CbRemoveBarEvent {}
impl TCbPluginEvent for CbRemoveBarEvent {}
impl TWxEvent for CbRemoveBarEvent {}
impl TWxObject for CbRemoveBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRemoveBarEvent {
    pub fn from(ptr: *mut c_void) -> CbRemoveBarEvent { CbRemoveBarEvent { ptr: ptr } }
    pub fn null() -> CbRemoveBarEvent { CbRemoveBarEvent::from(0 as *mut c_void) }
    
}

pub trait TCbRemoveBarEvent : TCbPluginEvent {
}

pub struct CbResizeBarEvent { ptr: *mut c_void }
impl TCbResizeBarEvent for CbResizeBarEvent {}
impl TCbPluginEvent for CbResizeBarEvent {}
impl TWxEvent for CbResizeBarEvent {}
impl TWxObject for CbResizeBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbResizeBarEvent {
    pub fn from(ptr: *mut c_void) -> CbResizeBarEvent { CbResizeBarEvent { ptr: ptr } }
    pub fn null() -> CbResizeBarEvent { CbResizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait TCbResizeBarEvent : TCbPluginEvent {
}

pub struct CbResizeRowEvent { ptr: *mut c_void }
impl TCbResizeRowEvent for CbResizeRowEvent {}
impl TCbPluginEvent for CbResizeRowEvent {}
impl TWxEvent for CbResizeRowEvent {}
impl TWxObject for CbResizeRowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbResizeRowEvent {
    pub fn from(ptr: *mut c_void) -> CbResizeRowEvent { CbResizeRowEvent { ptr: ptr } }
    pub fn null() -> CbResizeRowEvent { CbResizeRowEvent::from(0 as *mut c_void) }
    
}

pub trait TCbResizeRowEvent : TCbPluginEvent {
}

pub struct CbRightDownEvent { ptr: *mut c_void }
impl TCbRightDownEvent for CbRightDownEvent {}
impl TCbPluginEvent for CbRightDownEvent {}
impl TWxEvent for CbRightDownEvent {}
impl TWxObject for CbRightDownEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRightDownEvent {
    pub fn from(ptr: *mut c_void) -> CbRightDownEvent { CbRightDownEvent { ptr: ptr } }
    pub fn null() -> CbRightDownEvent { CbRightDownEvent::from(0 as *mut c_void) }
    
}

pub trait TCbRightDownEvent : TCbPluginEvent {
}

pub struct CbRightUpEvent { ptr: *mut c_void }
impl TCbRightUpEvent for CbRightUpEvent {}
impl TCbPluginEvent for CbRightUpEvent {}
impl TWxEvent for CbRightUpEvent {}
impl TWxObject for CbRightUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRightUpEvent {
    pub fn from(ptr: *mut c_void) -> CbRightUpEvent { CbRightUpEvent { ptr: ptr } }
    pub fn null() -> CbRightUpEvent { CbRightUpEvent::from(0 as *mut c_void) }
    
}

pub trait TCbRightUpEvent : TCbPluginEvent {
}

pub struct CbRowDragPlugin { ptr: *mut c_void }
impl TCbRowDragPlugin for CbRowDragPlugin {}
impl TCbPluginBase for CbRowDragPlugin {}
impl TWxEvtHandler for CbRowDragPlugin {}
impl TWxObject for CbRowDragPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRowDragPlugin {
    pub fn from(ptr: *mut c_void) -> CbRowDragPlugin { CbRowDragPlugin { ptr: ptr } }
    pub fn null() -> CbRowDragPlugin { CbRowDragPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbRowDragPlugin : TCbPluginBase {
}

pub struct CbRowInfo { ptr: *mut c_void }
impl TCbRowInfo for CbRowInfo {}
impl TWxObject for CbRowInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRowInfo {
    pub fn from(ptr: *mut c_void) -> CbRowInfo { CbRowInfo { ptr: ptr } }
    pub fn null() -> CbRowInfo { CbRowInfo::from(0 as *mut c_void) }
    
}

pub trait TCbRowInfo : TWxObject {
}

pub struct CbRowLayoutPlugin { ptr: *mut c_void }
impl TCbRowLayoutPlugin for CbRowLayoutPlugin {}
impl TCbPluginBase for CbRowLayoutPlugin {}
impl TWxEvtHandler for CbRowLayoutPlugin {}
impl TWxObject for CbRowLayoutPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRowLayoutPlugin {
    pub fn from(ptr: *mut c_void) -> CbRowLayoutPlugin { CbRowLayoutPlugin { ptr: ptr } }
    pub fn null() -> CbRowLayoutPlugin { CbRowLayoutPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbRowLayoutPlugin : TCbPluginBase {
}

pub struct CbSimpleCustomizationPlugin { ptr: *mut c_void }
impl TCbSimpleCustomizationPlugin for CbSimpleCustomizationPlugin {}
impl TCbPluginBase for CbSimpleCustomizationPlugin {}
impl TWxEvtHandler for CbSimpleCustomizationPlugin {}
impl TWxObject for CbSimpleCustomizationPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbSimpleCustomizationPlugin {
    pub fn from(ptr: *mut c_void) -> CbSimpleCustomizationPlugin { CbSimpleCustomizationPlugin { ptr: ptr } }
    pub fn null() -> CbSimpleCustomizationPlugin { CbSimpleCustomizationPlugin::from(0 as *mut c_void) }
    
}

pub trait TCbSimpleCustomizationPlugin : TCbPluginBase {
}

pub struct CbSimpleUpdatesMgr { ptr: *mut c_void }
impl TCbSimpleUpdatesMgr for CbSimpleUpdatesMgr {}
impl TCbUpdatesManagerBase for CbSimpleUpdatesMgr {}
impl TWxObject for CbSimpleUpdatesMgr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbSimpleUpdatesMgr {
    pub fn from(ptr: *mut c_void) -> CbSimpleUpdatesMgr { CbSimpleUpdatesMgr { ptr: ptr } }
    pub fn null() -> CbSimpleUpdatesMgr { CbSimpleUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait TCbSimpleUpdatesMgr : TCbUpdatesManagerBase {
}

pub struct CbSizeBarWndEvent { ptr: *mut c_void }
impl TCbSizeBarWndEvent for CbSizeBarWndEvent {}
impl TCbPluginEvent for CbSizeBarWndEvent {}
impl TWxEvent for CbSizeBarWndEvent {}
impl TWxObject for CbSizeBarWndEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbSizeBarWndEvent {
    pub fn from(ptr: *mut c_void) -> CbSizeBarWndEvent { CbSizeBarWndEvent { ptr: ptr } }
    pub fn null() -> CbSizeBarWndEvent { CbSizeBarWndEvent::from(0 as *mut c_void) }
    
}

pub trait TCbSizeBarWndEvent : TCbPluginEvent {
}

pub struct CbStartBarDraggingEvent { ptr: *mut c_void }
impl TCbStartBarDraggingEvent for CbStartBarDraggingEvent {}
impl TCbPluginEvent for CbStartBarDraggingEvent {}
impl TWxEvent for CbStartBarDraggingEvent {}
impl TWxObject for CbStartBarDraggingEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbStartBarDraggingEvent {
    pub fn from(ptr: *mut c_void) -> CbStartBarDraggingEvent { CbStartBarDraggingEvent { ptr: ptr } }
    pub fn null() -> CbStartBarDraggingEvent { CbStartBarDraggingEvent::from(0 as *mut c_void) }
    
}

pub trait TCbStartBarDraggingEvent : TCbPluginEvent {
}

pub struct CbStartDrawInAreaEvent { ptr: *mut c_void }
impl TCbStartDrawInAreaEvent for CbStartDrawInAreaEvent {}
impl TCbPluginEvent for CbStartDrawInAreaEvent {}
impl TWxEvent for CbStartDrawInAreaEvent {}
impl TWxObject for CbStartDrawInAreaEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbStartDrawInAreaEvent {
    pub fn from(ptr: *mut c_void) -> CbStartDrawInAreaEvent { CbStartDrawInAreaEvent { ptr: ptr } }
    pub fn null() -> CbStartDrawInAreaEvent { CbStartDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait TCbStartDrawInAreaEvent : TCbPluginEvent {
}

pub struct CbUpdatesManagerBase { ptr: *mut c_void }
impl TCbUpdatesManagerBase for CbUpdatesManagerBase {}
impl TWxObject for CbUpdatesManagerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbUpdatesManagerBase {
    pub fn from(ptr: *mut c_void) -> CbUpdatesManagerBase { CbUpdatesManagerBase { ptr: ptr } }
    pub fn null() -> CbUpdatesManagerBase { CbUpdatesManagerBase::from(0 as *mut c_void) }
    
}

pub trait TCbUpdatesManagerBase : TWxObject {
}

