use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use _unavailable::*;

pub struct CbAntiflickerPlugin { ptr: *mut c_void }
impl CbAntiflickerPluginMethods for CbAntiflickerPlugin {}
impl CbPluginBaseMethods for CbAntiflickerPlugin {}
impl EvtHandlerMethods for CbAntiflickerPlugin {}
impl ObjectMethods for CbAntiflickerPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbAntiflickerPlugin {
    pub fn from(ptr: *mut c_void) -> CbAntiflickerPlugin { CbAntiflickerPlugin { ptr: ptr } }
    pub fn null() -> CbAntiflickerPlugin { CbAntiflickerPlugin::from(0 as *mut c_void) }
    
}

pub trait CbAntiflickerPluginMethods : CbPluginBaseMethods {
}

pub struct CbBarDragPlugin { ptr: *mut c_void }
impl CbBarDragPluginMethods for CbBarDragPlugin {}
impl CbPluginBaseMethods for CbBarDragPlugin {}
impl EvtHandlerMethods for CbBarDragPlugin {}
impl ObjectMethods for CbBarDragPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarDragPlugin {
    pub fn from(ptr: *mut c_void) -> CbBarDragPlugin { CbBarDragPlugin { ptr: ptr } }
    pub fn null() -> CbBarDragPlugin { CbBarDragPlugin::from(0 as *mut c_void) }
    
}

pub trait CbBarDragPluginMethods : CbPluginBaseMethods {
}

pub struct CbBarHintsPlugin { ptr: *mut c_void }
impl CbBarHintsPluginMethods for CbBarHintsPlugin {}
impl CbPluginBaseMethods for CbBarHintsPlugin {}
impl EvtHandlerMethods for CbBarHintsPlugin {}
impl ObjectMethods for CbBarHintsPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarHintsPlugin {
    pub fn from(ptr: *mut c_void) -> CbBarHintsPlugin { CbBarHintsPlugin { ptr: ptr } }
    pub fn null() -> CbBarHintsPlugin { CbBarHintsPlugin::from(0 as *mut c_void) }
    
}

pub trait CbBarHintsPluginMethods : CbPluginBaseMethods {
}

pub struct CbBarInfo { ptr: *mut c_void }
impl CbBarInfoMethods for CbBarInfo {}
impl ObjectMethods for CbBarInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarInfo {
    pub fn from(ptr: *mut c_void) -> CbBarInfo { CbBarInfo { ptr: ptr } }
    pub fn null() -> CbBarInfo { CbBarInfo::from(0 as *mut c_void) }
    
}

pub trait CbBarInfoMethods : ObjectMethods {
}

pub struct CbBarSpy { ptr: *mut c_void }
impl CbBarSpyMethods for CbBarSpy {}
impl EvtHandlerMethods for CbBarSpy {}
impl ObjectMethods for CbBarSpy { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbBarSpy {
    pub fn from(ptr: *mut c_void) -> CbBarSpy { CbBarSpy { ptr: ptr } }
    pub fn null() -> CbBarSpy { CbBarSpy::from(0 as *mut c_void) }
    
}

pub trait CbBarSpyMethods : EvtHandlerMethods {
}

pub struct CbCloseBox { ptr: *mut c_void }
impl CbCloseBoxMethods for CbCloseBox {}
impl CbMiniButtonMethods for CbCloseBox {}
impl ObjectMethods for CbCloseBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCloseBox {
    pub fn from(ptr: *mut c_void) -> CbCloseBox { CbCloseBox { ptr: ptr } }
    pub fn null() -> CbCloseBox { CbCloseBox::from(0 as *mut c_void) }
    
}

pub trait CbCloseBoxMethods : CbMiniButtonMethods {
}

pub struct CbCollapseBox { ptr: *mut c_void }
impl CbCollapseBoxMethods for CbCollapseBox {}
impl CbMiniButtonMethods for CbCollapseBox {}
impl ObjectMethods for CbCollapseBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCollapseBox {
    pub fn from(ptr: *mut c_void) -> CbCollapseBox { CbCollapseBox { ptr: ptr } }
    pub fn null() -> CbCollapseBox { CbCollapseBox::from(0 as *mut c_void) }
    
}

pub trait CbCollapseBoxMethods : CbMiniButtonMethods {
}

pub struct CbCommonPaneProperties { ptr: *mut c_void }
impl CbCommonPanePropertiesMethods for CbCommonPaneProperties {}
impl ObjectMethods for CbCommonPaneProperties { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCommonPaneProperties {
    pub fn from(ptr: *mut c_void) -> CbCommonPaneProperties { CbCommonPaneProperties { ptr: ptr } }
    pub fn null() -> CbCommonPaneProperties { CbCommonPaneProperties::from(0 as *mut c_void) }
    
}

pub trait CbCommonPanePropertiesMethods : ObjectMethods {
}

pub struct CbCustomizeBarEvent { ptr: *mut c_void }
impl CbCustomizeBarEventMethods for CbCustomizeBarEvent {}
impl CbPluginEventMethods for CbCustomizeBarEvent {}
impl EventMethods for CbCustomizeBarEvent {}
impl ObjectMethods for CbCustomizeBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCustomizeBarEvent {
    pub fn from(ptr: *mut c_void) -> CbCustomizeBarEvent { CbCustomizeBarEvent { ptr: ptr } }
    pub fn null() -> CbCustomizeBarEvent { CbCustomizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait CbCustomizeBarEventMethods : CbPluginEventMethods {
}

pub struct CbCustomizeLayoutEvent { ptr: *mut c_void }
impl CbCustomizeLayoutEventMethods for CbCustomizeLayoutEvent {}
impl CbPluginEventMethods for CbCustomizeLayoutEvent {}
impl EventMethods for CbCustomizeLayoutEvent {}
impl ObjectMethods for CbCustomizeLayoutEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbCustomizeLayoutEvent {
    pub fn from(ptr: *mut c_void) -> CbCustomizeLayoutEvent { CbCustomizeLayoutEvent { ptr: ptr } }
    pub fn null() -> CbCustomizeLayoutEvent { CbCustomizeLayoutEvent::from(0 as *mut c_void) }
    
}

pub trait CbCustomizeLayoutEventMethods : CbPluginEventMethods {
}

pub struct CbDimHandlerBase { ptr: *mut c_void }
impl CbDimHandlerBaseMethods for CbDimHandlerBase {}
impl ObjectMethods for CbDimHandlerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDimHandlerBase {
    pub fn from(ptr: *mut c_void) -> CbDimHandlerBase { CbDimHandlerBase { ptr: ptr } }
    pub fn null() -> CbDimHandlerBase { CbDimHandlerBase::from(0 as *mut c_void) }
    
}

pub trait CbDimHandlerBaseMethods : ObjectMethods {
}

pub struct CbDimInfo { ptr: *mut c_void }
impl CbDimInfoMethods for CbDimInfo {}
impl ObjectMethods for CbDimInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDimInfo {
    pub fn from(ptr: *mut c_void) -> CbDimInfo { CbDimInfo { ptr: ptr } }
    pub fn null() -> CbDimInfo { CbDimInfo::from(0 as *mut c_void) }
    
}

pub trait CbDimInfoMethods : ObjectMethods {
}

pub struct CbDockBox { ptr: *mut c_void }
impl CbDockBoxMethods for CbDockBox {}
impl CbMiniButtonMethods for CbDockBox {}
impl ObjectMethods for CbDockBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDockBox {
    pub fn from(ptr: *mut c_void) -> CbDockBox { CbDockBox { ptr: ptr } }
    pub fn null() -> CbDockBox { CbDockBox::from(0 as *mut c_void) }
    
}

pub trait CbDockBoxMethods : CbMiniButtonMethods {
}

pub struct CbDockPane { ptr: *mut c_void }
impl CbDockPaneMethods for CbDockPane {}
impl ObjectMethods for CbDockPane { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDockPane {
    pub fn from(ptr: *mut c_void) -> CbDockPane { CbDockPane { ptr: ptr } }
    pub fn null() -> CbDockPane { CbDockPane::from(0 as *mut c_void) }
    
}

pub trait CbDockPaneMethods : ObjectMethods {
}

pub struct CbDrawBarDecorEvent { ptr: *mut c_void }
impl CbDrawBarDecorEventMethods for CbDrawBarDecorEvent {}
impl CbPluginEventMethods for CbDrawBarDecorEvent {}
impl EventMethods for CbDrawBarDecorEvent {}
impl ObjectMethods for CbDrawBarDecorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawBarDecorEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawBarDecorEvent { CbDrawBarDecorEvent { ptr: ptr } }
    pub fn null() -> CbDrawBarDecorEvent { CbDrawBarDecorEvent::from(0 as *mut c_void) }
    
}

pub trait CbDrawBarDecorEventMethods : CbPluginEventMethods {
}

pub struct CbDrawBarHandlesEvent { ptr: *mut c_void }
impl CbDrawBarHandlesEventMethods for CbDrawBarHandlesEvent {}
impl CbPluginEventMethods for CbDrawBarHandlesEvent {}
impl EventMethods for CbDrawBarHandlesEvent {}
impl ObjectMethods for CbDrawBarHandlesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawBarHandlesEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawBarHandlesEvent { CbDrawBarHandlesEvent { ptr: ptr } }
    pub fn null() -> CbDrawBarHandlesEvent { CbDrawBarHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait CbDrawBarHandlesEventMethods : CbPluginEventMethods {
}

pub struct CbDrawHintRectEvent { ptr: *mut c_void }
impl CbDrawHintRectEventMethods for CbDrawHintRectEvent {}
impl CbPluginEventMethods for CbDrawHintRectEvent {}
impl EventMethods for CbDrawHintRectEvent {}
impl ObjectMethods for CbDrawHintRectEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawHintRectEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawHintRectEvent { CbDrawHintRectEvent { ptr: ptr } }
    pub fn null() -> CbDrawHintRectEvent { CbDrawHintRectEvent::from(0 as *mut c_void) }
    
}

pub trait CbDrawHintRectEventMethods : CbPluginEventMethods {
}

pub struct CbDrawPaneBkGroundEvent { ptr: *mut c_void }
impl CbDrawPaneBkGroundEventMethods for CbDrawPaneBkGroundEvent {}
impl CbPluginEventMethods for CbDrawPaneBkGroundEvent {}
impl EventMethods for CbDrawPaneBkGroundEvent {}
impl ObjectMethods for CbDrawPaneBkGroundEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawPaneBkGroundEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawPaneBkGroundEvent { CbDrawPaneBkGroundEvent { ptr: ptr } }
    pub fn null() -> CbDrawPaneBkGroundEvent { CbDrawPaneBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait CbDrawPaneBkGroundEventMethods : CbPluginEventMethods {
}

pub struct CbDrawPaneDecorEvent { ptr: *mut c_void }
impl CbDrawPaneDecorEventMethods for CbDrawPaneDecorEvent {}
impl CbPluginEventMethods for CbDrawPaneDecorEvent {}
impl EventMethods for CbDrawPaneDecorEvent {}
impl ObjectMethods for CbDrawPaneDecorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawPaneDecorEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawPaneDecorEvent { CbDrawPaneDecorEvent { ptr: ptr } }
    pub fn null() -> CbDrawPaneDecorEvent { CbDrawPaneDecorEvent::from(0 as *mut c_void) }
    
}

pub trait CbDrawPaneDecorEventMethods : CbPluginEventMethods {
}

pub struct CbDrawRowBkGroundEvent { ptr: *mut c_void }
impl CbDrawRowBkGroundEventMethods for CbDrawRowBkGroundEvent {}
impl CbPluginEventMethods for CbDrawRowBkGroundEvent {}
impl EventMethods for CbDrawRowBkGroundEvent {}
impl ObjectMethods for CbDrawRowBkGroundEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawRowBkGroundEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawRowBkGroundEvent { CbDrawRowBkGroundEvent { ptr: ptr } }
    pub fn null() -> CbDrawRowBkGroundEvent { CbDrawRowBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait CbDrawRowBkGroundEventMethods : CbPluginEventMethods {
}

pub struct CbDrawRowDecorEvent { ptr: *mut c_void }
impl CbDrawRowDecorEventMethods for CbDrawRowDecorEvent {}
impl CbPluginEventMethods for CbDrawRowDecorEvent {}
impl EventMethods for CbDrawRowDecorEvent {}
impl ObjectMethods for CbDrawRowDecorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawRowDecorEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawRowDecorEvent { CbDrawRowDecorEvent { ptr: ptr } }
    pub fn null() -> CbDrawRowDecorEvent { CbDrawRowDecorEvent::from(0 as *mut c_void) }
    
}

pub trait CbDrawRowDecorEventMethods : CbPluginEventMethods {
}

pub struct CbDrawRowHandlesEvent { ptr: *mut c_void }
impl CbDrawRowHandlesEventMethods for CbDrawRowHandlesEvent {}
impl CbPluginEventMethods for CbDrawRowHandlesEvent {}
impl EventMethods for CbDrawRowHandlesEvent {}
impl ObjectMethods for CbDrawRowHandlesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDrawRowHandlesEvent {
    pub fn from(ptr: *mut c_void) -> CbDrawRowHandlesEvent { CbDrawRowHandlesEvent { ptr: ptr } }
    pub fn null() -> CbDrawRowHandlesEvent { CbDrawRowHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait CbDrawRowHandlesEventMethods : CbPluginEventMethods {
}

pub struct CbDynToolBarDimHandler { ptr: *mut c_void }
impl CbDynToolBarDimHandlerMethods for CbDynToolBarDimHandler {}
impl CbDimHandlerBaseMethods for CbDynToolBarDimHandler {}
impl ObjectMethods for CbDynToolBarDimHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbDynToolBarDimHandler {
    pub fn from(ptr: *mut c_void) -> CbDynToolBarDimHandler { CbDynToolBarDimHandler { ptr: ptr } }
    pub fn null() -> CbDynToolBarDimHandler { CbDynToolBarDimHandler::from(0 as *mut c_void) }
    
}

pub trait CbDynToolBarDimHandlerMethods : CbDimHandlerBaseMethods {
}

pub struct CbFinishDrawInAreaEvent { ptr: *mut c_void }
impl CbFinishDrawInAreaEventMethods for CbFinishDrawInAreaEvent {}
impl CbPluginEventMethods for CbFinishDrawInAreaEvent {}
impl EventMethods for CbFinishDrawInAreaEvent {}
impl ObjectMethods for CbFinishDrawInAreaEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbFinishDrawInAreaEvent {
    pub fn from(ptr: *mut c_void) -> CbFinishDrawInAreaEvent { CbFinishDrawInAreaEvent { ptr: ptr } }
    pub fn null() -> CbFinishDrawInAreaEvent { CbFinishDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait CbFinishDrawInAreaEventMethods : CbPluginEventMethods {
}

pub struct CbFloatedBarWindow { ptr: *mut c_void }
impl CbFloatedBarWindowMethods for CbFloatedBarWindow {}
impl ToolWindowMethods for CbFloatedBarWindow {}
impl FrameMethods for CbFloatedBarWindow {}
impl TopLevelWindowMethods for CbFloatedBarWindow {}
impl WindowMethods for CbFloatedBarWindow {}
impl EvtHandlerMethods for CbFloatedBarWindow {}
impl ObjectMethods for CbFloatedBarWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbFloatedBarWindow {
    pub fn from(ptr: *mut c_void) -> CbFloatedBarWindow { CbFloatedBarWindow { ptr: ptr } }
    pub fn null() -> CbFloatedBarWindow { CbFloatedBarWindow::from(0 as *mut c_void) }
    
}

pub trait CbFloatedBarWindowMethods : ToolWindowMethods {
}

pub struct CbGCUpdatesMgr { ptr: *mut c_void }
impl CbGCUpdatesMgrMethods for CbGCUpdatesMgr {}
impl CbSimpleUpdatesMgrMethods for CbGCUpdatesMgr {}
impl CbUpdatesManagerBaseMethods for CbGCUpdatesMgr {}
impl ObjectMethods for CbGCUpdatesMgr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbGCUpdatesMgr {
    pub fn from(ptr: *mut c_void) -> CbGCUpdatesMgr { CbGCUpdatesMgr { ptr: ptr } }
    pub fn null() -> CbGCUpdatesMgr { CbGCUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait CbGCUpdatesMgrMethods : CbSimpleUpdatesMgrMethods {
}

pub struct CbHintAnimationPlugin { ptr: *mut c_void }
impl CbHintAnimationPluginMethods for CbHintAnimationPlugin {}
impl CbPluginBaseMethods for CbHintAnimationPlugin {}
impl EvtHandlerMethods for CbHintAnimationPlugin {}
impl ObjectMethods for CbHintAnimationPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbHintAnimationPlugin {
    pub fn from(ptr: *mut c_void) -> CbHintAnimationPlugin { CbHintAnimationPlugin { ptr: ptr } }
    pub fn null() -> CbHintAnimationPlugin { CbHintAnimationPlugin::from(0 as *mut c_void) }
    
}

pub trait CbHintAnimationPluginMethods : CbPluginBaseMethods {
}

pub struct CbInsertBarEvent { ptr: *mut c_void }
impl CbInsertBarEventMethods for CbInsertBarEvent {}
impl CbPluginEventMethods for CbInsertBarEvent {}
impl EventMethods for CbInsertBarEvent {}
impl ObjectMethods for CbInsertBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbInsertBarEvent {
    pub fn from(ptr: *mut c_void) -> CbInsertBarEvent { CbInsertBarEvent { ptr: ptr } }
    pub fn null() -> CbInsertBarEvent { CbInsertBarEvent::from(0 as *mut c_void) }
    
}

pub trait CbInsertBarEventMethods : CbPluginEventMethods {
}

pub struct CbLayoutRowEvent { ptr: *mut c_void }
impl CbLayoutRowEventMethods for CbLayoutRowEvent {}
impl CbPluginEventMethods for CbLayoutRowEvent {}
impl EventMethods for CbLayoutRowEvent {}
impl ObjectMethods for CbLayoutRowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLayoutRowEvent {
    pub fn from(ptr: *mut c_void) -> CbLayoutRowEvent { CbLayoutRowEvent { ptr: ptr } }
    pub fn null() -> CbLayoutRowEvent { CbLayoutRowEvent::from(0 as *mut c_void) }
    
}

pub trait CbLayoutRowEventMethods : CbPluginEventMethods {
}

pub struct CbLeftDClickEvent { ptr: *mut c_void }
impl CbLeftDClickEventMethods for CbLeftDClickEvent {}
impl CbPluginEventMethods for CbLeftDClickEvent {}
impl EventMethods for CbLeftDClickEvent {}
impl ObjectMethods for CbLeftDClickEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLeftDClickEvent {
    pub fn from(ptr: *mut c_void) -> CbLeftDClickEvent { CbLeftDClickEvent { ptr: ptr } }
    pub fn null() -> CbLeftDClickEvent { CbLeftDClickEvent::from(0 as *mut c_void) }
    
}

pub trait CbLeftDClickEventMethods : CbPluginEventMethods {
}

pub struct CbLeftDownEvent { ptr: *mut c_void }
impl CbLeftDownEventMethods for CbLeftDownEvent {}
impl CbPluginEventMethods for CbLeftDownEvent {}
impl EventMethods for CbLeftDownEvent {}
impl ObjectMethods for CbLeftDownEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLeftDownEvent {
    pub fn from(ptr: *mut c_void) -> CbLeftDownEvent { CbLeftDownEvent { ptr: ptr } }
    pub fn null() -> CbLeftDownEvent { CbLeftDownEvent::from(0 as *mut c_void) }
    
}

pub trait CbLeftDownEventMethods : CbPluginEventMethods {
}

pub struct CbLeftUpEvent { ptr: *mut c_void }
impl CbLeftUpEventMethods for CbLeftUpEvent {}
impl CbPluginEventMethods for CbLeftUpEvent {}
impl EventMethods for CbLeftUpEvent {}
impl ObjectMethods for CbLeftUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbLeftUpEvent {
    pub fn from(ptr: *mut c_void) -> CbLeftUpEvent { CbLeftUpEvent { ptr: ptr } }
    pub fn null() -> CbLeftUpEvent { CbLeftUpEvent::from(0 as *mut c_void) }
    
}

pub trait CbLeftUpEventMethods : CbPluginEventMethods {
}

pub struct CbMiniButton { ptr: *mut c_void }
impl CbMiniButtonMethods for CbMiniButton {}
impl ObjectMethods for CbMiniButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbMiniButton {
    pub fn from(ptr: *mut c_void) -> CbMiniButton { CbMiniButton { ptr: ptr } }
    pub fn null() -> CbMiniButton { CbMiniButton::from(0 as *mut c_void) }
    
}

pub trait CbMiniButtonMethods : ObjectMethods {
}

pub struct CbMotionEvent { ptr: *mut c_void }
impl CbMotionEventMethods for CbMotionEvent {}
impl CbPluginEventMethods for CbMotionEvent {}
impl EventMethods for CbMotionEvent {}
impl ObjectMethods for CbMotionEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbMotionEvent {
    pub fn from(ptr: *mut c_void) -> CbMotionEvent { CbMotionEvent { ptr: ptr } }
    pub fn null() -> CbMotionEvent { CbMotionEvent::from(0 as *mut c_void) }
    
}

pub trait CbMotionEventMethods : CbPluginEventMethods {
}

pub struct CbPaneDrawPlugin { ptr: *mut c_void }
impl CbPaneDrawPluginMethods for CbPaneDrawPlugin {}
impl CbPluginBaseMethods for CbPaneDrawPlugin {}
impl EvtHandlerMethods for CbPaneDrawPlugin {}
impl ObjectMethods for CbPaneDrawPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbPaneDrawPlugin {
    pub fn from(ptr: *mut c_void) -> CbPaneDrawPlugin { CbPaneDrawPlugin { ptr: ptr } }
    pub fn null() -> CbPaneDrawPlugin { CbPaneDrawPlugin::from(0 as *mut c_void) }
    
}

pub trait CbPaneDrawPluginMethods : CbPluginBaseMethods {
}

pub struct CbPluginBase { ptr: *mut c_void }
impl CbPluginBaseMethods for CbPluginBase {}
impl EvtHandlerMethods for CbPluginBase {}
impl ObjectMethods for CbPluginBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbPluginBase {
    pub fn from(ptr: *mut c_void) -> CbPluginBase { CbPluginBase { ptr: ptr } }
    pub fn null() -> CbPluginBase { CbPluginBase::from(0 as *mut c_void) }
    
}

pub trait CbPluginBaseMethods : EvtHandlerMethods {
}

pub struct CbPluginEvent { ptr: *mut c_void }
impl CbPluginEventMethods for CbPluginEvent {}
impl EventMethods for CbPluginEvent {}
impl ObjectMethods for CbPluginEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbPluginEvent {
    pub fn from(ptr: *mut c_void) -> CbPluginEvent { CbPluginEvent { ptr: ptr } }
    pub fn null() -> CbPluginEvent { CbPluginEvent::from(0 as *mut c_void) }
    
}

pub trait CbPluginEventMethods : EventMethods {
}

pub struct CbRemoveBarEvent { ptr: *mut c_void }
impl CbRemoveBarEventMethods for CbRemoveBarEvent {}
impl CbPluginEventMethods for CbRemoveBarEvent {}
impl EventMethods for CbRemoveBarEvent {}
impl ObjectMethods for CbRemoveBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRemoveBarEvent {
    pub fn from(ptr: *mut c_void) -> CbRemoveBarEvent { CbRemoveBarEvent { ptr: ptr } }
    pub fn null() -> CbRemoveBarEvent { CbRemoveBarEvent::from(0 as *mut c_void) }
    
}

pub trait CbRemoveBarEventMethods : CbPluginEventMethods {
}

pub struct CbResizeBarEvent { ptr: *mut c_void }
impl CbResizeBarEventMethods for CbResizeBarEvent {}
impl CbPluginEventMethods for CbResizeBarEvent {}
impl EventMethods for CbResizeBarEvent {}
impl ObjectMethods for CbResizeBarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbResizeBarEvent {
    pub fn from(ptr: *mut c_void) -> CbResizeBarEvent { CbResizeBarEvent { ptr: ptr } }
    pub fn null() -> CbResizeBarEvent { CbResizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait CbResizeBarEventMethods : CbPluginEventMethods {
}

pub struct CbResizeRowEvent { ptr: *mut c_void }
impl CbResizeRowEventMethods for CbResizeRowEvent {}
impl CbPluginEventMethods for CbResizeRowEvent {}
impl EventMethods for CbResizeRowEvent {}
impl ObjectMethods for CbResizeRowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbResizeRowEvent {
    pub fn from(ptr: *mut c_void) -> CbResizeRowEvent { CbResizeRowEvent { ptr: ptr } }
    pub fn null() -> CbResizeRowEvent { CbResizeRowEvent::from(0 as *mut c_void) }
    
}

pub trait CbResizeRowEventMethods : CbPluginEventMethods {
}

pub struct CbRightDownEvent { ptr: *mut c_void }
impl CbRightDownEventMethods for CbRightDownEvent {}
impl CbPluginEventMethods for CbRightDownEvent {}
impl EventMethods for CbRightDownEvent {}
impl ObjectMethods for CbRightDownEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRightDownEvent {
    pub fn from(ptr: *mut c_void) -> CbRightDownEvent { CbRightDownEvent { ptr: ptr } }
    pub fn null() -> CbRightDownEvent { CbRightDownEvent::from(0 as *mut c_void) }
    
}

pub trait CbRightDownEventMethods : CbPluginEventMethods {
}

pub struct CbRightUpEvent { ptr: *mut c_void }
impl CbRightUpEventMethods for CbRightUpEvent {}
impl CbPluginEventMethods for CbRightUpEvent {}
impl EventMethods for CbRightUpEvent {}
impl ObjectMethods for CbRightUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRightUpEvent {
    pub fn from(ptr: *mut c_void) -> CbRightUpEvent { CbRightUpEvent { ptr: ptr } }
    pub fn null() -> CbRightUpEvent { CbRightUpEvent::from(0 as *mut c_void) }
    
}

pub trait CbRightUpEventMethods : CbPluginEventMethods {
}

pub struct CbRowDragPlugin { ptr: *mut c_void }
impl CbRowDragPluginMethods for CbRowDragPlugin {}
impl CbPluginBaseMethods for CbRowDragPlugin {}
impl EvtHandlerMethods for CbRowDragPlugin {}
impl ObjectMethods for CbRowDragPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRowDragPlugin {
    pub fn from(ptr: *mut c_void) -> CbRowDragPlugin { CbRowDragPlugin { ptr: ptr } }
    pub fn null() -> CbRowDragPlugin { CbRowDragPlugin::from(0 as *mut c_void) }
    
}

pub trait CbRowDragPluginMethods : CbPluginBaseMethods {
}

pub struct CbRowInfo { ptr: *mut c_void }
impl CbRowInfoMethods for CbRowInfo {}
impl ObjectMethods for CbRowInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRowInfo {
    pub fn from(ptr: *mut c_void) -> CbRowInfo { CbRowInfo { ptr: ptr } }
    pub fn null() -> CbRowInfo { CbRowInfo::from(0 as *mut c_void) }
    
}

pub trait CbRowInfoMethods : ObjectMethods {
}

pub struct CbRowLayoutPlugin { ptr: *mut c_void }
impl CbRowLayoutPluginMethods for CbRowLayoutPlugin {}
impl CbPluginBaseMethods for CbRowLayoutPlugin {}
impl EvtHandlerMethods for CbRowLayoutPlugin {}
impl ObjectMethods for CbRowLayoutPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbRowLayoutPlugin {
    pub fn from(ptr: *mut c_void) -> CbRowLayoutPlugin { CbRowLayoutPlugin { ptr: ptr } }
    pub fn null() -> CbRowLayoutPlugin { CbRowLayoutPlugin::from(0 as *mut c_void) }
    
}

pub trait CbRowLayoutPluginMethods : CbPluginBaseMethods {
}

pub struct CbSimpleCustomizationPlugin { ptr: *mut c_void }
impl CbSimpleCustomizationPluginMethods for CbSimpleCustomizationPlugin {}
impl CbPluginBaseMethods for CbSimpleCustomizationPlugin {}
impl EvtHandlerMethods for CbSimpleCustomizationPlugin {}
impl ObjectMethods for CbSimpleCustomizationPlugin { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbSimpleCustomizationPlugin {
    pub fn from(ptr: *mut c_void) -> CbSimpleCustomizationPlugin { CbSimpleCustomizationPlugin { ptr: ptr } }
    pub fn null() -> CbSimpleCustomizationPlugin { CbSimpleCustomizationPlugin::from(0 as *mut c_void) }
    
}

pub trait CbSimpleCustomizationPluginMethods : CbPluginBaseMethods {
}

pub struct CbSimpleUpdatesMgr { ptr: *mut c_void }
impl CbSimpleUpdatesMgrMethods for CbSimpleUpdatesMgr {}
impl CbUpdatesManagerBaseMethods for CbSimpleUpdatesMgr {}
impl ObjectMethods for CbSimpleUpdatesMgr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbSimpleUpdatesMgr {
    pub fn from(ptr: *mut c_void) -> CbSimpleUpdatesMgr { CbSimpleUpdatesMgr { ptr: ptr } }
    pub fn null() -> CbSimpleUpdatesMgr { CbSimpleUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait CbSimpleUpdatesMgrMethods : CbUpdatesManagerBaseMethods {
}

pub struct CbSizeBarWndEvent { ptr: *mut c_void }
impl CbSizeBarWndEventMethods for CbSizeBarWndEvent {}
impl CbPluginEventMethods for CbSizeBarWndEvent {}
impl EventMethods for CbSizeBarWndEvent {}
impl ObjectMethods for CbSizeBarWndEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbSizeBarWndEvent {
    pub fn from(ptr: *mut c_void) -> CbSizeBarWndEvent { CbSizeBarWndEvent { ptr: ptr } }
    pub fn null() -> CbSizeBarWndEvent { CbSizeBarWndEvent::from(0 as *mut c_void) }
    
}

pub trait CbSizeBarWndEventMethods : CbPluginEventMethods {
}

pub struct CbStartBarDraggingEvent { ptr: *mut c_void }
impl CbStartBarDraggingEventMethods for CbStartBarDraggingEvent {}
impl CbPluginEventMethods for CbStartBarDraggingEvent {}
impl EventMethods for CbStartBarDraggingEvent {}
impl ObjectMethods for CbStartBarDraggingEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbStartBarDraggingEvent {
    pub fn from(ptr: *mut c_void) -> CbStartBarDraggingEvent { CbStartBarDraggingEvent { ptr: ptr } }
    pub fn null() -> CbStartBarDraggingEvent { CbStartBarDraggingEvent::from(0 as *mut c_void) }
    
}

pub trait CbStartBarDraggingEventMethods : CbPluginEventMethods {
}

pub struct CbStartDrawInAreaEvent { ptr: *mut c_void }
impl CbStartDrawInAreaEventMethods for CbStartDrawInAreaEvent {}
impl CbPluginEventMethods for CbStartDrawInAreaEvent {}
impl EventMethods for CbStartDrawInAreaEvent {}
impl ObjectMethods for CbStartDrawInAreaEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbStartDrawInAreaEvent {
    pub fn from(ptr: *mut c_void) -> CbStartDrawInAreaEvent { CbStartDrawInAreaEvent { ptr: ptr } }
    pub fn null() -> CbStartDrawInAreaEvent { CbStartDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait CbStartDrawInAreaEventMethods : CbPluginEventMethods {
}

pub struct CbUpdatesManagerBase { ptr: *mut c_void }
impl CbUpdatesManagerBaseMethods for CbUpdatesManagerBase {}
impl ObjectMethods for CbUpdatesManagerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CbUpdatesManagerBase {
    pub fn from(ptr: *mut c_void) -> CbUpdatesManagerBase { CbUpdatesManagerBase { ptr: ptr } }
    pub fn null() -> CbUpdatesManagerBase { CbUpdatesManagerBase::from(0 as *mut c_void) }
    
}

pub trait CbUpdatesManagerBaseMethods : ObjectMethods {
}

