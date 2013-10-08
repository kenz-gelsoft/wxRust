use std::libc::*;
use native::*;

// skipping globals...

trait wxMultiCellItemHandle {
    // missing: wxMultiCellItemHandle_Create
    // missing: wxMultiCellItemHandle_CreateWithSize
    // missing: wxMultiCellItemHandle_CreateWithStyle
    // missing: wxMultiCellItemHandle_GetAlignment
    // missing: wxMultiCellItemHandle_GetColumn
    // missing: wxMultiCellItemHandle_GetHeight
    // missing: wxMultiCellItemHandle_GetLocalSize
    // missing: wxMultiCellItemHandle_GetRow
    // missing: wxMultiCellItemHandle_GetStyle
    // missing: wxMultiCellItemHandle_GetWeight
    // missing: wxMultiCellItemHandle_GetWidth
}
trait wxKeyEvent {
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_AltDown(_obj)
        }
    }
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_ControlDown(_obj)
        }
    }
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxKeyEvent_CopyObject(_obj, obj)
        }
    }
    fn GetKeyCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxKeyEvent_GetKeyCode(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxKeyEvent_GetPosition(_obj)
        }
    }
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxKeyEvent_GetX(_obj)
        }
    }
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxKeyEvent_GetY(_obj)
        }
    }
    fn GetModifiers(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxKeyEvent_GetModifiers(_obj)
        }
    }
    fn HasModifiers(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_HasModifiers(_obj)
        }
    }
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_MetaDown(_obj)
        }
    }
    fn SetKeyCode(_obj: *u8 /* void* */, code: c_int /* int */) {
        unsafe {
            wxKeyEvent_SetKeyCode(_obj, code)
        }
    }
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_ShiftDown(_obj)
        }
    }
}
trait wxWindow {
    fn AddChild(_obj: *u8 /* void* */, child: *u8 /* void* */) {
        unsafe {
            wxWindow_AddChild(_obj, child)
        }
    }
    fn AddConstraintReference(_obj: *u8 /* void* */, otherWin: *u8 /* void* */) {
        unsafe {
            wxWindow_AddConstraintReference(_obj, otherWin)
        }
    }
    fn CaptureMouse(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_CaptureMouse(_obj)
        }
    }
    fn Center(_obj: *u8 /* void* */, direction: c_int /* int */) {
        unsafe {
            wxWindow_Center(_obj, direction)
        }
    }
    fn CenterOnParent(_obj: *u8 /* void* */, dir: c_int /* int */) {
        unsafe {
            wxWindow_CenterOnParent(_obj, dir)
        }
    }
    fn ClearBackground(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_ClearBackground(_obj)
        }
    }
    fn ClientToScreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ClientToScreen(_obj, arg0, arg1)
        }
    }
    fn Close(_obj: *u8 /* void* */, _force: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxWindow_Close(_obj, _force)
        }
    }
    fn ConvertDialogToPixels(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ConvertDialogToPixels(_obj)
        }
    }
    fn ConvertPixelsToDialog(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ConvertPixelsToDialog(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn DeleteRelatedConstraints(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_DeleteRelatedConstraints(_obj)
        }
    }
    fn Destroy(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Destroy(_obj)
        }
    }
    fn DestroyChildren(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_DestroyChildren(_obj)
        }
    }
    fn Disable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Disable(_obj)
        }
    }
    fn DoPhase(_obj: *u8 /* void* */, phase: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_DoPhase(_obj, phase)
        }
    }
    fn Enable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Enable(_obj)
        }
    }
    fn FindFocus(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_FindFocus(_obj)
        }
    }
    fn FindWindow(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_FindWindow(_obj, name)
        }
    }
    fn Fit(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Fit(_obj)
        }
    }
    fn FitInside(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_FitInside(_obj)
        }
    }
    fn Freeze(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Freeze(_obj)
        }
    }
    fn GetEffectiveMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetEffectiveMinSize(_obj)
        }
    }
    fn GetAutoLayout(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetAutoLayout(_obj)
        }
    }
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxWindow_GetBackgroundColour(_obj, _ref)
        }
    }
    fn GetBestSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetBestSize(_obj)
        }
    }
    fn GetCaret(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetCaret(_obj)
        }
    }
    fn GetCharHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetCharHeight(_obj)
        }
    }
    fn GetCharWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetCharWidth(_obj)
        }
    }
    fn GetChildren(_obj: *u8 /* void* */, _res: *u8 /* void* */, _cnt: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetChildren(_obj, _res, _cnt)
        }
    }
    fn GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetClientData(_obj)
        }
    }
    fn GetClientSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetClientSize(_obj)
        }
    }
    fn GetClientSizeConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxWindow_GetClientSizeConstraint(_obj, arg0, arg1)
        }
    }
    fn GetConstraints(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetConstraints(_obj)
        }
    }
    fn GetConstraintsInvolvedIn(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetConstraintsInvolvedIn(_obj)
        }
    }
    fn GetCursor(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetCursor(_obj)
        }
    }
    fn GetDropTarget(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetDropTarget(_obj)
        }
    }
    fn GetEventHandler(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetEventHandler(_obj)
        }
    }
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxWindow_GetFont(_obj, _ref)
        }
    }
    fn GetForegroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxWindow_GetForegroundColour(_obj, _ref)
        }
    }
    fn GetHandle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetHandle(_obj)
        }
    }
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetId(_obj)
        }
    }
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetLabel(_obj)
        }
    }
    fn GetLabelEmpty(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetLabelEmpty(_obj)
        }
    }
    fn GetMaxHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetMaxHeight(_obj)
        }
    }
    fn GetMaxWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetMaxWidth(_obj)
        }
    }
    fn GetMinHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetMinHeight(_obj)
        }
    }
    fn GetMinWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetMinWidth(_obj)
        }
    }
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetName(_obj)
        }
    }
    fn GetParent(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetParent(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetPosition(_obj)
        }
    }
    fn GetPositionConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxWindow_GetPositionConstraint(_obj, arg0, arg1)
        }
    }
    fn GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetRect(_obj)
        }
    }
    fn GetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetScrollPos(_obj, orient)
        }
    }
    fn GetScrollRange(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetScrollRange(_obj, orient)
        }
    }
    fn GetScrollThumb(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetScrollThumb(_obj, orient)
        }
    }
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetSize(_obj)
        }
    }
    fn GetSizeConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxWindow_GetSizeConstraint(_obj, arg0, arg1)
        }
    }
    fn GetSizer(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetSizer(_obj)
        }
    }
    fn GetTextExtent(_obj: *u8 /* void* */, string: *u8 /* void* */, x: *c_int /* int* */, y: *c_int /* int* */, descent: *c_int /* int* */, externalLeading: *c_int /* int* */, theFont: *u8 /* void* */) {
        unsafe {
            wxWindow_GetTextExtent(_obj, string, x, y, descent, externalLeading, theFont)
        }
    }
    fn GetToolTip(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetToolTip(_obj)
        }
    }
    fn GetUpdateRegion(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetUpdateRegion(_obj)
        }
    }
    fn GetValidator(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetValidator(_obj)
        }
    }
    fn GetVirtualSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetVirtualSize(_obj)
        }
    }
    fn GetWindowStyleFlag(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetWindowStyleFlag(_obj)
        }
    }
    fn HasFlag(_obj: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxWindow_HasFlag(_obj, flag)
        }
    }
    fn Hide(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Hide(_obj)
        }
    }
    fn InitDialog(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_InitDialog(_obj)
        }
    }
    fn IsBeingDeleted(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsBeingDeleted(_obj)
        }
    }
    fn IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsEnabled(_obj)
        }
    }
    fn IsExposed(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsExposed(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn IsShown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsShown(_obj)
        }
    }
    fn IsTopLevel(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsTopLevel(_obj)
        }
    }
    fn Layout(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_Layout(_obj)
        }
    }
    fn LayoutPhase1(_obj: *u8 /* void* */, noChanges: *c_int /* int* */) -> c_int /* int */ {
        unsafe {
            wxWindow_LayoutPhase1(_obj, noChanges)
        }
    }
    fn LayoutPhase2(_obj: *u8 /* void* */, noChanges: *c_int /* int* */) -> c_int /* int */ {
        unsafe {
            wxWindow_LayoutPhase2(_obj, noChanges)
        }
    }
    fn Lower(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Lower(_obj)
        }
    }
    fn MakeModal(_obj: *u8 /* void* */, modal: bool /* bool */) {
        unsafe {
            wxWindow_MakeModal(_obj, modal)
        }
    }
    fn Move(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_Move(_obj, arg0, arg1)
        }
    }
    fn MoveConstraint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_MoveConstraint(_obj, arg0, arg1)
        }
    }
    fn PopEventHandler(_obj: *u8 /* void* */, deleteHandler: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_PopEventHandler(_obj, deleteHandler)
        }
    }
    fn PopupMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_PopupMenu(_obj, menu, arg0, arg1)
        }
    }
    fn PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxWindow_PrepareDC(_obj, dc)
        }
    }
    fn PushEventHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxWindow_PushEventHandler(_obj, handler)
        }
    }
    fn Raise(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Raise(_obj)
        }
    }
    fn Refresh(_obj: *u8 /* void* */, eraseBackground: bool /* bool */) {
        unsafe {
            wxWindow_Refresh(_obj, eraseBackground)
        }
    }
    fn RefreshRect(_obj: *u8 /* void* */, eraseBackground: bool /* bool */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxWindow_RefreshRect(_obj, eraseBackground, arg0, arg1, arg2, arg3)
        }
    }
    fn ReleaseMouse(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_ReleaseMouse(_obj)
        }
    }
    fn RemoveChild(_obj: *u8 /* void* */, child: *u8 /* void* */) {
        unsafe {
            wxWindow_RemoveChild(_obj, child)
        }
    }
    fn RemoveConstraintReference(_obj: *u8 /* void* */, otherWin: *u8 /* void* */) {
        unsafe {
            wxWindow_RemoveConstraintReference(_obj, otherWin)
        }
    }
    fn Reparent(_obj: *u8 /* void* */, _par: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_Reparent(_obj, _par)
        }
    }
    fn ResetConstraints(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_ResetConstraints(_obj)
        }
    }
    fn ScreenToClient(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ScreenToClient(_obj, arg0, arg1)
        }
    }
    fn ScrollWindow(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_ScrollWindow(_obj, arg0, arg1)
        }
    }
    fn ScrollWindowRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: c_int /* int */) {
        unsafe {
            wxWindow_ScrollWindowRect(_obj, arg0, arg1, arg2, arg3, arg4, arg5)
        }
    }
    fn SetAcceleratorTable(_obj: *u8 /* void* */, accel: *u8 /* void* */) {
        unsafe {
            wxWindow_SetAcceleratorTable(_obj, accel)
        }
    }
    fn SetAutoLayout(_obj: *u8 /* void* */, autoLayout: bool /* bool */) {
        unsafe {
            wxWindow_SetAutoLayout(_obj, autoLayout)
        }
    }
    fn SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_SetBackgroundColour(_obj, colour)
        }
    }
    fn SetCaret(_obj: *u8 /* void* */, caret: *u8 /* void* */) {
        unsafe {
            wxWindow_SetCaret(_obj, caret)
        }
    }
    fn SetClientData(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxWindow_SetClientData(_obj, data)
        }
    }
    fn SetClientObject(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxWindow_SetClientObject(_obj, data)
        }
    }
    fn SetClientSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_SetClientSize(_obj, arg0, arg1)
        }
    }
    fn SetConstraintSizes(_obj: *u8 /* void* */, recurse: c_int /* int */) {
        unsafe {
            wxWindow_SetConstraintSizes(_obj, recurse)
        }
    }
    fn SetConstraints(_obj: *u8 /* void* */, constraints: *u8 /* void* */) {
        unsafe {
            wxWindow_SetConstraints(_obj, constraints)
        }
    }
    fn SetCursor(_obj: *u8 /* void* */, cursor: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_SetCursor(_obj, cursor)
        }
    }
    fn SetDropTarget(_obj: *u8 /* void* */, dropTarget: *u8 /* void* */) {
        unsafe {
            wxWindow_SetDropTarget(_obj, dropTarget)
        }
    }
    fn SetExtraStyle(_obj: *u8 /* void* */, exStyle: c_long /* long */) {
        unsafe {
            wxWindow_SetExtraStyle(_obj, exStyle)
        }
    }
    fn SetFocus(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_SetFocus(_obj)
        }
    }
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_SetFont(_obj, font)
        }
    }
    fn SetForegroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_SetForegroundColour(_obj, colour)
        }
    }
    fn SetId(_obj: *u8 /* void* */, _id: c_int /* int */) {
        unsafe {
            wxWindow_SetId(_obj, _id)
        }
    }
    fn SetLabel(_obj: *u8 /* void* */, _title: *u8 /* void* */) {
        unsafe {
            wxWindow_SetLabel(_obj, _title)
        }
    }
    fn SetName(_obj: *u8 /* void* */, _name: *u8 /* void* */) {
        unsafe {
            wxWindow_SetName(_obj, _name)
        }
    }
    fn SetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */, pos: c_int /* int */, refresh: bool /* bool */) {
        unsafe {
            wxWindow_SetScrollPos(_obj, orient, pos, refresh)
        }
    }
    fn SetScrollbar(_obj: *u8 /* void* */, orient: c_int /* int */, pos: c_int /* int */, thumbVisible: c_int /* int */, range: c_int /* int */, refresh: bool /* bool */) {
        unsafe {
            wxWindow_SetScrollbar(_obj, orient, pos, thumbVisible, range, refresh)
        }
    }
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, sizeFlags: c_int /* int */) {
        unsafe {
            wxWindow_SetSize(_obj, arg0, arg1, arg2, arg3, sizeFlags)
        }
    }
    fn SetSizeConstraint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxWindow_SetSizeConstraint(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn SetSizeHints(_obj: *u8 /* void* */, minW: c_int /* int */, minH: c_int /* int */, maxW: c_int /* int */, maxH: c_int /* int */, incW: c_int /* int */, incH: c_int /* int */) {
        unsafe {
            wxWindow_SetSizeHints(_obj, minW, minH, maxW, maxH, incW, incH)
        }
    }
    fn SetSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) {
        unsafe {
            wxWindow_SetSizer(_obj, sizer)
        }
    }
    fn SetToolTip(_obj: *u8 /* void* */, tip: *u8 /* void* */) {
        unsafe {
            wxWindow_SetToolTip(_obj, tip)
        }
    }
    fn SetValidator(_obj: *u8 /* void* */, validator: *u8 /* void* */) {
        unsafe {
            wxWindow_SetValidator(_obj, validator)
        }
    }
    fn SetWindowStyleFlag(_obj: *u8 /* void* */, style: c_long /* long */) {
        unsafe {
            wxWindow_SetWindowStyleFlag(_obj, style)
        }
    }
    fn Show(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Show(_obj)
        }
    }
    fn Thaw(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Thaw(_obj)
        }
    }
    fn TransferDataFromWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_TransferDataFromWindow(_obj)
        }
    }
    fn TransferDataToWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_TransferDataToWindow(_obj)
        }
    }
    fn UnsetConstraints(_obj: *u8 /* void* */, c: *u8 /* void* */) {
        unsafe {
            wxWindow_UnsetConstraints(_obj, c)
        }
    }
    fn UpdateWindowUI(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_UpdateWindowUI(_obj)
        }
    }
    fn Validate(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Validate(_obj)
        }
    }
    fn SetVirtualSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_SetVirtualSize(_obj, arg0, arg1)
        }
    }
    fn WarpPointer(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_WarpPointer(_obj, arg0, arg1)
        }
    }
}
trait wxCustomDataObject {
}
trait wxSashWindow {
    fn Create(_par: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSashWindow_Create(_par, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetDefaultBorderSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetDefaultBorderSize(_obj)
        }
    }
    fn GetEdgeMargin(_obj: *u8 /* void* */, edge: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetEdgeMargin(_obj, edge)
        }
    }
    fn GetExtraBorderSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetExtraBorderSize(_obj)
        }
    }
    fn GetMaximumSizeX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetMaximumSizeX(_obj)
        }
    }
    fn GetMaximumSizeY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetMaximumSizeY(_obj)
        }
    }
    fn GetMinimumSizeX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetMinimumSizeX(_obj)
        }
    }
    fn GetMinimumSizeY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetMinimumSizeY(_obj)
        }
    }
    fn GetSashVisible(_obj: *u8 /* void* */, edge: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSashWindow_GetSashVisible(_obj, edge)
        }
    }
    fn HasBorder(_obj: *u8 /* void* */, edge: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSashWindow_HasBorder(_obj, edge)
        }
    }
    fn SetDefaultBorderSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxSashWindow_SetDefaultBorderSize(_obj, width)
        }
    }
    fn SetExtraBorderSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxSashWindow_SetExtraBorderSize(_obj, width)
        }
    }
    fn SetMaximumSizeX(_obj: *u8 /* void* */, max: c_int /* int */) {
        unsafe {
            wxSashWindow_SetMaximumSizeX(_obj, max)
        }
    }
    fn SetMaximumSizeY(_obj: *u8 /* void* */, max: c_int /* int */) {
        unsafe {
            wxSashWindow_SetMaximumSizeY(_obj, max)
        }
    }
    fn SetMinimumSizeX(_obj: *u8 /* void* */, min: c_int /* int */) {
        unsafe {
            wxSashWindow_SetMinimumSizeX(_obj, min)
        }
    }
    fn SetMinimumSizeY(_obj: *u8 /* void* */, min: c_int /* int */) {
        unsafe {
            wxSashWindow_SetMinimumSizeY(_obj, min)
        }
    }
    fn SetSashBorder(_obj: *u8 /* void* */, edge: c_int /* int */, border: bool /* bool */) {
        unsafe {
            wxSashWindow_SetSashBorder(_obj, edge, border)
        }
    }
    fn SetSashVisible(_obj: *u8 /* void* */, edge: c_int /* int */, sash: bool /* bool */) {
        unsafe {
            wxSashWindow_SetSashVisible(_obj, edge, sash)
        }
    }
}
trait wxRealPoint {
}
trait wxSystemSettings {
    fn GetColour(index: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxSystemSettings_GetColour(index, _ref)
        }
    }
    fn GetFont(index: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxSystemSettings_GetFont(index, _ref)
        }
    }
    fn GetMetric(index: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxSystemSettings_GetMetric(index)
        }
    }
    fn GetScreenType() -> c_int /* int */ {
        unsafe {
            wxSystemSettings_GetScreenType()
        }
    }
}
trait cbBarSpy {
    // missing: cbBarSpy_Create
    // missing: cbBarSpy_CreateDefault
    // missing: cbBarSpy_Delete
    // missing: cbBarSpy_ProcessEvent
    // missing: cbBarSpy_SetBarWindow
}
trait wxNotebookEvent {
}
trait wxJoystickEvent {
    fn ButtonDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_ButtonDown(_obj, but)
        }
    }
    fn ButtonIsDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_ButtonIsDown(_obj, but)
        }
    }
    fn ButtonUp(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_ButtonUp(_obj, but)
        }
    }
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxJoystickEvent_CopyObject(_obj, obj)
        }
    }
    fn GetButtonChange(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystickEvent_GetButtonChange(_obj)
        }
    }
    fn GetButtonState(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystickEvent_GetButtonState(_obj)
        }
    }
    fn GetJoystick(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystickEvent_GetJoystick(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxJoystickEvent_GetPosition(_obj)
        }
    }
    fn GetZPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystickEvent_GetZPosition(_obj)
        }
    }
    fn IsButton(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_IsButton(_obj)
        }
    }
    fn IsMove(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_IsMove(_obj)
        }
    }
    fn IsZMove(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_IsZMove(_obj)
        }
    }
    fn SetButtonChange(_obj: *u8 /* void* */, change: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetButtonChange(_obj, change)
        }
    }
    fn SetButtonState(_obj: *u8 /* void* */, state: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetButtonState(_obj, state)
        }
    }
    fn SetJoystick(_obj: *u8 /* void* */, stick: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetJoystick(_obj, stick)
        }
    }
    fn SetPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetPosition(_obj, arg0, arg1)
        }
    }
    fn SetZPosition(_obj: *u8 /* void* */, zPos: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetZPosition(_obj, zPos)
        }
    }
}
trait wxMBConvUTF7 {
}
trait wxScrollWinEvent {
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollWinEvent_GetOrientation(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollWinEvent_GetPosition(_obj)
        }
    }
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */) {
        unsafe {
            wxScrollWinEvent_SetOrientation(_obj, orient)
        }
    }
    fn SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxScrollWinEvent_SetPosition(_obj, pos)
        }
    }
}
trait wxMBConvUTF8 {
}
trait wxCountingOutputStream {
}
trait wxDataObjectComposite {
    fn Add(_obj: *u8 /* void* */, _dat: *u8 /* void* */, _preferred: c_int /* int */) {
        unsafe {
            wxDataObjectComposite_Add(_obj, _dat, _preferred)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxDataObjectComposite_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDataObjectComposite_Delete(_obj)
        }
    }
}
trait wxScreenDC {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxScreenDC_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxScreenDC_Delete(_obj)
        }
    }
    fn EndDrawingOnTop(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxScreenDC_EndDrawingOnTop(_obj)
        }
    }
    fn StartDrawingOnTop(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxScreenDC_StartDrawingOnTop(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn StartDrawingOnTopOfWin(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxScreenDC_StartDrawingOnTopOfWin(_obj, win)
        }
    }
}
trait wxHelpControllerBase {
}
trait wxTextDropTarget {
}
trait wxBoolProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxBoolProperty_Create(label, name, value)
        }
    }
}
trait wxSysColourChangedEvent {
}
trait wxDrawControl {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDrawControl_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait cbRowDragPlugin {
    // missing: cbRowDragPlugin_Create
    // missing: cbRowDragPlugin_CreateDefault
    // missing: cbRowDragPlugin_Delete
}
trait wxBufferedInputStream {
}
trait wxIconBundle {
    fn AddIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) {
        unsafe {
            wxIconBundle_AddIcon(_obj, icon)
        }
    }
    fn AddIconFromFile(_obj: *u8 /* void* */, file: *u8 /* void* */, type_: c_int /* int */) {
        unsafe {
            wxIconBundle_AddIconFromFile(_obj, file, type_)
        }
    }
    fn Assign(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxIconBundle_Assign(_obj, _ref)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxIconBundle_CreateDefault()
        }
    }
    fn CreateFromFile(file: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxIconBundle_CreateFromFile(file, type_)
        }
    }
    fn CreateFromIcon(icon: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxIconBundle_CreateFromIcon(icon)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxIconBundle_Delete(_obj)
        }
    }
    fn GetIcon(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxIconBundle_GetIcon(_obj, arg0, arg1, _ref)
        }
    }
}
trait wxGridCellFloatRenderer {
}
trait wxTextValidator {
    fn Create(style: c_int /* int */, val: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextValidator_Create(style, val)
        }
    }
    fn GetExcludes(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxTextValidator_GetExcludes(_obj, _ref)
        }
    }
    fn GetIncludes(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxTextValidator_GetIncludes(_obj, _ref)
        }
    }
    fn SetExcludes(_obj: *u8 /* void* */, list: *wchar_t /* wchar_t* */, count: c_int /* int */) {
        unsafe {
            wxTextValidator_SetExcludes(_obj, list, count)
        }
    }
    fn SetIncludes(_obj: *u8 /* void* */, list: *wchar_t /* wchar_t* */, count: c_int /* int */) {
        unsafe {
            wxTextValidator_SetIncludes(_obj, list, count)
        }
    }
    fn Clone(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextValidator_Clone(_obj)
        }
    }
    fn TransferToWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextValidator_TransferToWindow(_obj)
        }
    }
    fn TransferFromWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextValidator_TransferFromWindow(_obj)
        }
    }
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTextValidator_GetStyle(_obj)
        }
    }
    fn OnChar(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxTextValidator_OnChar(_obj, event)
        }
    }
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxTextValidator_SetStyle(_obj, style)
        }
    }
}
trait wxDDEConnection {
}
trait wxCSConv {
}
trait cbStartDrawInAreaEvent {
    // missing: cbStartDrawInAreaEvent_Area
}
trait ELJCommand {
    // missing: ELJCommand_CanUndo
    // missing: ELJCommand_Create
    // missing: ELJCommand_Delete
    // missing: ELJCommand_GetName
}
trait ELJDragDataObject {
    fn Create(_obj: *u8 /* void* */, _fmt: *u8 /* void* */, _func1: *u8 /* void* */, _func2: *u8 /* void* */, _func3: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJDragDataObject_Create(_obj, _fmt, _func1, _func2, _func3)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJDragDataObject_Delete(_obj)
        }
    }
}
trait wxStaticLine {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStaticLine_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetDefaultSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStaticLine_GetDefaultSize(_obj)
        }
    }
    fn IsVertical(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStaticLine_IsVertical(_obj)
        }
    }
}
trait wxWindowDisabler {
}
trait wxTextInputStream {
}
trait wxListCtrl {
    fn Arrange(_obj: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_Arrange(_obj, flag)
        }
    }
    fn ClearAll(_obj: *u8 /* void* */) {
        unsafe {
            wxListCtrl_ClearAll(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn DeleteAllColumns(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_DeleteAllColumns(_obj)
        }
    }
    fn DeleteAllItems(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_DeleteAllItems(_obj)
        }
    }
    fn DeleteColumn(_obj: *u8 /* void* */, col: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_DeleteColumn(_obj, col)
        }
    }
    fn DeleteItem(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_DeleteItem(_obj, item)
        }
    }
    fn EditLabel(_obj: *u8 /* void* */, item: c_int /* int */) {
        unsafe {
            wxListCtrl_EditLabel(_obj, item)
        }
    }
    fn EndEditLabel(_obj: *u8 /* void* */, cancel: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_EndEditLabel(_obj, cancel)
        }
    }
    fn EnsureVisible(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_EnsureVisible(_obj, item)
        }
    }
    fn FindItem(_obj: *u8 /* void* */, start: c_int /* int */, str: *u8 /* void* */, partial: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_FindItem(_obj, start, str, partial)
        }
    }
    fn FindItemByData(_obj: *u8 /* void* */, start: c_int /* int */, data: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_FindItemByData(_obj, start, data)
        }
    }
    fn FindItemByPosition(_obj: *u8 /* void* */, start: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, direction: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_FindItemByPosition(_obj, start, arg0, arg1, direction)
        }
    }
    fn GetColumn(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_GetColumn(_obj, col, item)
        }
    }
    fn GetColumnCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetColumnCount(_obj)
        }
    }
    fn GetColumnWidth(_obj: *u8 /* void* */, col: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetColumnWidth(_obj, col)
        }
    }
    fn GetCountPerPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetCountPerPage(_obj)
        }
    }
    fn GetEditControl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetEditControl(_obj)
        }
    }
    fn GetImageList(_obj: *u8 /* void* */, which: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetImageList(_obj, which)
        }
    }
    fn GetItem(_obj: *u8 /* void* */, info: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_GetItem(_obj, info)
        }
    }
    fn GetItemCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetItemCount(_obj)
        }
    }
    fn GetItemData(_obj: *u8 /* void* */, item: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetItemData(_obj, item)
        }
    }
    fn GetItemFont(_obj: *u8 /* void* */, item: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemFont(_obj, item)
        }
    }
    fn GetItemPosition(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemPosition(_obj, item)
        }
    }
    fn GetItemRect(_obj: *u8 /* void* */, item: c_int /* int */, code: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemRect(_obj, item, code)
        }
    }
    fn GetItemSpacing(_obj: *u8 /* void* */, isSmall: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemSpacing(_obj, isSmall)
        }
    }
    fn GetItemState(_obj: *u8 /* void* */, item: c_int /* int */, stateMask: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetItemState(_obj, item, stateMask)
        }
    }
    fn GetItemText(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemText(_obj, item)
        }
    }
    fn GetNextItem(_obj: *u8 /* void* */, item: c_int /* int */, geometry: c_int /* int */, state: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetNextItem(_obj, item, geometry, state)
        }
    }
    fn GetSelectedItemCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetSelectedItemCount(_obj)
        }
    }
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListCtrl_GetTextColour(_obj, _ref)
        }
    }
    fn GetTopItem(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetTopItem(_obj)
        }
    }
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_HitTest(_obj, arg0, arg1, flags)
        }
    }
    fn InsertColumn(_obj: *u8 /* void* */, col: c_int /* int */, heading: *u8 /* void* */, format: c_int /* int */, width: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertColumn(_obj, col, heading, format, width)
        }
    }
    fn InsertColumnFromInfo(_obj: *u8 /* void* */, col: c_int /* int */, info: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertColumnFromInfo(_obj, col, info)
        }
    }
    fn InsertItem(_obj: *u8 /* void* */, info: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertItem(_obj, info)
        }
    }
    fn InsertItemWithData(_obj: *u8 /* void* */, index: c_int /* int */, label: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertItemWithData(_obj, index, label)
        }
    }
    fn InsertItemWithImage(_obj: *u8 /* void* */, index: c_int /* int */, imageIndex: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertItemWithImage(_obj, index, imageIndex)
        }
    }
    fn InsertItemWithLabel(_obj: *u8 /* void* */, index: c_int /* int */, label: *u8 /* void* */, imageIndex: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertItemWithLabel(_obj, index, label, imageIndex)
        }
    }
    fn IsVirtual(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_IsVirtual(_obj)
        }
    }
    fn RefreshItem(_obj: *u8 /* void* */, item: c_long /* long */) {
        unsafe {
            wxListCtrl_RefreshItem(_obj, item)
        }
    }
    fn ScrollList(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_ScrollList(_obj, arg0, arg1)
        }
    }
    fn SetBackgroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxListCtrl_SetBackgroundColour(_obj, col)
        }
    }
    fn SetColumn(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetColumn(_obj, col, item)
        }
    }
    fn SetColumnWidth(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetColumnWidth(_obj, col, width)
        }
    }
    fn SetForegroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_SetForegroundColour(_obj, col)
        }
    }
    fn SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */, which: c_int /* int */) {
        unsafe {
            wxListCtrl_SetImageList(_obj, imageList, which)
        }
    }
    fn SetItem(_obj: *u8 /* void* */, index: c_int /* int */, col: c_int /* int */, label: *u8 /* void* */, imageId: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItem(_obj, index, col, label, imageId)
        }
    }
    fn SetItemData(_obj: *u8 /* void* */, item: c_int /* int */, data: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemData(_obj, item, data)
        }
    }
    fn SetItemFromInfo(_obj: *u8 /* void* */, info: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemFromInfo(_obj, info)
        }
    }
    fn SetItemImage(_obj: *u8 /* void* */, item: c_int /* int */, image: c_int /* int */, selImage: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemImage(_obj, item, image, selImage)
        }
    }
    fn SetItemPosition(_obj: *u8 /* void* */, item: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemPosition(_obj, item, arg0, arg1)
        }
    }
    fn SetItemState(_obj: *u8 /* void* */, item: c_int /* int */, state: c_int /* int */, stateMask: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemState(_obj, item, state, stateMask)
        }
    }
    fn SetItemText(_obj: *u8 /* void* */, item: c_int /* int */, str: *u8 /* void* */) {
        unsafe {
            wxListCtrl_SetItemText(_obj, item, str)
        }
    }
    fn SetSingleStyle(_obj: *u8 /* void* */, style: c_int /* int */, add: bool /* bool */) {
        unsafe {
            wxListCtrl_SetSingleStyle(_obj, style, add)
        }
    }
    fn SetTextColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxListCtrl_SetTextColour(_obj, col)
        }
    }
    fn SetWindowStyleFlag(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxListCtrl_SetWindowStyleFlag(_obj, style)
        }
    }
    fn SortItems(_obj: *u8 /* void* */, fn_: *u8 /* void* */, eif_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SortItems(_obj, fn_, eif_obj)
        }
    }
    fn UpdateStyle(_obj: *u8 /* void* */) {
        unsafe {
            wxListCtrl_UpdateStyle(_obj)
        }
    }
}
trait wxThinSplitterWindow {
    // missing: wxThinSplitterWindow_Create
    // missing: wxThinSplitterWindow_DrawSash
    // missing: wxThinSplitterWindow_SashHitTest
    // missing: wxThinSplitterWindow_SizeWindows
}
trait wxGridCellWorker {
}
trait wxWindowCreateEvent {
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindowCreateEvent_GetWindow(_obj)
        }
    }
}
trait wxcHtmlEvent {
    fn GetMouseEvent(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetMouseEvent(self_)
        }
    }
    fn GetHtmlCell(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetHtmlCell(self_)
        }
    }
    fn GetHtmlCellId(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetHtmlCellId(self_)
        }
    }
    fn GetHref(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetHref(self_)
        }
    }
    fn GetTarget(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetTarget(self_)
        }
    }
    fn GetLogicalPosition(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetLogicalPosition(self_)
        }
    }
}
trait wxAutomationObject {
}
trait wxPageSetupDialog {
    fn Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialog_Create(parent, data)
        }
    }
    fn GetPageSetupData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialog_GetPageSetupData(_obj, _ref)
        }
    }
}
trait wxMultiCellSizer {
    // missing: wxMultiCellSizer_CalcMin
    // missing: wxMultiCellSizer_Create
    // missing: wxMultiCellSizer_Delete
    // missing: wxMultiCellSizer_EnableGridLines
    // missing: wxMultiCellSizer_RecalcSizes
    // missing: wxMultiCellSizer_SetColumnWidth
    // missing: wxMultiCellSizer_SetDefaultCellSize
    // missing: wxMultiCellSizer_SetGridPen
    // missing: wxMultiCellSizer_SetRowHeight
}
trait wxPaintDC {
    fn Create(win: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPaintDC_Create(win)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPaintDC_Delete(_obj)
        }
    }
}
trait wxToolBarBase {
}
trait wxVariant {
}
trait wxSpinEvent {
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinEvent_GetPosition(_obj)
        }
    }
    fn SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxSpinEvent_SetPosition(_obj, pos)
        }
    }
}
trait wxLogStderr {
}
trait wxDocMDIParentFrame {
}
trait wxIPV4address {
}
trait wxPaintEvent {
}
trait wxSocketServer {
}
trait wxString {
}
trait wxBitmapToggleButton {
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmapToggleButton_Create(parent, id, _bmp, arg0, arg1, arg2, arg3, style)
        }
    }
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxBitmapToggleButton_Enable(_obj, enable)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBitmapToggleButton_GetValue(_obj)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, state: bool /* bool */) {
        unsafe {
            wxBitmapToggleButton_SetValue(_obj, state)
        }
    }
    fn SetBitmapLabel(_obj: *u8 /* void* */, _bmp: *u8 /* void* */) {
        unsafe {
            wxBitmapToggleButton_SetBitmapLabel(_obj, _bmp)
        }
    }
}
trait wxDbSqlTypeInfo {
}
trait wxFontDialog {
    fn Create(_prt: *u8 /* void* */, fnt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFontDialog_Create(_prt, fnt)
        }
    }
    fn GetFontData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxFontDialog_GetFontData(_obj, _ref)
        }
    }
}
trait wxOutputStream {
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxOutputStream_Delete(_obj)
        }
    }
    fn LastWrite(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxOutputStream_LastWrite(_obj)
        }
    }
    fn PutC(_obj: *u8 /* void* */, c: wchar_t /* wchar_t */) {
        unsafe {
            wxOutputStream_PutC(_obj, c)
        }
    }
    fn Seek(_obj: *u8 /* void* */, pos: c_int /* int */, mode: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxOutputStream_Seek(_obj, pos, mode)
        }
    }
    fn Sync(_obj: *u8 /* void* */) {
        unsafe {
            wxOutputStream_Sync(_obj)
        }
    }
    fn Tell(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxOutputStream_Tell(_obj)
        }
    }
    fn Write(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */) {
        unsafe {
            wxOutputStream_Write(_obj, buffer, size)
        }
    }
}
trait wxTextAttr {
}
trait wxHelpController {
}
trait wxDbColDef {
}
trait wxMaximizeEvent {
}
trait wxDocMDIChildFrame {
}
trait wxAcceleratorEntry {
    fn Create(flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxAcceleratorEntry_Create(flags, keyCode, cmd)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxAcceleratorEntry_Delete(_obj)
        }
    }
    fn GetCommand(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxAcceleratorEntry_GetCommand(_obj)
        }
    }
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxAcceleratorEntry_GetFlags(_obj)
        }
    }
    fn GetKeyCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxAcceleratorEntry_GetKeyCode(_obj)
        }
    }
    fn Set(_obj: *u8 /* void* */, flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */) {
        unsafe {
            wxAcceleratorEntry_Set(_obj, flags, keyCode, cmd)
        }
    }
}
trait wxDialUpManager {
    // missing: wxDialUpManager_CancelDialing
    // missing: wxDialUpManager_Create
    // missing: wxDialUpManager_Delete
    // missing: wxDialUpManager_Dial
    // missing: wxDialUpManager_DisableAutoCheckOnlineStatus
    // missing: wxDialUpManager_EnableAutoCheckOnlineStatus
    // missing: wxDialUpManager_GetISPNames
    // missing: wxDialUpManager_HangUp
    // missing: wxDialUpManager_IsAlwaysOnline
    // missing: wxDialUpManager_IsDialing
    // missing: wxDialUpManager_IsOk
    // missing: wxDialUpManager_IsOnline
    // missing: wxDialUpManager_SetConnectCommand
    // missing: wxDialUpManager_SetOnlineStatus
    // missing: wxDialUpManager_SetWellKnownHost
}
trait wxDynToolInfo {
    // missing: wxDynToolInfo_Index
    // missing: wxDynToolInfo_RealSize
    // missing: wxDynToolInfo_pToolWnd
}
trait wxDocTemplate {
}
trait wxPreviewControlBar {
}
trait ELJPreviewFrame {
    fn Create(_obj: *u8 /* void* */, _init: *u8 /* void* */, _create_canvas: *u8 /* void* */, _create_toolbar: *u8 /* void* */, preview: *u8 /* void* */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent, title, arg0, arg1, arg2, arg3, style)
        }
    }
    fn GetControlBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewFrame_GetControlBar(_obj)
        }
    }
    fn GetPreviewCanvas(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewFrame_GetPreviewCanvas(_obj)
        }
    }
    fn GetPrintPreview(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewFrame_GetPrintPreview(_obj)
        }
    }
    fn Initialize(_obj: *u8 /* void* */) {
        unsafe {
            ELJPreviewFrame_Initialize(_obj)
        }
    }
    fn SetControlBar(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            ELJPreviewFrame_SetControlBar(_obj, obj)
        }
    }
    fn SetPreviewCanvas(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            ELJPreviewFrame_SetPreviewCanvas(_obj, obj)
        }
    }
    fn SetPrintPreview(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            ELJPreviewFrame_SetPrintPreview(_obj, obj)
        }
    }
}
trait wxBitmapHandler {
}
trait wxGaugeMSW {
}
trait cbDimInfo {
    // missing: cbDimInfo_Assign
    // missing: cbDimInfo_Create
    // missing: cbDimInfo_CreateDefault
    // missing: cbDimInfo_CreateWithHandler
    // missing: cbDimInfo_CreateWithInfo
    // missing: cbDimInfo_Delete
    // missing: cbDimInfo_GetDimHandler
}
trait wxMDIClientWindow {
}
trait wxDataObjectSimple {
}
trait wxDialog {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDialog_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn EndModal(_obj: *u8 /* void* */, retCode: c_int /* int */) {
        unsafe {
            wxDialog_EndModal(_obj, retCode)
        }
    }
    fn GetReturnCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDialog_GetReturnCode(_obj)
        }
    }
    fn IsModal(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialog_IsModal(_obj)
        }
    }
    fn SetReturnCode(_obj: *u8 /* void* */, returnCode: c_int /* int */) {
        unsafe {
            wxDialog_SetReturnCode(_obj, returnCode)
        }
    }
    fn ShowModal(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDialog_ShowModal(_obj)
        }
    }
}
trait wxImageHandler {
}
trait wxClosure {
    fn Create(_fun_CEvent: *u8 /* void* */, _data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClosure_Create(_fun_CEvent, _data)
        }
    }
    fn GetData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClosure_GetData(_obj)
        }
    }
    fn wxEvtHandler_GetClosure(_obj: *u8 /* void* */, id: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_GetClosure(_obj, id, type_)
        }
    }
    fn wxEvtHandler_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_GetClientClosure(_obj)
        }
    }
    fn wxEvtHandler_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_SetClientClosure(_obj, closure)
        }
    }
    fn wxObject_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxObject_GetClientClosure(_obj)
        }
    }
    fn wxObject_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxObject_SetClientClosure(_obj, closure)
        }
    }
}
trait wxIndividualLayoutConstraint {
    fn Above(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_Above(_obj, sibling, marg)
        }
    }
    fn Absolute(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_Absolute(_obj, val)
        }
    }
    fn AsIs(_obj: *u8 /* void* */) {
        unsafe {
            wxIndividualLayoutConstraint_AsIs(_obj)
        }
    }
    fn Below(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_Below(_obj, sibling, marg)
        }
    }
    fn GetDone(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIndividualLayoutConstraint_GetDone(_obj)
        }
    }
    fn GetEdge(_obj: *u8 /* void* */, which: c_int /* int */, thisWin: *u8 /* void* */, other: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetEdge(_obj, which, thisWin, other)
        }
    }
    fn GetMargin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetMargin(_obj)
        }
    }
    fn GetMyEdge(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetMyEdge(_obj)
        }
    }
    fn GetOtherEdge(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetOtherEdge(_obj)
        }
    }
    fn GetOtherWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxIndividualLayoutConstraint_GetOtherWindow(_obj)
        }
    }
    fn GetPercent(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetPercent(_obj)
        }
    }
    fn GetRelationship(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetRelationship(_obj)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetValue(_obj)
        }
    }
    fn LeftOf(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_LeftOf(_obj, sibling, marg)
        }
    }
    fn PercentOf(_obj: *u8 /* void* */, otherW: *u8 /* void* */, wh: c_int /* int */, per: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_PercentOf(_obj, otherW, wh, per)
        }
    }
    fn ResetIfWin(_obj: *u8 /* void* */, otherW: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIndividualLayoutConstraint_ResetIfWin(_obj, otherW)
        }
    }
    fn RightOf(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_RightOf(_obj, sibling, marg)
        }
    }
    fn SameAs(_obj: *u8 /* void* */, otherW: *u8 /* void* */, edge: c_int /* int */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SameAs(_obj, otherW, edge, marg)
        }
    }
    fn SatisfyConstraint(_obj: *u8 /* void* */, constraints: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIndividualLayoutConstraint_SatisfyConstraint(_obj, constraints, win)
        }
    }
    fn Set(_obj: *u8 /* void* */, rel: c_int /* int */, otherW: *u8 /* void* */, otherE: c_int /* int */, val: c_int /* int */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_Set(_obj, rel, otherW, otherE, val, marg)
        }
    }
    fn SetDone(_obj: *u8 /* void* */, d: bool /* bool */) {
        unsafe {
            wxIndividualLayoutConstraint_SetDone(_obj, d)
        }
    }
    fn SetEdge(_obj: *u8 /* void* */, which: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SetEdge(_obj, which)
        }
    }
    fn SetMargin(_obj: *u8 /* void* */, m: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SetMargin(_obj, m)
        }
    }
    fn SetRelationship(_obj: *u8 /* void* */, r: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SetRelationship(_obj, r)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SetValue(_obj, v)
        }
    }
    fn Unconstrained(_obj: *u8 /* void* */) {
        unsafe {
            wxIndividualLayoutConstraint_Unconstrained(_obj)
        }
    }
}
trait wxTempFile {
}
trait cbFloatedBarWindow {
    // missing: cbFloatedBarWindow_Create
    // missing: cbFloatedBarWindow_GetBar
    // missing: cbFloatedBarWindow_PositionFloatedWnd
    // missing: cbFloatedBarWindow_SetBar
    // missing: cbFloatedBarWindow_SetLayout
}
trait cbLeftUpEvent {
    // missing: cbLeftUpEvent_Pos
}
trait wxHtmlTagsModule {
}
trait wxMirrorDC {
    fn Create(dc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMirrorDC_Create(dc)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMirrorDC_Delete(_obj)
        }
    }
}
trait wxLocale {
    fn AddCatalog(_obj: *u8 /* void* */, szDomain: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxLocale_AddCatalog(_obj, szDomain)
        }
    }
    fn AddCatalogLookupPathPrefix(_obj: *u8 /* void* */, prefix: *u8 /* void* */) {
        unsafe {
            wxLocale_AddCatalogLookupPathPrefix(_obj, prefix)
        }
    }
    fn Create(_name: c_int /* int */, _flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxLocale_Create(_name, _flags)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxLocale_Delete(_obj)
        }
    }
    fn GetLocale(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLocale_GetLocale(_obj)
        }
    }
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLocale_GetName(_obj)
        }
    }
    fn GetString(_obj: *u8 /* void* */, szOrigString: *u8 /* void* */, szDomain: *u8 /* void* */) -> *wchar_t /* wchar_t* */ {
        unsafe {
            wxLocale_GetString(_obj, szOrigString, szDomain)
        }
    }
    fn IsLoaded(_obj: *u8 /* void* */, szDomain: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLocale_IsLoaded(_obj, szDomain)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLocale_IsOk(_obj)
        }
    }
}
trait wxFindDialogEvent {
    fn GetFindString(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFindDialogEvent_GetFindString(_obj, _ref)
        }
    }
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFindDialogEvent_GetFlags(_obj)
        }
    }
    fn GetReplaceString(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFindDialogEvent_GetReplaceString(_obj, _ref)
        }
    }
}
trait wxDb {
}
trait wxQueryField {
}
trait wxHtmlDCRenderer {
}
trait wxGauge95 {
}
trait wxDC {
    fn Blit(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, source: *u8 /* void* */, arg4: c_int /* int */, arg5: c_int /* int */, rop: c_int /* int */, useMask: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxDC_Blit(_obj, arg0, arg1, arg2, arg3, source, arg4, arg5, rop, useMask)
        }
    }
    fn CalcBoundingBox(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_CalcBoundingBox(_obj, arg0, arg1)
        }
    }
    fn CanDrawBitmap(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_CanDrawBitmap(_obj)
        }
    }
    fn CanGetTextExtent(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_CanGetTextExtent(_obj)
        }
    }
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_Clear(_obj)
        }
    }
    fn ComputeScaleAndOrigin(obj: *u8 /* void* */) {
        unsafe {
            wxDC_ComputeScaleAndOrigin(obj)
        }
    }
    fn CrossHair(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_CrossHair(_obj, arg0, arg1)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_Delete(_obj)
        }
    }
    fn DestroyClippingRegion(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_DestroyClippingRegion(_obj)
        }
    }
    fn DeviceToLogicalX(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_DeviceToLogicalX(_obj, x)
        }
    }
    fn DeviceToLogicalXRel(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_DeviceToLogicalXRel(_obj, x)
        }
    }
    fn DeviceToLogicalY(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_DeviceToLogicalY(_obj, y)
        }
    }
    fn DeviceToLogicalYRel(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_DeviceToLogicalYRel(_obj, y)
        }
    }
    fn DrawArc(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: c_int /* int */) {
        unsafe {
            wxDC_DrawArc(_obj, arg0, arg1, arg2, arg3, arg4, arg5)
        }
    }
    fn DrawBitmap(_obj: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, useMask: bool /* bool */) {
        unsafe {
            wxDC_DrawBitmap(_obj, bmp, arg0, arg1, useMask)
        }
    }
    fn DrawCheckMark(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_DrawCheckMark(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn DrawCircle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, radius: c_int /* int */) {
        unsafe {
            wxDC_DrawCircle(_obj, arg0, arg1, radius)
        }
    }
    fn DrawEllipse(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_DrawEllipse(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn DrawEllipticArc(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, sa: c_double /* double */, ea: c_double /* double */) {
        unsafe {
            wxDC_DrawEllipticArc(_obj, arg0, arg1, arg2, arg3, sa, ea)
        }
    }
    fn DrawIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_DrawIcon(_obj, icon, arg0, arg1)
        }
    }
    fn DrawLabel(_obj: *u8 /* void* */, str: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, align: c_int /* int */, indexAccel: c_int /* int */) {
        unsafe {
            wxDC_DrawLabel(_obj, str, arg0, arg1, arg2, arg3, align, indexAccel)
        }
    }
    fn DrawLabelBitmap(_obj: *u8 /* void* */, str: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, align: c_int /* int */, indexAccel: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDC_DrawLabelBitmap(_obj, str, bmp, arg0, arg1, arg2, arg3, align, indexAccel)
        }
    }
    fn DrawLine(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_DrawLine(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn DrawLines(_obj: *u8 /* void* */, n: c_int /* int */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_DrawLines(_obj, n, x, y, arg0, arg1)
        }
    }
    fn DrawPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_DrawPoint(_obj, arg0, arg1)
        }
    }
    fn DrawPolygon(_obj: *u8 /* void* */, n: c_int /* int */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, fillStyle: c_int /* int */) {
        unsafe {
            wxDC_DrawPolygon(_obj, n, x, y, arg0, arg1, fillStyle)
        }
    }
    fn DrawPolyPolygon(_obj: *u8 /* void* */, n: c_int /* int */, count: *u8 /* void* */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, fillStyle: c_int /* int */) {
        unsafe {
            wxDC_DrawPolyPolygon(_obj, n, count, x, y, arg0, arg1, fillStyle)
        }
    }
    fn DrawRectangle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_DrawRectangle(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn DrawRotatedText(_obj: *u8 /* void* */, text: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, angle: c_double /* double */) {
        unsafe {
            wxDC_DrawRotatedText(_obj, text, arg0, arg1, angle)
        }
    }
    fn DrawRoundedRectangle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, radius: c_double /* double */) {
        unsafe {
            wxDC_DrawRoundedRectangle(_obj, arg0, arg1, arg2, arg3, radius)
        }
    }
    fn DrawText(_obj: *u8 /* void* */, text: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_DrawText(_obj, text, arg0, arg1)
        }
    }
    fn EndDoc(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_EndDoc(_obj)
        }
    }
    fn EndPage(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_EndPage(_obj)
        }
    }
    fn FloodFill(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxDC_FloodFill(_obj, arg0, arg1, col, style)
        }
    }
    fn GetBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetBackground(_obj, _ref)
        }
    }
    fn GetBackgroundMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetBackgroundMode(_obj)
        }
    }
    fn GetBrush(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetBrush(_obj, _ref)
        }
    }
    fn GetCharHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetCharHeight(_obj)
        }
    }
    fn GetCharWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetCharWidth(_obj)
        }
    }
    fn GetClippingBox(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxDC_GetClippingBox(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn GetDepth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetDepth(_obj)
        }
    }
    fn GetDeviceOrigin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxDC_GetDeviceOrigin(_obj, arg0, arg1)
        }
    }
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetFont(_obj, _ref)
        }
    }
    fn GetLogicalFunction(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetLogicalFunction(_obj)
        }
    }
    fn GetLogicalOrigin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxDC_GetLogicalOrigin(_obj, arg0, arg1)
        }
    }
    fn GetLogicalScale(_obj: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */) {
        unsafe {
            wxDC_GetLogicalScale(_obj, arg0, arg1)
        }
    }
    fn GetMapMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetMapMode(_obj)
        }
    }
    fn GetPPI(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDC_GetPPI(_obj)
        }
    }
    fn GetPen(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetPen(_obj, _ref)
        }
    }
    fn GetPixel(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_GetPixel(_obj, arg0, arg1, col)
        }
    }
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDC_GetSize(_obj)
        }
    }
    fn GetSizeMM(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDC_GetSizeMM(_obj)
        }
    }
    fn GetTextBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetTextBackground(_obj, _ref)
        }
    }
    fn GetTextExtent(self_: *u8 /* void* */, string: *u8 /* void* */, w: *u8 /* void* */, h: *u8 /* void* */, descent: *u8 /* void* */, externalLeading: *u8 /* void* */, theFont: *u8 /* void* */) {
        unsafe {
            wxDC_GetTextExtent(self_, string, w, h, descent, externalLeading, theFont)
        }
    }
    fn GetMultiLineTextExtent(self_: *u8 /* void* */, string: *u8 /* void* */, w: *u8 /* void* */, h: *u8 /* void* */, heightLine: *u8 /* void* */, theFont: *u8 /* void* */) {
        unsafe {
            wxDC_GetMultiLineTextExtent(self_, string, w, h, heightLine, theFont)
        }
    }
    fn GetTextForeground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetTextForeground(_obj, _ref)
        }
    }
    fn GetUserScale(_obj: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */) {
        unsafe {
            wxDC_GetUserScale(_obj, arg0, arg1)
        }
    }
    fn LogicalToDeviceX(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_LogicalToDeviceX(_obj, x)
        }
    }
    fn LogicalToDeviceXRel(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_LogicalToDeviceXRel(_obj, x)
        }
    }
    fn LogicalToDeviceY(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_LogicalToDeviceY(_obj, y)
        }
    }
    fn LogicalToDeviceYRel(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_LogicalToDeviceYRel(_obj, y)
        }
    }
    fn MaxX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_MaxX(_obj)
        }
    }
    fn MaxY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_MaxY(_obj)
        }
    }
    fn MinX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_MinX(_obj)
        }
    }
    fn MinY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_MinY(_obj)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_IsOk(_obj)
        }
    }
    fn ResetBoundingBox(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_ResetBoundingBox(_obj)
        }
    }
    fn SetAxisOrientation(_obj: *u8 /* void* */, xLeftRight: bool /* bool */, yBottomUp: bool /* bool */) {
        unsafe {
            wxDC_SetAxisOrientation(_obj, xLeftRight, yBottomUp)
        }
    }
    fn SetBackground(_obj: *u8 /* void* */, brush: *u8 /* void* */) {
        unsafe {
            wxDC_SetBackground(_obj, brush)
        }
    }
    fn SetBackgroundMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxDC_SetBackgroundMode(_obj, mode)
        }
    }
    fn SetBrush(_obj: *u8 /* void* */, brush: *u8 /* void* */) {
        unsafe {
            wxDC_SetBrush(_obj, brush)
        }
    }
    fn SetClippingRegion(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_SetClippingRegion(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn SetClippingRegionFromRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) {
        unsafe {
            wxDC_SetClippingRegionFromRegion(_obj, region)
        }
    }
    fn SetDeviceOrigin(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_SetDeviceOrigin(_obj, arg0, arg1)
        }
    }
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxDC_SetFont(_obj, font)
        }
    }
    fn SetLogicalFunction(_obj: *u8 /* void* */, function: c_int /* int */) {
        unsafe {
            wxDC_SetLogicalFunction(_obj, function)
        }
    }
    fn SetLogicalOrigin(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_SetLogicalOrigin(_obj, arg0, arg1)
        }
    }
    fn SetLogicalScale(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */) {
        unsafe {
            wxDC_SetLogicalScale(_obj, x, y)
        }
    }
    fn SetMapMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxDC_SetMapMode(_obj, mode)
        }
    }
    fn SetPalette(_obj: *u8 /* void* */, palette: *u8 /* void* */) {
        unsafe {
            wxDC_SetPalette(_obj, palette)
        }
    }
    fn SetPen(_obj: *u8 /* void* */, pen: *u8 /* void* */) {
        unsafe {
            wxDC_SetPen(_obj, pen)
        }
    }
    fn SetTextBackground(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxDC_SetTextBackground(_obj, colour)
        }
    }
    fn SetTextForeground(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxDC_SetTextForeground(_obj, colour)
        }
    }
    fn SetUserScale(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */) {
        unsafe {
            wxDC_SetUserScale(_obj, x, y)
        }
    }
    fn StartDoc(_obj: *u8 /* void* */, msg: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_StartDoc(_obj, msg)
        }
    }
    fn StartPage(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_StartPage(_obj)
        }
    }
}
trait ELJTextValidator {
    fn Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */, _txt: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJTextValidator_Create(_obj, _fnc, _txt, _stl)
        }
    }
}
trait wxHelpProvider {
    fn AddHelp(_obj: *u8 /* void* */, window: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxHelpProvider_AddHelp(_obj, window, text)
        }
    }
    fn AddHelpById(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */) {
        unsafe {
            wxHelpProvider_AddHelpById(_obj, id, text)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxHelpProvider_Delete(_obj)
        }
    }
    fn Get() -> *u8 /* void* */ {
        unsafe {
            wxHelpProvider_Get()
        }
    }
    fn GetHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpProvider_GetHelp(_obj, window)
        }
    }
    fn RemoveHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxHelpProvider_RemoveHelp(_obj, window)
        }
    }
    fn Set(helpProvider: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpProvider_Set(helpProvider)
        }
    }
    fn ShowHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHelpProvider_ShowHelp(_obj, window)
        }
    }
}
trait wxHashMap {
}
trait wxCondition {
    // missing: wxCondition_Broadcast
    // missing: wxCondition_Create
    // missing: wxCondition_Delete
    // missing: wxCondition_Signal
    // missing: wxCondition_Wait
    // missing: wxCondition_WaitFor
}
trait wxGridTableBase {
}
trait wxApp {
}
trait wxCommand {
}
trait wxStreamBase {
    fn GetLastError(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStreamBase_GetLastError(_obj)
        }
    }
    fn GetSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStreamBase_GetSize(_obj)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStreamBase_IsOk(_obj)
        }
    }
}
trait wxMDIParentFrame {
    fn ActivateNext(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_ActivateNext(_obj)
        }
    }
    fn ActivatePrevious(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_ActivatePrevious(_obj)
        }
    }
    fn ArrangeIcons(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_ArrangeIcons(_obj)
        }
    }
    fn Cascade(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_Cascade(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetActiveChild(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_GetActiveChild(_obj)
        }
    }
    fn GetClientWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_GetClientWindow(_obj)
        }
    }
    fn GetWindowMenu(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_GetWindowMenu(_obj)
        }
    }
    fn OnCreateClient(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_OnCreateClient(_obj)
        }
    }
    fn SetWindowMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_SetWindowMenu(_obj, menu)
        }
    }
    fn Tile(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_Tile(_obj)
        }
    }
}
trait wxFilterOutputStream {
}
trait wxClientDC {
    fn Create(win: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClientDC_Create(win)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxClientDC_Delete(_obj)
        }
    }
}
trait wxWizardPage {
}
trait wxTextCtrl {
    fn AppendText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_AppendText(_obj, text)
        }
    }
    fn CanCopy(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanCopy(_obj)
        }
    }
    fn CanCut(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanCut(_obj)
        }
    }
    fn CanPaste(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanPaste(_obj)
        }
    }
    fn CanRedo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanRedo(_obj)
        }
    }
    fn CanUndo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanUndo(_obj)
        }
    }
    fn ChangeValue(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_ChangeValue(_obj, text)
        }
    }
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Clear(_obj)
        }
    }
    fn Copy(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Copy(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn Cut(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Cut(_obj)
        }
    }
    fn DiscardEdits(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_DiscardEdits(_obj)
        }
    }
    fn GetInsertionPoint(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxTextCtrl_GetInsertionPoint(_obj)
        }
    }
    fn GetLastPosition(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxTextCtrl_GetLastPosition(_obj)
        }
    }
    fn GetLineLength(_obj: *u8 /* void* */, lineNo: c_long /* long */) -> c_int /* int */ {
        unsafe {
            wxTextCtrl_GetLineLength(_obj, lineNo)
        }
    }
    fn GetLineText(_obj: *u8 /* void* */, lineNo: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetLineText(_obj, lineNo)
        }
    }
    fn GetNumberOfLines(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTextCtrl_GetNumberOfLines(_obj)
        }
    }
    fn GetSelection(_obj: *u8 /* void* */, from: *u8 /* void* */, to: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_GetSelection(_obj, from, to)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetValue(_obj)
        }
    }
    fn IsEditable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_IsEditable(_obj)
        }
    }
    fn IsModified(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_IsModified(_obj)
        }
    }
    fn LoadFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_LoadFile(_obj, file)
        }
    }
    fn Paste(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Paste(_obj)
        }
    }
    fn PositionToXY(_obj: *u8 /* void* */, pos: c_long /* long */, x: *c_long /* long* */, y: *c_long /* long* */) -> c_int /* int */ {
        unsafe {
            wxTextCtrl_PositionToXY(_obj, pos, x, y)
        }
    }
    fn Redo(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Redo(_obj)
        }
    }
    fn Remove(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */) {
        unsafe {
            wxTextCtrl_Remove(_obj, from, to)
        }
    }
    fn Replace(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */, value: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Replace(_obj, from, to, value)
        }
    }
    fn SaveFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_SaveFile(_obj, file)
        }
    }
    fn SetEditable(_obj: *u8 /* void* */, editable: bool /* bool */) {
        unsafe {
            wxTextCtrl_SetEditable(_obj, editable)
        }
    }
    fn SetInsertionPoint(_obj: *u8 /* void* */, pos: c_long /* long */) {
        unsafe {
            wxTextCtrl_SetInsertionPoint(_obj, pos)
        }
    }
    fn SetInsertionPointEnd(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_SetInsertionPointEnd(_obj)
        }
    }
    fn SetSelection(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */) {
        unsafe {
            wxTextCtrl_SetSelection(_obj, from, to)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, value: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_SetValue(_obj, value)
        }
    }
    fn ShowPosition(_obj: *u8 /* void* */, pos: c_long /* long */) {
        unsafe {
            wxTextCtrl_ShowPosition(_obj, pos)
        }
    }
    fn Undo(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Undo(_obj)
        }
    }
    fn WriteText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_WriteText(_obj, text)
        }
    }
    fn XYToPosition(_obj: *u8 /* void* */, arg0: c_long /* long */, arg1: c_long /* long */) -> c_long /* long */ {
        unsafe {
            wxTextCtrl_XYToPosition(_obj, arg0, arg1)
        }
    }
}
trait wxConnection {
}
trait wxTimerEx {
    fn Connect(_obj: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxTimerEx_Connect(_obj, closure)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxTimerEx_Create()
        }
    }
    fn GetClosure(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTimerEx_GetClosure(_obj)
        }
    }
    fn wxMenu_AppendRadioItem(self_: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */) {
        unsafe {
            wxMenu_AppendRadioItem(self_, id, text, help)
        }
    }
    fn wxMenuItem_CreateSeparator() -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_CreateSeparator()
        }
    }
    fn wxMenuItem_CreateEx(id: c_int /* int */, label: *u8 /* void* */, help: *u8 /* void* */, itemkind: c_int /* int */, submenu: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_CreateEx(id, label, help, itemkind, submenu)
        }
    }
    fn wxToolBar_AddTool2(_obj: *u8 /* void* */, toolId: c_int /* int */, label: *u8 /* void* */, bmp: *u8 /* void* */, bmpDisabled: *u8 /* void* */, itemKind: c_int /* int */, shortHelp: *u8 /* void* */, longHelp: *u8 /* void* */) {
        unsafe {
            wxToolBar_AddTool2(_obj, toolId, label, bmp, bmpDisabled, itemKind, shortHelp, longHelp)
        }
    }
    fn wxProgressDialog_Create(title: *u8 /* void* */, message: *u8 /* void* */, max: c_int /* int */, parent: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxProgressDialog_Create(title, message, max, parent, style)
        }
    }
    fn wxProgressDialog_Update(obj: *u8 /* void* */, value: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxProgressDialog_Update(obj, value)
        }
    }
    fn wxProgressDialog_UpdateWithMessage(obj: *u8 /* void* */, value: c_int /* int */, message: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProgressDialog_UpdateWithMessage(obj, value, message)
        }
    }
    fn wxProgressDialog_Resume(obj: *u8 /* void* */) {
        unsafe {
            wxProgressDialog_Resume(obj)
        }
    }
    fn wxVersionNumber() -> c_int /* int */ {
        unsafe {
            wxVersionNumber()
        }
    }
    fn wxIsDefined(s: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxIsDefined(s)
        }
    }
}
trait wxFrame {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn CreateStatusBar(_obj: *u8 /* void* */, number: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_CreateStatusBar(_obj, number, style)
        }
    }
    fn CreateToolBar(_obj: *u8 /* void* */, style: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_CreateToolBar(_obj, style)
        }
    }
    fn GetClientAreaOrigin_left(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFrame_GetClientAreaOrigin_left(_obj)
        }
    }
    fn GetClientAreaOrigin_top(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFrame_GetClientAreaOrigin_top(_obj)
        }
    }
    fn GetMenuBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_GetMenuBar(_obj)
        }
    }
    fn GetStatusBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_GetStatusBar(_obj)
        }
    }
    fn GetToolBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_GetToolBar(_obj)
        }
    }
    fn Restore(_obj: *u8 /* void* */) {
        unsafe {
            wxFrame_Restore(_obj)
        }
    }
    fn SetMenuBar(_obj: *u8 /* void* */, menubar: *u8 /* void* */) {
        unsafe {
            wxFrame_SetMenuBar(_obj, menubar)
        }
    }
    fn SetStatusBar(_obj: *u8 /* void* */, statBar: *u8 /* void* */) {
        unsafe {
            wxFrame_SetStatusBar(_obj, statBar)
        }
    }
    fn SetStatusText(_obj: *u8 /* void* */, _txt: *u8 /* void* */, _number: c_int /* int */) {
        unsafe {
            wxFrame_SetStatusText(_obj, _txt, _number)
        }
    }
    fn SetStatusWidths(_obj: *u8 /* void* */, _n: c_int /* int */, _widths_field: *u8 /* void* */) {
        unsafe {
            wxFrame_SetStatusWidths(_obj, _n, _widths_field)
        }
    }
    fn SetToolBar(_obj: *u8 /* void* */, _toolbar: *u8 /* void* */) {
        unsafe {
            wxFrame_SetToolBar(_obj, _toolbar)
        }
    }
}
trait wxDateTime {
    fn AddDate(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_AddDate(_obj, diff, _ref)
        }
    }
    fn AddDateValues(_obj: *u8 /* void* */, _yrs: c_int /* int */, _mnt: c_int /* int */, _wek: c_int /* int */, _day: c_int /* int */) {
        unsafe {
            wxDateTime_AddDateValues(_obj, _yrs, _mnt, _wek, _day)
        }
    }
    fn AddTime(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_AddTime(_obj, diff, _ref)
        }
    }
    fn AddTimeValues(_obj: *u8 /* void* */, _hrs: c_int /* int */, _min: c_int /* int */, _sec: c_int /* int */, _mls: c_int /* int */) {
        unsafe {
            wxDateTime_AddTimeValues(_obj, _hrs, _min, _sec, _mls)
        }
    }
    fn ConvertYearToBC(year: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_ConvertYearToBC(year)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxDateTime_Create()
        }
    }
    fn Format(_obj: *u8 /* void* */, format: *u8 /* void* */, tz: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_Format(_obj, format, tz)
        }
    }
    fn FormatDate(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_FormatDate(_obj)
        }
    }
    fn FormatISODate(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_FormatISODate(_obj)
        }
    }
    fn FormatISOTime(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_FormatISOTime(_obj)
        }
    }
    fn FormatTime(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_FormatTime(_obj)
        }
    }
    fn GetAmString() -> *u8 /* void* */ {
        unsafe {
            wxDateTime_GetAmString()
        }
    }
    fn GetBeginDST(year: c_int /* int */, country: c_int /* int */, dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetBeginDST(year, country, dt)
        }
    }
    fn GetCentury(year: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetCentury(year)
        }
    }
    fn GetCountry() -> c_int /* int */ {
        unsafe {
            wxDateTime_GetCountry()
        }
    }
    fn GetCurrentMonth(cal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetCurrentMonth(cal)
        }
    }
    fn GetCurrentYear(cal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetCurrentYear(cal)
        }
    }
    fn GetDay(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetDay(_obj, tz)
        }
    }
    fn GetDayOfYear(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetDayOfYear(_obj, tz)
        }
    }
    fn GetEndDST(year: c_int /* int */, country: c_int /* int */, dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetEndDST(year, country, dt)
        }
    }
    fn GetHour(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetHour(_obj, tz)
        }
    }
    fn GetLastMonthDay(_obj: *u8 /* void* */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetLastMonthDay(_obj, month, year, _ref)
        }
    }
    fn GetLastWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetLastWeekDay(_obj, weekday, month, year, _ref)
        }
    }
    fn GetMillisecond(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetMillisecond(_obj, tz)
        }
    }
    fn GetMinute(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetMinute(_obj, tz)
        }
    }
    fn GetMonth(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetMonth(_obj, tz)
        }
    }
    fn GetMonthName(month: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_GetMonthName(month, flags)
        }
    }
    fn GetNextWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetNextWeekDay(_obj, weekday, _ref)
        }
    }
    fn GetNumberOfDays(year: c_int /* int */, cal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetNumberOfDays(year, cal)
        }
    }
    fn GetNumberOfDaysMonth(month: c_int /* int */, year: c_int /* int */, cal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetNumberOfDaysMonth(month, year, cal)
        }
    }
    fn GetPmString() -> *u8 /* void* */ {
        unsafe {
            wxDateTime_GetPmString()
        }
    }
    fn GetPrevWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetPrevWeekDay(_obj, weekday, _ref)
        }
    }
    fn GetSecond(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetSecond(_obj, tz)
        }
    }
    fn GetTicks(_obj: *u8 /* void* */) -> time_t /* time_t */ {
        unsafe {
            wxDateTime_GetTicks(_obj)
        }
    }
    fn GetTimeNow() -> c_int /* int */ {
        unsafe {
            wxDateTime_GetTimeNow()
        }
    }
    fn GetValue(_obj: *u8 /* void* */, hi_long: *u8 /* void* */, lo_long: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetValue(_obj, hi_long, lo_long)
        }
    }
    fn GetWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, n: c_int /* int */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetWeekDay(_obj, weekday, n, month, year, _ref)
        }
    }
    fn GetWeekDayInSameWeek(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetWeekDayInSameWeek(_obj, weekday, _ref)
        }
    }
    fn GetWeekDayName(weekday: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_GetWeekDayName(weekday, flags)
        }
    }
    fn GetWeekDayTZ(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetWeekDayTZ(_obj, tz)
        }
    }
    fn GetWeekOfMonth(_obj: *u8 /* void* */, flags: c_int /* int */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetWeekOfMonth(_obj, flags, tz)
        }
    }
    fn GetWeekOfYear(_obj: *u8 /* void* */, flags: c_int /* int */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetWeekOfYear(_obj, flags, tz)
        }
    }
    fn GetYear(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetYear(_obj, tz)
        }
    }
    fn IsBetween(_obj: *u8 /* void* */, t1: *u8 /* void* */, t2: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsBetween(_obj, t1, t2)
        }
    }
    fn IsDST(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsDST(_obj, country)
        }
    }
    fn IsDSTApplicable(year: c_int /* int */, country: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsDSTApplicable(year, country)
        }
    }
    fn IsEarlierThan(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsEarlierThan(_obj, datetime)
        }
    }
    fn IsEqualTo(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsEqualTo(_obj, datetime)
        }
    }
    fn IsEqualUpTo(_obj: *u8 /* void* */, dt: *u8 /* void* */, ts: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsEqualUpTo(_obj, dt, ts)
        }
    }
    // missing: wxDateTime_IsGregorianDate
    fn IsLaterThan(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsLaterThan(_obj, datetime)
        }
    }
    fn IsLeapYear(year: c_int /* int */, cal: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsLeapYear(year, cal)
        }
    }
    fn IsSameDate(_obj: *u8 /* void* */, dt: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsSameDate(_obj, dt)
        }
    }
    fn IsSameTime(_obj: *u8 /* void* */, dt: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsSameTime(_obj, dt)
        }
    }
    fn IsStrictlyBetween(_obj: *u8 /* void* */, t1: *u8 /* void* */, t2: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsStrictlyBetween(_obj, t1, t2)
        }
    }
    fn IsValid(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsValid(_obj)
        }
    }
    fn IsWestEuropeanCountry(country: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsWestEuropeanCountry(country)
        }
    }
    fn IsWorkDay(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsWorkDay(_obj, country)
        }
    }
    fn MakeGMT(_obj: *u8 /* void* */, noDST: c_int /* int */) {
        unsafe {
            wxDateTime_MakeGMT(_obj, noDST)
        }
    }
    fn MakeTimezone(_obj: *u8 /* void* */, tz: c_int /* int */, noDST: c_int /* int */) {
        unsafe {
            wxDateTime_MakeTimezone(_obj, tz, noDST)
        }
    }
    fn Now(dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_Now(dt)
        }
    }
    fn ParseDate(_obj: *u8 /* void* */, date: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseDate(_obj, date)
        }
    }
    fn ParseDateTime(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseDateTime(_obj, datetime)
        }
    }
    fn ParseFormat(_obj: *u8 /* void* */, date: *u8 /* void* */, format: *u8 /* void* */, dateDef: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseFormat(_obj, date, format, dateDef)
        }
    }
    fn ParseRfc822Date(_obj: *u8 /* void* */, date: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseRfc822Date(_obj, date)
        }
    }
    fn ParseTime(_obj: *u8 /* void* */, time: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseTime(_obj, time)
        }
    }
    fn ResetTime(_obj: *u8 /* void* */) {
        unsafe {
            wxDateTime_ResetTime(_obj)
        }
    }
    fn Set(_obj: *u8 /* void* */, day: c_int /* int */, month: c_int /* int */, year: c_int /* int */, hour: c_int /* int */, minute: c_int /* int */, second: c_int /* int */, millisec: c_int /* int */) {
        unsafe {
            wxDateTime_Set(_obj, day, month, year, hour, minute, second, millisec)
        }
    }
    fn SetCountry(country: c_int /* int */) {
        unsafe {
            wxDateTime_SetCountry(country)
        }
    }
    fn SetDay(_obj: *u8 /* void* */, day: c_int /* int */) {
        unsafe {
            wxDateTime_SetDay(_obj, day)
        }
    }
    fn SetHour(_obj: *u8 /* void* */, hour: c_int /* int */) {
        unsafe {
            wxDateTime_SetHour(_obj, hour)
        }
    }
    fn SetMillisecond(_obj: *u8 /* void* */, millisecond: c_int /* int */) {
        unsafe {
            wxDateTime_SetMillisecond(_obj, millisecond)
        }
    }
    fn SetMinute(_obj: *u8 /* void* */, minute: c_int /* int */) {
        unsafe {
            wxDateTime_SetMinute(_obj, minute)
        }
    }
    fn SetMonth(_obj: *u8 /* void* */, month: c_int /* int */) {
        unsafe {
            wxDateTime_SetMonth(_obj, month)
        }
    }
    fn SetSecond(_obj: *u8 /* void* */, second: c_int /* int */) {
        unsafe {
            wxDateTime_SetSecond(_obj, second)
        }
    }
    fn SetTime(_obj: *u8 /* void* */, hour: c_int /* int */, minute: c_int /* int */, second: c_int /* int */, millisec: c_int /* int */) {
        unsafe {
            wxDateTime_SetTime(_obj, hour, minute, second, millisec)
        }
    }
    fn SetToCurrent(_obj: *u8 /* void* */) {
        unsafe {
            wxDateTime_SetToCurrent(_obj)
        }
    }
    fn SetToLastMonthDay(_obj: *u8 /* void* */, month: c_int /* int */, year: c_int /* int */) {
        unsafe {
            wxDateTime_SetToLastMonthDay(_obj, month, year)
        }
    }
    fn SetToLastWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, month: c_int /* int */, year: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_SetToLastWeekDay(_obj, weekday, month, year)
        }
    }
    fn SetToNextWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */) {
        unsafe {
            wxDateTime_SetToNextWeekDay(_obj, weekday)
        }
    }
    fn SetToPrevWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */) {
        unsafe {
            wxDateTime_SetToPrevWeekDay(_obj, weekday)
        }
    }
    fn SetToWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, n: c_int /* int */, month: c_int /* int */, year: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_SetToWeekDay(_obj, weekday, n, month, year)
        }
    }
    fn SetToWeekDayInSameWeek(_obj: *u8 /* void* */, weekday: c_int /* int */) {
        unsafe {
            wxDateTime_SetToWeekDayInSameWeek(_obj, weekday)
        }
    }
    fn SetYear(_obj: *u8 /* void* */, year: c_int /* int */) {
        unsafe {
            wxDateTime_SetYear(_obj, year)
        }
    }
    fn SubtractDate(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_SubtractDate(_obj, diff, _ref)
        }
    }
    fn SubtractTime(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_SubtractTime(_obj, diff, _ref)
        }
    }
    fn ToGMT(_obj: *u8 /* void* */, noDST: c_int /* int */) {
        unsafe {
            wxDateTime_ToGMT(_obj, noDST)
        }
    }
    fn ToTimezone(_obj: *u8 /* void* */, tz: c_int /* int */, noDST: c_int /* int */) {
        unsafe {
            wxDateTime_ToTimezone(_obj, tz, noDST)
        }
    }
    fn Today(dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_Today(dt)
        }
    }
    fn UNow(dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_UNow(dt)
        }
    }
    fn wxDateTime(hi_long: c_int /* int */, lo_long: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_wxDateTime(hi_long, lo_long)
        }
    }
}
trait ELJGridTable {
    fn Create(_obj: *u8 /* void* */, _EifGetNumberRows: *u8 /* void* */, _EifGetNumberCols: *u8 /* void* */, _EifGetValue: *u8 /* void* */, _EifSetValue: *u8 /* void* */, _EifIsEmptyCell: *u8 /* void* */, _EifClear: *u8 /* void* */, _EifInsertRows: *u8 /* void* */, _EifAppendRows: *u8 /* void* */, _EifDeleteRows: *u8 /* void* */, _EifInsertCols: *u8 /* void* */, _EifAppendCols: *u8 /* void* */, _EifDeleteCols: *u8 /* void* */, _EifSetRowLabelValue: *u8 /* void* */, _EifSetColLabelValue: *u8 /* void* */, _EifGetRowLabelValue: *u8 /* void* */, _EifGetColLabelValue: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJGridTable_Delete(_obj)
        }
    }
    fn GetView(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJGridTable_GetView(_obj)
        }
    }
    fn SendTableMessage(_obj: *u8 /* void* */, id: c_int /* int */, val1: c_int /* int */, val2: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJGridTable_SendTableMessage(_obj, id, val1, val2)
        }
    }
}
trait wxTabCtrl {
}
trait wxQueryNewPaletteEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxQueryNewPaletteEvent_CopyObject(_obj, obj)
        }
    }
    fn GetPaletteRealized(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxQueryNewPaletteEvent_GetPaletteRealized(_obj)
        }
    }
    fn SetPaletteRealized(_obj: *u8 /* void* */, realized: bool /* bool */) {
        unsafe {
            wxQueryNewPaletteEvent_SetPaletteRealized(_obj, realized)
        }
    }
}
trait wxBitmapButton {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmapButton_Create(_prt, _id, _bmp, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetBitmapDisabled(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_GetBitmapDisabled(_obj, _ref)
        }
    }
    fn GetBitmapFocus(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_GetBitmapFocus(_obj, _ref)
        }
    }
    fn GetBitmapLabel(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_GetBitmapLabel(_obj, _ref)
        }
    }
    fn GetBitmapSelected(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_GetBitmapSelected(_obj, _ref)
        }
    }
    fn GetMarginX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmapButton_GetMarginX(_obj)
        }
    }
    fn GetMarginY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmapButton_GetMarginY(_obj)
        }
    }
    fn SetBitmapDisabled(_obj: *u8 /* void* */, disabled: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_SetBitmapDisabled(_obj, disabled)
        }
    }
    fn SetBitmapFocus(_obj: *u8 /* void* */, focus: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_SetBitmapFocus(_obj, focus)
        }
    }
    fn SetBitmapLabel(_obj: *u8 /* void* */, bitmap: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_SetBitmapLabel(_obj, bitmap)
        }
    }
    fn SetBitmapSelected(_obj: *u8 /* void* */, sel: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_SetBitmapSelected(_obj, sel)
        }
    }
    fn SetMargins(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxBitmapButton_SetMargins(_obj, arg0, arg1)
        }
    }
}
trait wxRecordSet {
}
trait wxStringBuffer {
}
trait cbDrawRowBkGroundEvent {
    // missing: cbDrawRowBkGroundEvent_Dc
    // missing: cbDrawRowBkGroundEvent_Row
}
trait wxHtmlTagHandler {
}
trait wxSizerItem {
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_CalcMin(_obj)
        }
    }
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_Create(arg0, arg1, option, flag, border, userData)
        }
    }
    fn CreateInSizer(sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_CreateInSizer(sizer, option, flag, border, userData)
        }
    }
    fn CreateInWindow(window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_CreateInWindow(window, option, flag, border, userData)
        }
    }
    fn GetBorder(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSizerItem_GetBorder(_obj)
        }
    }
    fn GetFlag(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSizerItem_GetFlag(_obj)
        }
    }
    fn GetMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetMinSize(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetPosition(_obj)
        }
    }
    fn GetRatio(_obj: *u8 /* void* */) -> float /* float */ {
        unsafe {
            wxSizerItem_GetRatio(_obj)
        }
    }
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetSize(_obj)
        }
    }
    fn GetSizer(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetSizer(_obj)
        }
    }
    fn GetUserData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetUserData(_obj)
        }
    }
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetWindow(_obj)
        }
    }
    fn IsSizer(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizerItem_IsSizer(_obj)
        }
    }
    fn IsSpacer(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizerItem_IsSpacer(_obj)
        }
    }
    fn IsWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizerItem_IsWindow(_obj)
        }
    }
    fn SetBorder(_obj: *u8 /* void* */, border: c_int /* int */) {
        unsafe {
            wxSizerItem_SetBorder(_obj, border)
        }
    }
    fn SetDimension(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxSizerItem_SetDimension(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn SetFlag(_obj: *u8 /* void* */, flag: c_int /* int */) {
        unsafe {
            wxSizerItem_SetFlag(_obj, flag)
        }
    }
    fn SetFloatRatio(_obj: *u8 /* void* */, ratio: float /* float */) {
        unsafe {
            wxSizerItem_SetFloatRatio(_obj, ratio)
        }
    }
    fn SetInitSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizerItem_SetInitSize(_obj, arg0, arg1)
        }
    }
    fn SetRatio(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizerItem_SetRatio(_obj, arg0, arg1)
        }
    }
    fn SetSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) {
        unsafe {
            wxSizerItem_SetSizer(_obj, sizer)
        }
    }
    fn SetWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizerItem_SetWindow(_obj, window)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxSizerItem_Delete(_obj)
        }
    }
    fn DeleteWindows(_obj: *u8 /* void* */) {
        unsafe {
            wxSizerItem_DeleteWindows(_obj)
        }
    }
    fn DetachSizer(_obj: *u8 /* void* */) {
        unsafe {
            wxSizerItem_DetachSizer(_obj)
        }
    }
    fn GetProportion(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSizerItem_GetProportion(_obj)
        }
    }
    fn GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetRect(_obj)
        }
    }
    fn GetSpacer(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetSpacer(_obj)
        }
    }
    fn IsShown(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSizerItem_IsShown(_obj)
        }
    }
    fn SetProportion(_obj: *u8 /* void* */, proportion: c_int /* int */) {
        unsafe {
            wxSizerItem_SetProportion(_obj, proportion)
        }
    }
    fn SetSpacer(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizerItem_SetSpacer(_obj, arg0, arg1)
        }
    }
    fn Show(_obj: *u8 /* void* */, show: c_int /* int */) {
        unsafe {
            wxSizerItem_Show(_obj, show)
        }
    }
}
trait wxMenuBar {
    fn Append(_obj: *u8 /* void* */, menu: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_Append(_obj, menu, title)
        }
    }
    fn Check(_obj: *u8 /* void* */, id: c_int /* int */, check: bool /* bool */) {
        unsafe {
            wxMenuBar_Check(_obj, id, check)
        }
    }
    fn Create(_style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_Create(_style)
        }
    }
    fn DeletePointer(_obj: *u8 /* void* */) {
        unsafe {
            wxMenuBar_DeletePointer(_obj)
        }
    }
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_Enable(_obj, enable)
        }
    }
    fn EnableItem(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxMenuBar_EnableItem(_obj, id, enable)
        }
    }
    fn EnableTop(_obj: *u8 /* void* */, pos: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxMenuBar_EnableTop(_obj, pos, enable)
        }
    }
    fn FindItem(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_FindItem(_obj, id)
        }
    }
    fn FindMenu(_obj: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_FindMenu(_obj, title)
        }
    }
    fn FindMenuItem(_obj: *u8 /* void* */, menuString: *u8 /* void* */, itemString: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_FindMenuItem(_obj, menuString, itemString)
        }
    }
    fn GetHelpString(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetHelpString(_obj, id)
        }
    }
    fn GetLabel(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetLabel(_obj, id)
        }
    }
    fn GetLabelTop(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetLabelTop(_obj, pos)
        }
    }
    fn GetMenu(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetMenu(_obj, pos)
        }
    }
    fn GetMenuCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_GetMenuCount(_obj)
        }
    }
    fn Insert(_obj: *u8 /* void* */, pos: c_int /* int */, menu: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_Insert(_obj, pos, menu, title)
        }
    }
    fn IsChecked(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMenuBar_IsChecked(_obj, id)
        }
    }
    fn IsEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMenuBar_IsEnabled(_obj, id)
        }
    }
    fn Remove(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_Remove(_obj, pos)
        }
    }
    fn Replace(_obj: *u8 /* void* */, pos: c_int /* int */, menu: *u8 /* void* */, title: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_Replace(_obj, pos, menu, title)
        }
    }
    fn SetHelpString(_obj: *u8 /* void* */, id: c_int /* int */, helpString: *u8 /* void* */) {
        unsafe {
            wxMenuBar_SetHelpString(_obj, id, helpString)
        }
    }
    fn SetItemLabel(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxMenuBar_SetItemLabel(_obj, id, label)
        }
    }
    fn SetLabel(_obj: *u8 /* void* */, s: *u8 /* void* */) {
        unsafe {
            wxMenuBar_SetLabel(_obj, s)
        }
    }
    fn SetLabelTop(_obj: *u8 /* void* */, pos: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxMenuBar_SetLabelTop(_obj, pos, label)
        }
    }
}
trait cbCloseBox {
    // missing: cbCloseBox_Create
}
trait wxMetafileDC {
    fn Close(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMetafileDC_Close(_obj)
        }
    }
    fn Create(_file: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMetafileDC_Create(_file)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMetafileDC_Delete(_obj)
        }
    }
}
trait wxDataOutputStream {
}
trait wxGridCellEditor {
    fn BeginEdit(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, grid: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_BeginEdit(_obj, row, col, grid)
        }
    }
    fn Create(_obj: *u8 /* void* */, parent: *u8 /* void* */, id: c_int /* int */, evtHandler: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_Create(_obj, parent, id, evtHandler)
        }
    }
    fn Destroy(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_Destroy(_obj)
        }
    }
    fn EndEdit(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, grid: *u8 /* void* */, oldStr: *u8 /* void* */, newStr: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridCellEditor_EndEdit(_obj, row, col, grid, oldStr, newStr)
        }
    }
    fn GetControl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellEditor_GetControl(_obj)
        }
    }
    fn HandleReturn(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_HandleReturn(_obj, event)
        }
    }
    fn IsAcceptedKey(_obj: *u8 /* void* */, event: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellEditor_IsAcceptedKey(_obj, event)
        }
    }
    fn IsCreated(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellEditor_IsCreated(_obj)
        }
    }
    fn PaintBackground(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_PaintBackground(_obj, arg0, arg1, arg2, arg3, attr)
        }
    }
    fn Reset(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_Reset(_obj)
        }
    }
    fn SetControl(_obj: *u8 /* void* */, control: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_SetControl(_obj, control)
        }
    }
    fn SetParameters(_obj: *u8 /* void* */, params: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_SetParameters(_obj, params)
        }
    }
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxGridCellEditor_SetSize(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn Show(_obj: *u8 /* void* */, show: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_Show(_obj, show, attr)
        }
    }
    fn StartingClick(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_StartingClick(_obj)
        }
    }
    fn StartingKey(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_StartingKey(_obj, event)
        }
    }
}
trait wxBufferedDC {
    fn CreateByDCAndSize(dc: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBufferedDC_CreateByDCAndSize(dc, arg0, arg1, style)
        }
    }
    fn CreateByDCAndBitmap(dc: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBufferedDC_CreateByDCAndBitmap(dc, bitmap, style)
        }
    }
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxBufferedDC_Delete(self_)
        }
    }
}
trait wxLogPassThrough {
}
trait wxTimer {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTimer_Create(_prt, _id)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxTimer_Delete(_obj)
        }
    }
    fn GetInterval(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTimer_GetInterval(_obj)
        }
    }
    fn IsOneShot(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTimer_IsOneShot(_obj)
        }
    }
    fn IsRuning(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTimer_IsRuning(_obj)
        }
    }
    fn Start(_obj: *u8 /* void* */, _int: c_int /* int */, _one: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxTimer_Start(_obj, _int, _one)
        }
    }
    fn Stop(_obj: *u8 /* void* */) {
        unsafe {
            wxTimer_Stop(_obj)
        }
    }
}
trait ELJArtProv {
    fn Create(_obj: *u8 /* void* */, _clb: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJArtProv_Create(_obj, _clb)
        }
    }
    fn Release(_obj: *u8 /* void* */) {
        unsafe {
            ELJArtProv_Release(_obj)
        }
    }
}
trait wxXmlResourceHandler {
}
trait wxExpr {
}
trait wxWizardEvent {
    fn GetDirection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWizardEvent_GetDirection(_obj)
        }
    }
}
trait wxSplitterScrolledWindow {
    // missing: wxSplitterScrolledWindow_Create
}
trait wxDocManager {
}
trait wxDatabase {
}
trait wxPGProperty {
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPGProperty_GetLabel(_obj)
        }
    }
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPGProperty_GetName(_obj)
        }
    }
    fn GetValueAsString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPGProperty_GetValueAsString(_obj)
        }
    }
    fn GetValueType(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPGProperty_GetValueType(_obj)
        }
    }
    fn SetHelpString(_obj: *u8 /* void* */, helpString: *u8 /* void* */) {
        unsafe {
            wxPGProperty_SetHelpString(_obj, helpString)
        }
    }
}
trait wxList {
}
trait wxDynamicToolBar {
    // missing: wxDynamicToolBar_AddSeparator
    // missing: wxDynamicToolBar_AddTool
    // missing: wxDynamicToolBar_AddToolBitmap
    // missing: wxDynamicToolBar_AddToolImage
    // missing: wxDynamicToolBar_AddToolLabel
    // missing: wxDynamicToolBar_Create
    // missing: wxDynamicToolBar_CreateDefault
    // missing: wxDynamicToolBar_CreateDefaultLayout
    // missing: wxDynamicToolBar_CreateParams
    // missing: wxDynamicToolBar_CreateTool
    // missing: wxDynamicToolBar_CreateToolControl
    // missing: wxDynamicToolBar_Delete
    // missing: wxDynamicToolBar_DoDeleteTool
    // missing: wxDynamicToolBar_DoEnableTool
    // missing: wxDynamicToolBar_DoInsertTool
    // missing: wxDynamicToolBar_DoSetToggle
    // missing: wxDynamicToolBar_DoToggleTool
    // missing: wxDynamicToolBar_DrawSeparator
    // missing: wxDynamicToolBar_EnableTool
    // missing: wxDynamicToolBar_FindToolForPosition
    // missing: wxDynamicToolBar_GetPreferredDim
    // missing: wxDynamicToolBar_GetToolInfo
    // missing: wxDynamicToolBar_Layout
    // missing: wxDynamicToolBar_RemoveTool
    // missing: wxDynamicToolBar_SetLayout
}
trait wxDirDialog {
    fn Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _dir: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDirDialog_Create(_prt, _msg, _dir, arg0, arg1, _stl)
        }
    }
    fn GetMessage(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDirDialog_GetMessage(_obj)
        }
    }
    fn GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDirDialog_GetPath(_obj)
        }
    }
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDirDialog_GetStyle(_obj)
        }
    }
    fn SetMessage(_obj: *u8 /* void* */, msg: *u8 /* void* */) {
        unsafe {
            wxDirDialog_SetMessage(_obj, msg)
        }
    }
    fn SetPath(_obj: *u8 /* void* */, pth: *u8 /* void* */) {
        unsafe {
            wxDirDialog_SetPath(_obj, pth)
        }
    }
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxDirDialog_SetStyle(_obj, style)
        }
    }
}
trait cbCustomizeBarEvent {
    // missing: cbCustomizeBarEvent_Bar
    // missing: cbCustomizeBarEvent_ClickPos
}
trait wxArray {
}
trait wxDialUpEvent {
    // missing: wxDialUpEvent_IsConnectedEvent
    // missing: wxDialUpEvent_IsOwnEvent
}
trait cbCommonPaneProperties {
    // missing: cbCommonPaneProperties_Assign
    // missing: cbCommonPaneProperties_BarCollapseIconsOn
    // missing: cbCommonPaneProperties_BarDragHintsOn
    // missing: cbCommonPaneProperties_BarFloatingOn
    // missing: cbCommonPaneProperties_ColProportionsOn
    // missing: cbCommonPaneProperties_CreateDefault
    // missing: cbCommonPaneProperties_Delete
    // missing: cbCommonPaneProperties_ExactDockPredictionOn
    // missing: cbCommonPaneProperties_MinCBarDim
    // missing: cbCommonPaneProperties_NonDestructFrictionOn
    // missing: cbCommonPaneProperties_OutOfPaneDragOn
    // missing: cbCommonPaneProperties_RealTimeUpdatesOn
    // missing: cbCommonPaneProperties_ResizeHandleSize
    // missing: cbCommonPaneProperties_RowProportionsOn
    // missing: cbCommonPaneProperties_SetBarCollapseIconsOn
    // missing: cbCommonPaneProperties_SetBarDragHintsOn
    // missing: cbCommonPaneProperties_SetBarFloatingOn
    // missing: cbCommonPaneProperties_SetColProportionsOn
    // missing: cbCommonPaneProperties_SetExactDockPredictionOn
    // missing: cbCommonPaneProperties_SetMinCBarDim
    // missing: cbCommonPaneProperties_SetNonDestructFrictionOn
    // missing: cbCommonPaneProperties_SetOutOfPaneDragOn
    // missing: cbCommonPaneProperties_SetRealTimeUpdatesOn
    // missing: cbCommonPaneProperties_SetResizeHandleSize
    // missing: cbCommonPaneProperties_SetRowProportionsOn
    // missing: cbCommonPaneProperties_SetShow3DPaneBorderOn
    // missing: cbCommonPaneProperties_Show3DPaneBorderOn
}
trait wxDirTraverser {
}
trait wxColourData {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxColourData_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxColourData_Delete(_obj)
        }
    }
    fn GetChooseFull(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxColourData_GetChooseFull(_obj)
        }
    }
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxColourData_GetColour(_obj, _ref)
        }
    }
    fn GetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxColourData_GetCustomColour(_obj, i, _ref)
        }
    }
    fn SetChooseFull(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxColourData_SetChooseFull(_obj, flag)
        }
    }
    fn SetColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxColourData_SetColour(_obj, colour)
        }
    }
    fn SetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxColourData_SetCustomColour(_obj, i, colour)
        }
    }
}
trait wxTaskBarIcon {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxTaskBarIcon_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxTaskBarIcon_Delete(_obj)
        }
    }
    fn IsIconInstalled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_IsIconInstalled(_obj)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_IsOk(_obj)
        }
    }
    fn PopupMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_PopupMenu(_obj, menu)
        }
    }
    fn RemoveIcon(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_RemoveIcon(_obj)
        }
    }
    fn SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */, text: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_SetIcon(_obj, icon, text)
        }
    }
}
trait wxStaticBoxSizer {
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBoxSizer_CalcMin(_obj)
        }
    }
    fn Create(box: *u8 /* void* */, orient: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBoxSizer_Create(box, orient)
        }
    }
    fn GetStaticBox(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBoxSizer_GetStaticBox(_obj)
        }
    }
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxStaticBoxSizer_RecalcSizes(_obj)
        }
    }
}
trait wxTime {
}
trait wxMouseEvent {
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_AltDown(_obj)
        }
    }
    fn Button(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Button(_obj, but)
        }
    }
    fn ButtonDClick(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ButtonDClick(_obj, but)
        }
    }
    fn ButtonDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ButtonDown(_obj, but)
        }
    }
    fn ButtonIsDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ButtonIsDown(_obj, but)
        }
    }
    fn ButtonUp(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ButtonUp(_obj, but)
        }
    }
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ControlDown(_obj)
        }
    }
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxMouseEvent_CopyObject(_obj, object_dest)
        }
    }
    fn Dragging(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Dragging(_obj)
        }
    }
    fn Entering(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Entering(_obj)
        }
    }
    fn GetLogicalPosition(_obj: *u8 /* void* */, dc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMouseEvent_GetLogicalPosition(_obj, dc)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMouseEvent_GetPosition(_obj)
        }
    }
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetX(_obj)
        }
    }
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetY(_obj)
        }
    }
    fn IsButton(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_IsButton(_obj)
        }
    }
    fn Leaving(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Leaving(_obj)
        }
    }
    fn LeftDClick(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_LeftDClick(_obj)
        }
    }
    fn LeftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_LeftDown(_obj)
        }
    }
    fn LeftIsDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_LeftIsDown(_obj)
        }
    }
    fn LeftUp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_LeftUp(_obj)
        }
    }
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MetaDown(_obj)
        }
    }
    fn MiddleDClick(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MiddleDClick(_obj)
        }
    }
    fn MiddleDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MiddleDown(_obj)
        }
    }
    fn MiddleIsDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MiddleIsDown(_obj)
        }
    }
    fn MiddleUp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MiddleUp(_obj)
        }
    }
    fn Moving(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Moving(_obj)
        }
    }
    fn RightDClick(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_RightDClick(_obj)
        }
    }
    fn RightDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_RightDown(_obj)
        }
    }
    fn RightIsDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_RightIsDown(_obj)
        }
    }
    fn RightUp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_RightUp(_obj)
        }
    }
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ShiftDown(_obj)
        }
    }
}
trait ELJApp {
    fn Bell() {
        unsafe {
            ELJApp_Bell()
        }
    }
    fn CreateLogTarget() -> *u8 /* void* */ {
        unsafe {
            ELJApp_CreateLogTarget()
        }
    }
    fn Dispatch() {
        unsafe {
            ELJApp_Dispatch()
        }
    }
    fn DisplaySize() -> *u8 /* void* */ {
        unsafe {
            ELJApp_DisplaySize()
        }
    }
    fn EnableTooltips(_enable: bool /* bool */) {
        unsafe {
            ELJApp_EnableTooltips(_enable)
        }
    }
    fn EnableTopLevelWindows(_enb: c_int /* int */) {
        unsafe {
            ELJApp_EnableTopLevelWindows(_enb)
        }
    }
    fn ExecuteProcess(_cmd: *u8 /* void* */, _snc: c_int /* int */, _prc: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJApp_ExecuteProcess(_cmd, _snc, _prc)
        }
    }
    fn Exit() {
        unsafe {
            ELJApp_Exit()
        }
    }
    fn ExitMainLoop() {
        unsafe {
            ELJApp_ExitMainLoop()
        }
    }
    fn FindWindowById(_id: c_int /* int */, _prt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJApp_FindWindowById(_id, _prt)
        }
    }
    fn FindWindowByLabel(_lbl: *u8 /* void* */, _prt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJApp_FindWindowByLabel(_lbl, _prt)
        }
    }
    fn FindWindowByName(_lbl: *u8 /* void* */, _prt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJApp_FindWindowByName(_lbl, _prt)
        }
    }
    fn GetApp() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetApp()
        }
    }
    fn GetAppName() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetAppName()
        }
    }
    fn GetClassName() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetClassName()
        }
    }
    fn GetExitOnFrameDelete() -> c_int /* int */ {
        unsafe {
            ELJApp_GetExitOnFrameDelete()
        }
    }
    fn GetOsDescription() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetOsDescription()
        }
    }
    fn GetOsVersion(_maj: *u8 /* void* */, _min: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJApp_GetOsVersion(_maj, _min)
        }
    }
    fn GetTopWindow() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetTopWindow()
        }
    }
    fn GetUseBestVisual() -> c_int /* int */ {
        unsafe {
            ELJApp_GetUseBestVisual()
        }
    }
    fn GetUserHome(_usr: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetUserHome(_usr)
        }
    }
    fn GetUserId() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetUserId()
        }
    }
    fn GetUserName() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetUserName()
        }
    }
    fn GetVendorName() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetVendorName()
        }
    }
    fn InitAllImageHandlers() {
        unsafe {
            ELJApp_InitAllImageHandlers()
        }
    }
    fn Initialized() -> bool /* bool */ {
        unsafe {
            ELJApp_Initialized()
        }
    }
    fn MainLoop() -> c_int /* int */ {
        unsafe {
            ELJApp_MainLoop()
        }
    }
    fn MousePosition() -> *u8 /* void* */ {
        unsafe {
            ELJApp_MousePosition()
        }
    }
    fn Pending() -> c_int /* int */ {
        unsafe {
            ELJApp_Pending()
        }
    }
    fn SafeYield(_win: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJApp_SafeYield(_win)
        }
    }
    fn SetAppName(name: *u8 /* void* */) {
        unsafe {
            ELJApp_SetAppName(name)
        }
    }
    fn SetClassName(name: *u8 /* void* */) {
        unsafe {
            ELJApp_SetClassName(name)
        }
    }
    fn SetExitOnFrameDelete(flag: c_int /* int */) {
        unsafe {
            ELJApp_SetExitOnFrameDelete(flag)
        }
    }
    fn SetPrintMode(mode: c_int /* int */) {
        unsafe {
            ELJApp_SetPrintMode(mode)
        }
    }
    fn SetTooltipDelay(_ms: c_int /* int */) {
        unsafe {
            ELJApp_SetTooltipDelay(_ms)
        }
    }
    fn SetTopWindow(_wnd: *u8 /* void* */) {
        unsafe {
            ELJApp_SetTopWindow(_wnd)
        }
    }
    fn SetUseBestVisual(flag: c_int /* int */) {
        unsafe {
            ELJApp_SetUseBestVisual(flag)
        }
    }
    fn SetVendorName(name: *u8 /* void* */) {
        unsafe {
            ELJApp_SetVendorName(name)
        }
    }
    fn Sleep(_scs: c_int /* int */) {
        unsafe {
            ELJApp_Sleep(_scs)
        }
    }
    fn MilliSleep(_mscs: c_int /* int */) {
        unsafe {
            ELJApp_MilliSleep(_mscs)
        }
    }
    fn Yield() -> c_int /* int */ {
        unsafe {
            ELJApp_Yield()
        }
    }
    fn IsTerminating() -> c_int /* int */ {
        unsafe {
            ELJApp_IsTerminating()
        }
    }
}
trait wxTabEvent {
}
trait cbLeftDClickEvent {
    // missing: cbLeftDClickEvent_Pos
}
trait wxMBConvFile {
}
trait wxCaret {
    fn Create(_wnd: *u8 /* void* */, _wth: c_int /* int */, _hgt: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCaret_Create(_wnd, _wth, _hgt)
        }
    }
    fn GetBlinkTime() -> c_int /* int */ {
        unsafe {
            wxCaret_GetBlinkTime()
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCaret_GetPosition(_obj)
        }
    }
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCaret_GetSize(_obj)
        }
    }
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCaret_GetWindow(_obj)
        }
    }
    fn Hide(_obj: *u8 /* void* */) {
        unsafe {
            wxCaret_Hide(_obj)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCaret_IsOk(_obj)
        }
    }
    fn IsVisible(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCaret_IsVisible(_obj)
        }
    }
    fn Move(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxCaret_Move(_obj, arg0, arg1)
        }
    }
    fn SetBlinkTime(milliseconds: c_int /* int */) {
        unsafe {
            wxCaret_SetBlinkTime(milliseconds)
        }
    }
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxCaret_SetSize(_obj, arg0, arg1)
        }
    }
    fn Show(_obj: *u8 /* void* */) {
        unsafe {
            wxCaret_Show(_obj)
        }
    }
}
trait wxHelpControllerHelpProvider {
    fn Create(ctr: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpControllerHelpProvider_Create(ctr)
        }
    }
    fn GetHelpController(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpControllerHelpProvider_GetHelpController(_obj)
        }
    }
    fn SetHelpController(_obj: *u8 /* void* */, hc: *u8 /* void* */) {
        unsafe {
            wxHelpControllerHelpProvider_SetHelpController(_obj, hc)
        }
    }
}
trait wxPlotWindow {
    // missing: wxPlotWindow_Add
    // missing: wxPlotWindow_AddOnOff
    // missing: wxPlotWindow_Create
    // missing: wxPlotWindow_Delete
    // missing: wxPlotWindow_DeleteOnOff
    // missing: wxPlotWindow_Enlarge
    // missing: wxPlotWindow_GetAt
    // missing: wxPlotWindow_GetCount
    // missing: wxPlotWindow_GetCurrent
    // missing: wxPlotWindow_GetEnlargeAroundWindowCentre
    // missing: wxPlotWindow_GetOnOffCurveAt
    // missing: wxPlotWindow_GetOnOffCurveCount
    // missing: wxPlotWindow_GetScrollOnThumbRelease
    // missing: wxPlotWindow_GetUnitsPerValue
    // missing: wxPlotWindow_GetZoom
    // missing: wxPlotWindow_Move
    // missing: wxPlotWindow_RedrawEverything
    // missing: wxPlotWindow_RedrawXAxis
    // missing: wxPlotWindow_RedrawYAxis
    // missing: wxPlotWindow_ResetScrollbar
    // missing: wxPlotWindow_SetCurrent
    // missing: wxPlotWindow_SetEnlargeAroundWindowCentre
    // missing: wxPlotWindow_SetScrollOnThumbRelease
    // missing: wxPlotWindow_SetUnitsPerValue
    // missing: wxPlotWindow_SetZoom
}
trait wxSocketClient {
}
trait wxPlotEvent {
    // missing: wxPlotEvent_GetCurve
    // missing: wxPlotEvent_GetPosition
    // missing: wxPlotEvent_GetZoom
    // missing: wxPlotEvent_SetPosition
    // missing: wxPlotEvent_SetZoom
}
trait wxCommandEvent {
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_CopyObject(_obj, object_dest)
        }
    }
    fn Create(_typ: c_int /* int */, _id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCommandEvent_Create(_typ, _id)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_Delete(_obj)
        }
    }
    fn GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCommandEvent_GetClientData(_obj)
        }
    }
    fn GetClientObject(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCommandEvent_GetClientObject(_obj)
        }
    }
    fn GetExtraLong(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxCommandEvent_GetExtraLong(_obj)
        }
    }
    fn GetInt(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxCommandEvent_GetInt(_obj)
        }
    }
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCommandEvent_GetSelection(_obj)
        }
    }
    fn GetString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCommandEvent_GetString(_obj)
        }
    }
    fn IsChecked(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCommandEvent_IsChecked(_obj)
        }
    }
    fn IsSelection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCommandEvent_IsSelection(_obj)
        }
    }
    fn SetClientData(_obj: *u8 /* void* */, clientData: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_SetClientData(_obj, clientData)
        }
    }
    fn SetClientObject(_obj: *u8 /* void* */, clientObject: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_SetClientObject(_obj, clientObject)
        }
    }
    fn SetExtraLong(_obj: *u8 /* void* */, extraLong: c_long /* long */) {
        unsafe {
            wxCommandEvent_SetExtraLong(_obj, extraLong)
        }
    }
    fn SetInt(_obj: *u8 /* void* */, i: c_int /* int */) {
        unsafe {
            wxCommandEvent_SetInt(_obj, i)
        }
    }
    fn SetString(_obj: *u8 /* void* */, s: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_SetString(_obj, s)
        }
    }
}
trait cbRemoveBarEvent {
    // missing: cbRemoveBarEvent_Bar
}
trait wxZlibOutputStream {
}
trait wxLogWindow {
}
trait wxPaletteChangedEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxPaletteChangedEvent_CopyObject(_obj, obj)
        }
    }
    fn GetChangedWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPaletteChangedEvent_GetChangedWindow(_obj)
        }
    }
    fn SetChangedWindow(_obj: *u8 /* void* */, win: *u8 /* void* */) {
        unsafe {
            wxPaletteChangedEvent_SetChangedWindow(_obj, win)
        }
    }
}
trait ELJClient {
    // missing: ELJClient_Create
    // missing: ELJClient_Delete
    // missing: ELJClient_MakeConnection
}
trait wxGenericValidator {
}
trait wxLogChain {
    fn Create(logger: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLogChain_Create(logger)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxLogChain_Delete(_obj)
        }
    }
    fn GetOldLog(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLogChain_GetOldLog(_obj)
        }
    }
    fn IsPassingMessages(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLogChain_IsPassingMessages(_obj)
        }
    }
    fn PassMessages(_obj: *u8 /* void* */, bDoPass: bool /* bool */) {
        unsafe {
            wxLogChain_PassMessages(_obj, bDoPass)
        }
    }
    fn SetLog(_obj: *u8 /* void* */, logger: *u8 /* void* */) {
        unsafe {
            wxLogChain_SetLog(_obj, logger)
        }
    }
}
trait wxPreviewCanvas {
    fn Create(preview: *u8 /* void* */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPreviewCanvas_Create(preview, parent, arg0, arg1, arg2, arg3, style)
        }
    }
}
trait wxActivateEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxActivateEvent_CopyObject(_obj, obj)
        }
    }
    fn GetActive(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxActivateEvent_GetActive(_obj)
        }
    }
}
trait cbDrawHintRectEvent {
    // missing: cbDrawHintRectEvent_EraseRect
    // missing: cbDrawHintRectEvent_IsInClient
    // missing: cbDrawHintRectEvent_LastTime
    // missing: cbDrawHintRectEvent_Rect
}
trait cbInsertBarEvent {
    // missing: cbInsertBarEvent_Bar
    // missing: cbInsertBarEvent_Row
}
trait wxHtmlColourCell {
}
trait cbDrawBarDecorEvent {
    // missing: cbDrawBarDecorEvent_Bar
    // missing: cbDrawBarDecorEvent_BoundsInParent
    // missing: cbDrawBarDecorEvent_Dc
}
trait wxGridCellTextEditor {
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellTextEditor_Ctor()
        }
    }
}
trait wxHTTP {
}
trait wxDocument {
}
trait cbBarHintsPlugin {
    // missing: cbBarHintsPlugin_Create
    // missing: cbBarHintsPlugin_CreateDefault
    // missing: cbBarHintsPlugin_Delete
    // missing: cbBarHintsPlugin_SetGrooveCount
}
trait cbSizeBarWndEvent {
    // missing: cbSizeBarWndEvent_Bar
    // missing: cbSizeBarWndEvent_BoundsInParent
}
trait wxPropertyCategory {
    fn Create(label: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPropertyCategory_Create(label)
        }
    }
}
trait wxGLCanvas {
}
trait wxSystemOptions {
}
trait wxSizeEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxSizeEvent_CopyObject(_obj, obj)
        }
    }
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizeEvent_GetSize(_obj)
        }
    }
}
trait wxWizardPageSimple {
    fn Create(_prt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizardPageSimple_Create(_prt)
        }
    }
    fn GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxWizardPageSimple_GetBitmap(_obj, _ref)
        }
    }
    fn GetNext(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizardPageSimple_GetNext(_obj)
        }
    }
    fn GetPrev(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizardPageSimple_GetPrev(_obj)
        }
    }
    fn SetNext(_obj: *u8 /* void* */, next: *u8 /* void* */) {
        unsafe {
            wxWizardPageSimple_SetNext(_obj, next)
        }
    }
    fn SetPrev(_obj: *u8 /* void* */, prev: *u8 /* void* */) {
        unsafe {
            wxWizardPageSimple_SetPrev(_obj, prev)
        }
    }
}
trait wxFilterInputStream {
}
trait cbBarDragPlugin {
    // missing: cbBarDragPlugin_Create
    // missing: cbBarDragPlugin_CreateDefault
    // missing: cbBarDragPlugin_Delete
}
trait wxBusyInfo {
    fn Create(_txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBusyInfo_Create(_txt)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxBusyInfo_Delete(_obj)
        }
    }
}
trait wxInputSink {
    fn Create(input: *u8 /* void* */, evtHandler: *u8 /* void* */, bufferLen: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxInputSink_Create(input, evtHandler, bufferLen)
        }
    }
    fn GetId(obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputSink_GetId(obj)
        }
    }
    fn Start(obj: *u8 /* void* */) {
        unsafe {
            wxInputSink_Start(obj)
        }
    }
}
trait wxTimerEvent {
    fn GetInterval(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTimerEvent_GetInterval(_obj)
        }
    }
}
trait wxListEvent {
    fn Cancelled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListEvent_Cancelled(_obj)
        }
    }
    fn GetCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetCode(_obj)
        }
    }
    fn GetColumn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetColumn(_obj)
        }
    }
    fn GetData(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetData(_obj)
        }
    }
    fn GetImage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetImage(_obj)
        }
    }
    fn GetIndex(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetIndex(_obj)
        }
    }
    fn GetItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListEvent_GetItem(_obj, _ref)
        }
    }
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListEvent_GetLabel(_obj)
        }
    }
    fn GetMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetMask(_obj)
        }
    }
    fn GetPoint(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListEvent_GetPoint(_obj)
        }
    }
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListEvent_GetText(_obj)
        }
    }
}
trait wxColour {
    fn Alpha(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */ {
        unsafe {
            wxColour_Alpha(_obj)
        }
    }
    fn Assign(_obj: *u8 /* void* */, other: *u8 /* void* */) {
        unsafe {
            wxColour_Assign(_obj, other)
        }
    }
    fn Blue(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */ {
        unsafe {
            wxColour_Blue(_obj)
        }
    }
    fn Copy(_obj: *u8 /* void* */, _other: *u8 /* void* */) {
        unsafe {
            wxColour_Copy(_obj, _other)
        }
    }
    fn CreateByName(_name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateByName(_name)
        }
    }
    fn CreateEmpty() -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateEmpty()
        }
    }
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateFromStock(id)
        }
    }
    fn CreateRGB(_red: uint8_t /* uint8_t */, _green: uint8_t /* uint8_t */, _blue: uint8_t /* uint8_t */, _alpha: uint8_t /* uint8_t */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateRGB(_red, _green, _blue, _alpha)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxColour_Delete(_obj)
        }
    }
    fn Green(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */ {
        unsafe {
            wxColour_Green(_obj)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxColour_IsOk(_obj)
        }
    }
    fn Red(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */ {
        unsafe {
            wxColour_Red(_obj)
        }
    }
    fn Set(_obj: *u8 /* void* */, _red: uint8_t /* uint8_t */, _green: uint8_t /* uint8_t */, _blue: uint8_t /* uint8_t */, _alpha: uint8_t /* uint8_t */) {
        unsafe {
            wxColour_Set(_obj, _red, _green, _blue, _alpha)
        }
    }
    fn SetByName(_obj: *u8 /* void* */, _name: *u8 /* void* */) {
        unsafe {
            wxColour_SetByName(_obj, _name)
        }
    }
    fn ValidName(_name: *wchar_t /* wchar_t* */) -> bool /* bool */ {
        unsafe {
            wxColour_ValidName(_name)
        }
    }
}
trait wxGridCellStringRenderer {
}
trait wxTreeCompanionWindow {
    // missing: wxTreeCompanionWindow_Create
    // missing: wxTreeCompanionWindow_DrawItem
    // missing: wxTreeCompanionWindow_GetTreeCtrl
    // missing: wxTreeCompanionWindow_SetTreeCtrl
}
trait wxRegionIterator {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxRegionIterator_Create()
        }
    }
    fn CreateFromRegion(region: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxRegionIterator_CreateFromRegion(region)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxRegionIterator_Delete(_obj)
        }
    }
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRegionIterator_GetHeight(_obj)
        }
    }
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRegionIterator_GetWidth(_obj)
        }
    }
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRegionIterator_GetX(_obj)
        }
    }
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRegionIterator_GetY(_obj)
        }
    }
    fn HaveRects(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegionIterator_HaveRects(_obj)
        }
    }
    fn Next(_obj: *u8 /* void* */) {
        unsafe {
            wxRegionIterator_Next(_obj)
        }
    }
    fn Reset(_obj: *u8 /* void* */) {
        unsafe {
            wxRegionIterator_Reset(_obj)
        }
    }
    fn ResetToRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) {
        unsafe {
            wxRegionIterator_ResetToRegion(_obj, region)
        }
    }
}
trait wxCriticalSectionLocker {
}
trait wxHtmlWindow {
}
trait wxFFileOutputStream {
}
trait wxHtmlHelpFrame {
}
trait wxQueryCol {
}
trait wxLEDNumberCtrl {
    // missing: wxLEDNumberCtrl_Create
    // missing: wxLEDNumberCtrl_GetAlignment
    // missing: wxLEDNumberCtrl_GetDrawFaded
    // missing: wxLEDNumberCtrl_GetValue
    // missing: wxLEDNumberCtrl_SetAlignment
    // missing: wxLEDNumberCtrl_SetDrawFaded
    // missing: wxLEDNumberCtrl_SetValue
}
trait wxMimeTypesManager {
    fn AddFallbacks(_obj: *u8 /* void* */, _types: *u8 /* void* */) {
        unsafe {
            wxMimeTypesManager_AddFallbacks(_obj, _types)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxMimeTypesManager_Create()
        }
    }
    fn EnumAllFileTypes(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMimeTypesManager_EnumAllFileTypes(_obj, _lst)
        }
    }
    fn GetFileTypeFromExtension(_obj: *u8 /* void* */, _ext: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMimeTypesManager_GetFileTypeFromExtension(_obj, _ext)
        }
    }
    fn GetFileTypeFromMimeType(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMimeTypesManager_GetFileTypeFromMimeType(_obj, _name)
        }
    }
    fn IsOfType(_obj: *u8 /* void* */, _type: *u8 /* void* */, _wildcard: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMimeTypesManager_IsOfType(_obj, _type, _wildcard)
        }
    }
}
trait wxCursor {
    fn Cursor_CreateFromStock(_id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            Cursor_CreateFromStock(_id)
        }
    }
    fn Cursor_CreateFromImage(image: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            Cursor_CreateFromImage(image)
        }
    }
    fn Cursor_CreateLoad(name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            Cursor_CreateLoad(name, type_, arg0, arg1)
        }
    }
}
trait wxRect {
}
trait wxClientDataContainer {
}
trait wxStopWatch {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxStopWatch_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxStopWatch_Delete(_obj)
        }
    }
    fn Start(_obj: *u8 /* void* */, msec: c_int /* int */) {
        unsafe {
            wxStopWatch_Start(_obj, msec)
        }
    }
    fn Pause(_obj: *u8 /* void* */) {
        unsafe {
            wxStopWatch_Pause(_obj)
        }
    }
    fn Resume(_obj: *u8 /* void* */) {
        unsafe {
            wxStopWatch_Resume(_obj)
        }
    }
    fn Time(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStopWatch_Time(_obj)
        }
    }
}
trait wxNotebook {
    fn AddPage(_obj: *u8 /* void* */, pPage: *u8 /* void* */, strText: *u8 /* void* */, bSelect: bool /* bool */, imageId: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_AddPage(_obj, pPage, strText, bSelect, imageId)
        }
    }
    fn AdvanceSelection(_obj: *u8 /* void* */, bForward: bool /* bool */) {
        unsafe {
            wxNotebook_AdvanceSelection(_obj, bForward)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxNotebook_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn DeleteAllPages(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNotebook_DeleteAllPages(_obj)
        }
    }
    fn DeletePage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_DeletePage(_obj, nPage)
        }
    }
    fn GetImageList(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxNotebook_GetImageList(_obj)
        }
    }
    fn GetPage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxNotebook_GetPage(_obj, nPage)
        }
    }
    fn GetPageCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxNotebook_GetPageCount(_obj)
        }
    }
    fn GetPageImage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxNotebook_GetPageImage(_obj, nPage)
        }
    }
    fn GetPageText(_obj: *u8 /* void* */, nPage: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxNotebook_GetPageText(_obj, nPage)
        }
    }
    fn GetRowCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxNotebook_GetRowCount(_obj)
        }
    }
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxNotebook_GetSelection(_obj)
        }
    }
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *c_long /* long* */) -> c_int /* int */ {
        unsafe {
            wxNotebook_HitTest(_obj, arg0, arg1, flags)
        }
    }
    fn InsertPage(_obj: *u8 /* void* */, nPage: c_int /* int */, pPage: *u8 /* void* */, strText: *u8 /* void* */, bSelect: bool /* bool */, imageId: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_InsertPage(_obj, nPage, pPage, strText, bSelect, imageId)
        }
    }
    fn RemovePage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_RemovePage(_obj, nPage)
        }
    }
    fn SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxNotebook_SetImageList(_obj, imageList)
        }
    }
    fn SetPadding(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxNotebook_SetPadding(_obj, arg0, arg1)
        }
    }
    fn SetPageImage(_obj: *u8 /* void* */, nPage: c_int /* int */, nImage: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_SetPageImage(_obj, nPage, nImage)
        }
    }
    fn SetPageSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxNotebook_SetPageSize(_obj, arg0, arg1)
        }
    }
    fn SetPageText(_obj: *u8 /* void* */, nPage: c_int /* int */, strText: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNotebook_SetPageText(_obj, nPage, strText)
        }
    }
    fn SetSelection(_obj: *u8 /* void* */, nPage: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxNotebook_SetSelection(_obj, nPage)
        }
    }
    fn expNB_TOP() -> c_int /* int */ {
        unsafe {
            expNB_TOP()
        }
    }
    fn expNB_BOTTOM() -> c_int /* int */ {
        unsafe {
            expNB_BOTTOM()
        }
    }
    fn expNB_LEFT() -> c_int /* int */ {
        unsafe {
            expNB_LEFT()
        }
    }
    fn expNB_RIGHT() -> c_int /* int */ {
        unsafe {
            expNB_RIGHT()
        }
    }
    fn expBK_HITTEST_NOWHERE() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_NOWHERE()
        }
    }
    fn expBK_HITTEST_ONICON() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_ONICON()
        }
    }
    fn expBK_HITTEST_ONLABEL() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_ONLABEL()
        }
    }
    fn expBK_HITTEST_ONITEM() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_ONITEM()
        }
    }
    fn expBK_HITTEST_ONPAGE() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_ONPAGE()
        }
    }
}
trait wxDCClipper {
}
trait wxBitmapDataObject {
    fn BitmapDataObject_Create(_bmp: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            BitmapDataObject_Create(_bmp)
        }
    }
    fn BitmapDataObject_CreateEmpty() -> *u8 /* void* */ {
        unsafe {
            BitmapDataObject_CreateEmpty()
        }
    }
    fn BitmapDataObject_Delete(_obj: *u8 /* void* */) {
        unsafe {
            BitmapDataObject_Delete(_obj)
        }
    }
    fn BitmapDataObject_GetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */) {
        unsafe {
            BitmapDataObject_GetBitmap(_obj, _bmp)
        }
    }
    fn BitmapDataObject_SetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */) {
        unsafe {
            BitmapDataObject_SetBitmap(_obj, _bmp)
        }
    }
}
trait cbCustomizeLayoutEvent {
    // missing: cbCustomizeLayoutEvent_ClickPos
}
trait wxPostScriptPrintNativeData {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxPostScriptPrintNativeData_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPostScriptPrintNativeData_Delete(_obj)
        }
    }
}
trait ELJPreviewControlBar {
    fn Create(preview: *u8 /* void* */, buttons: c_int /* int */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewControlBar_Create(preview, buttons, parent, title, arg0, arg1, arg2, arg3, style)
        }
    }
}
trait wxFFile {
}
trait wxPageSetupDialogData {
    fn Assign(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_Assign(_obj, data)
        }
    }
    fn AssignData(_obj: *u8 /* void* */, printData: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_AssignData(_obj, printData)
        }
    }
    fn CalculateIdFromPaperSize(_obj: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_CalculateIdFromPaperSize(_obj)
        }
    }
    fn CalculatePaperSizeFromId(_obj: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_CalculatePaperSizeFromId(_obj)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_Create()
        }
    }
    fn CreateFromData(printData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_CreateFromData(printData)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_Delete(_obj)
        }
    }
    fn EnableHelp(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnableHelp(_obj, flag)
        }
    }
    fn EnableMargins(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnableMargins(_obj, flag)
        }
    }
    fn EnableOrientation(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnableOrientation(_obj, flag)
        }
    }
    fn EnablePaper(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnablePaper(_obj, flag)
        }
    }
    fn EnablePrinter(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnablePrinter(_obj, flag)
        }
    }
    fn GetDefaultInfo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetDefaultInfo(_obj)
        }
    }
    fn GetDefaultMinMargins(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetDefaultMinMargins(_obj)
        }
    }
    fn GetEnableHelp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnableHelp(_obj)
        }
    }
    fn GetEnableMargins(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnableMargins(_obj)
        }
    }
    fn GetEnableOrientation(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnableOrientation(_obj)
        }
    }
    fn GetEnablePaper(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnablePaper(_obj)
        }
    }
    fn GetEnablePrinter(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnablePrinter(_obj)
        }
    }
    fn GetMarginBottomRight(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetMarginBottomRight(_obj)
        }
    }
    fn GetMarginTopLeft(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetMarginTopLeft(_obj)
        }
    }
    fn GetMinMarginBottomRight(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetMinMarginBottomRight(_obj)
        }
    }
    fn GetMinMarginTopLeft(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetMinMarginTopLeft(_obj)
        }
    }
    fn GetPaperId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPageSetupDialogData_GetPaperId(_obj)
        }
    }
    fn GetPaperSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetPaperSize(_obj)
        }
    }
    fn GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_GetPrintData(_obj, _ref)
        }
    }
    fn SetDefaultInfo(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_SetDefaultInfo(_obj, flag)
        }
    }
    fn SetDefaultMinMargins(_obj: *u8 /* void* */, flag: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetDefaultMinMargins(_obj, flag)
        }
    }
    fn SetMarginBottomRight(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetMarginBottomRight(_obj, arg0, arg1)
        }
    }
    fn SetMarginTopLeft(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetMarginTopLeft(_obj, arg0, arg1)
        }
    }
    fn SetMinMarginBottomRight(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetMinMarginBottomRight(_obj, arg0, arg1)
        }
    }
    fn SetMinMarginTopLeft(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetMinMarginTopLeft(_obj, arg0, arg1)
        }
    }
    fn SetPaperId(_obj: *u8 /* void* */, id: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_SetPaperId(_obj, id)
        }
    }
    fn SetPaperSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetPaperSize(_obj, arg0, arg1)
        }
    }
    fn SetPaperSizeId(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetPaperSizeId(_obj, id)
        }
    }
    fn SetPrintData(_obj: *u8 /* void* */, printData: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_SetPrintData(_obj, printData)
        }
    }
}
trait wxCommandLineParser {
}
trait wxLayoutConstraints {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_Create()
        }
    }
    fn bottom(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_bottom(_obj)
        }
    }
    fn centreX(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_centreX(_obj)
        }
    }
    fn centreY(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_centreY(_obj)
        }
    }
    fn height(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_height(_obj)
        }
    }
    fn left(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_left(_obj)
        }
    }
    fn right(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_right(_obj)
        }
    }
    fn top(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_top(_obj)
        }
    }
    fn width(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_width(_obj)
        }
    }
}
trait wxDrawWindow {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDrawWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait wxDataFormat {
    fn CreateFromId(name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDataFormat_CreateFromId(name)
        }
    }
    fn CreateFromType(typ: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDataFormat_CreateFromType(typ)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDataFormat_Delete(_obj)
        }
    }
    fn GetId(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDataFormat_GetId(_obj)
        }
    }
    fn GetType(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDataFormat_GetType(_obj)
        }
    }
    fn IsEqual(_obj: *u8 /* void* */, other: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDataFormat_IsEqual(_obj, other)
        }
    }
    fn SetId(_obj: *u8 /* void* */, id: *u8 /* void* */) {
        unsafe {
            wxDataFormat_SetId(_obj, id)
        }
    }
    fn SetType(_obj: *u8 /* void* */, typ: c_int /* int */) {
        unsafe {
            wxDataFormat_SetType(_obj, typ)
        }
    }
}
trait cbDockBox {
    // missing: cbDockBox_Create
}
trait wxBitmap {
    fn AddHandler(handler: *u8 /* void* */) {
        unsafe {
            wxBitmap_AddHandler(handler)
        }
    }
    fn CleanUpHandlers() {
        unsafe {
            wxBitmap_CleanUpHandlers()
        }
    }
    fn Create(_data: *u8 /* void* */, _type: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, _depth: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_Create(_data, _type, arg0, arg1, _depth)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateDefault()
        }
    }
    fn CreateEmpty(arg0: c_int /* int */, arg1: c_int /* int */, _depth: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateEmpty(arg0, arg1, _depth)
        }
    }
    fn CreateFromXPM(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateFromXPM(data)
        }
    }
    fn CreateLoad(name: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateLoad(name, type_)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxBitmap_Delete(_obj)
        }
    }
    fn FindHandlerByExtension(extension: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_FindHandlerByExtension(extension, type_)
        }
    }
    fn FindHandlerByName(name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_FindHandlerByName(name)
        }
    }
    fn FindHandlerByType(type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_FindHandlerByType(type_)
        }
    }
    fn GetDepth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmap_GetDepth(_obj)
        }
    }
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmap_GetHeight(_obj)
        }
    }
    fn GetMask(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_GetMask(_obj)
        }
    }
    fn GetSubBitmap(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmap_GetSubBitmap(_obj, arg0, arg1, arg2, arg3, _ref)
        }
    }
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmap_GetWidth(_obj)
        }
    }
    fn InitStandardHandlers() {
        unsafe {
            wxBitmap_InitStandardHandlers()
        }
    }
    fn InsertHandler(handler: *u8 /* void* */) {
        unsafe {
            wxBitmap_InsertHandler(handler)
        }
    }
    fn LoadFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxBitmap_LoadFile(_obj, name, type_)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBitmap_IsOk(_obj)
        }
    }
    fn RemoveHandler(name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBitmap_RemoveHandler(name)
        }
    }
    fn SaveFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */, cmap: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmap_SaveFile(_obj, name, type_, cmap)
        }
    }
    fn SetDepth(_obj: *u8 /* void* */, d: c_int /* int */) {
        unsafe {
            wxBitmap_SetDepth(_obj, d)
        }
    }
    fn SetHeight(_obj: *u8 /* void* */, h: c_int /* int */) {
        unsafe {
            wxBitmap_SetHeight(_obj, h)
        }
    }
    fn SetMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) {
        unsafe {
            wxBitmap_SetMask(_obj, mask)
        }
    }
    fn SetWidth(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxBitmap_SetWidth(_obj, w)
        }
    }
}
trait wxBufferedOutputStream {
}
trait cbMotionEvent {
    // missing: cbMotionEvent_Pos
}
trait cbRowInfo {
    // missing: cbRowInfo_Create
    // missing: cbRowInfo_Delete
    // missing: cbRowInfo_GetFirstBar
}
trait wxGridCellChoiceEditor {
    fn Ctor(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */, allowOthers: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellChoiceEditor_Ctor(arg0, arg1, allowOthers)
        }
    }
}
trait wxScopedPtr {
}
trait wxEraseEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxEraseEvent_CopyObject(_obj, obj)
        }
    }
    fn GetDC(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEraseEvent_GetDC(_obj)
        }
    }
}
trait wxPanel {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPanel_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn InitDialog(_obj: *u8 /* void* */) {
        unsafe {
            wxPanel_InitDialog(_obj)
        }
    }
    fn SetFocus(_obj: *u8 /* void* */) {
        unsafe {
            wxPanel_SetFocus(_obj)
        }
    }
}
trait cbFinishDrawInAreaEvent {
    // missing: cbFinishDrawInAreaEvent_Area
}
trait wxPreviewFrame {
}
trait wxFocusEvent {
}
trait cbCollapseBox {
    // missing: cbCollapseBox_Create
}
trait wxLogStream {
}
trait wxScrolledWindow {
    fn AdjustScrollbars(_obj: *u8 /* void* */) {
        unsafe {
            wxScrolledWindow_AdjustScrollbars(_obj)
        }
    }
    fn CalcScrolledPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_CalcScrolledPosition(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn CalcUnscrolledPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_CalcUnscrolledPosition(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxScrolledWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn EnableScrolling(_obj: *u8 /* void* */, x_scrolling: bool /* bool */, y_scrolling: bool /* bool */) {
        unsafe {
            wxScrolledWindow_EnableScrolling(_obj, x_scrolling, y_scrolling)
        }
    }
    fn GetScaleX(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxScrolledWindow_GetScaleX(_obj)
        }
    }
    fn GetScaleY(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxScrolledWindow_GetScaleY(_obj)
        }
    }
    fn GetScrollPageSize(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxScrolledWindow_GetScrollPageSize(_obj, orient)
        }
    }
    fn GetScrollPixelsPerUnit(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_GetScrollPixelsPerUnit(_obj, arg0, arg1)
        }
    }
    fn GetTargetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxScrolledWindow_GetTargetWindow(_obj)
        }
    }
    fn GetViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_GetViewStart(_obj, arg0, arg1)
        }
    }
    fn GetVirtualSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_GetVirtualSize(_obj, arg0, arg1)
        }
    }
    fn OnDraw(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxScrolledWindow_OnDraw(_obj, dc)
        }
    }
    fn PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxScrolledWindow_PrepareDC(_obj, dc)
        }
    }
    fn Scroll(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxScrolledWindow_Scroll(_obj, arg0, arg1)
        }
    }
    fn SetScale(_obj: *u8 /* void* */, xs: c_double /* double */, ys: c_double /* double */) {
        unsafe {
            wxScrolledWindow_SetScale(_obj, xs, ys)
        }
    }
    fn SetScrollPageSize(_obj: *u8 /* void* */, orient: c_int /* int */, pageSize: c_int /* int */) {
        unsafe {
            wxScrolledWindow_SetScrollPageSize(_obj, orient, pageSize)
        }
    }
    fn SetScrollbars(_obj: *u8 /* void* */, pixelsPerUnitX: c_int /* int */, pixelsPerUnitY: c_int /* int */, noUnitsX: c_int /* int */, noUnitsY: c_int /* int */, xPos: c_int /* int */, yPos: c_int /* int */, noRefresh: bool /* bool */) {
        unsafe {
            wxScrolledWindow_SetScrollbars(_obj, pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh)
        }
    }
    fn ShowScrollbars(_obj: *u8 /* void* */, showh: c_int /* int */, showv: c_int /* int */) {
        unsafe {
            wxScrolledWindow_ShowScrollbars(_obj, showh, showv)
        }
    }
    fn SetTargetWindow(_obj: *u8 /* void* */, target: *u8 /* void* */) {
        unsafe {
            wxScrolledWindow_SetTargetWindow(_obj, target)
        }
    }
    fn ViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_ViewStart(_obj, arg0, arg1)
        }
    }
}
trait wxSockAddress {
}
trait wxMask {
    fn Create(bitmap: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMask_Create(bitmap)
        }
    }
    fn CreateColoured(bitmap: *u8 /* void* */, colour: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMask_CreateColoured(bitmap, colour)
        }
    }
}
trait wxImageList {
    fn AddBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */, mask: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImageList_AddBitmap(_obj, bitmap, mask)
        }
    }
    fn AddIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImageList_AddIcon(_obj, icon)
        }
    }
    fn AddMasked(_obj: *u8 /* void* */, bitmap: *u8 /* void* */, maskColour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImageList_AddMasked(_obj, bitmap, maskColour)
        }
    }
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */, mask: c_int /* int */, initialCount: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImageList_Create(arg0, arg1, mask, initialCount)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxImageList_Delete(_obj)
        }
    }
    fn Draw(_obj: *u8 /* void* */, index: c_int /* int */, dc: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: c_int /* int */, solidBackground: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxImageList_Draw(_obj, index, dc, arg0, arg1, flags, solidBackground)
        }
    }
    fn GetImageCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImageList_GetImageCount(_obj)
        }
    }
    fn GetSize(_obj: *u8 /* void* */, index: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxImageList_GetSize(_obj, index, arg0, arg1)
        }
    }
    fn Remove(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxImageList_Remove(_obj, index)
        }
    }
    fn RemoveAll(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImageList_RemoveAll(_obj)
        }
    }
    fn Replace(_obj: *u8 /* void* */, index: c_int /* int */, bitmap: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImageList_Replace(_obj, index, bitmap, mask)
        }
    }
    fn ReplaceIcon(_obj: *u8 /* void* */, index: c_int /* int */, icon: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImageList_ReplaceIcon(_obj, index, icon)
        }
    }
}
trait wxGDIObject {
}
trait wxClientData {
}
trait wxTextOutputStream {
}
trait wxDynamicSashWindow {
    // missing: wxDynamicSashWindow_Create
    // missing: wxDynamicSashWindow_Delete
    // missing: wxDynamicSashWindow_GetHScrollBar
    // missing: wxDynamicSashWindow_GetVScrollBar
}
trait wxTimerRunner {
}
trait wxMDIChildFrame {
    fn Activate(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIChildFrame_Activate(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMDIChildFrame_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait wxGridEditorCreatedEvent {
    fn GetCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridEditorCreatedEvent_GetCol(_obj)
        }
    }
    fn GetControl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridEditorCreatedEvent_GetControl(_obj)
        }
    }
    fn GetRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridEditorCreatedEvent_GetRow(_obj)
        }
    }
    fn SetCol(_obj: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxGridEditorCreatedEvent_SetCol(_obj, col)
        }
    }
    fn SetControl(_obj: *u8 /* void* */, ctrl: *u8 /* void* */) {
        unsafe {
            wxGridEditorCreatedEvent_SetControl(_obj, ctrl)
        }
    }
    fn SetRow(_obj: *u8 /* void* */, row: c_int /* int */) {
        unsafe {
            wxGridEditorCreatedEvent_SetRow(_obj, row)
        }
    }
}
trait wxPostScriptDC {
    fn Create(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPostScriptDC_Create(data)
        }
    }
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxPostScriptDC_Delete(self_)
        }
    }
    fn SetResolution(self_: *u8 /* void* */, ppi: c_int /* int */) {
        unsafe {
            wxPostScriptDC_SetResolution(self_, ppi)
        }
    }
    fn GetResolution(self_: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPostScriptDC_GetResolution(self_)
        }
    }
}
trait wxColourDialog {
    fn Create(_prt: *u8 /* void* */, col: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxColourDialog_Create(_prt, col)
        }
    }
    fn GetColourData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxColourDialog_GetColourData(_obj, _ref)
        }
    }
}
trait wxSplitterWindow {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSplitterWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetBorderSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetBorderSize(_obj)
        }
    }
    fn GetMinimumPaneSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetMinimumPaneSize(_obj)
        }
    }
    fn GetSashPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetSashPosition(_obj)
        }
    }
    fn GetSashSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetSashSize(_obj)
        }
    }
    fn GetSplitMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetSplitMode(_obj)
        }
    }
    fn GetWindow1(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSplitterWindow_GetWindow1(_obj)
        }
    }
    fn GetWindow2(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSplitterWindow_GetWindow2(_obj)
        }
    }
    fn Initialize(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSplitterWindow_Initialize(_obj, window)
        }
    }
    fn IsSplit(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_IsSplit(_obj)
        }
    }
    fn ReplaceWindow(_obj: *u8 /* void* */, winOld: *u8 /* void* */, winNew: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_ReplaceWindow(_obj, winOld, winNew)
        }
    }
    fn SetBorderSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxSplitterWindow_SetBorderSize(_obj, width)
        }
    }
    fn SetMinimumPaneSize(_obj: *u8 /* void* */, min: c_int /* int */) {
        unsafe {
            wxSplitterWindow_SetMinimumPaneSize(_obj, min)
        }
    }
    fn SetSashPosition(_obj: *u8 /* void* */, position: c_int /* int */, redraw: bool /* bool */) {
        unsafe {
            wxSplitterWindow_SetSashPosition(_obj, position, redraw)
        }
    }
    fn SetSashSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxSplitterWindow_SetSashSize(_obj, width)
        }
    }
    fn SetSplitMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxSplitterWindow_SetSplitMode(_obj, mode)
        }
    }
    fn SplitHorizontally(_obj: *u8 /* void* */, window1: *u8 /* void* */, window2: *u8 /* void* */, sashPosition: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_SplitHorizontally(_obj, window1, window2, sashPosition)
        }
    }
    fn SplitVertically(_obj: *u8 /* void* */, window1: *u8 /* void* */, window2: *u8 /* void* */, sashPosition: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_SplitVertically(_obj, window1, window2, sashPosition)
        }
    }
    fn Unsplit(_obj: *u8 /* void* */, toRemove: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_Unsplit(_obj, toRemove)
        }
    }
    fn GetSashGravity(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxSplitterWindow_GetSashGravity(_obj)
        }
    }
    fn SetSashGravity(_obj: *u8 /* void* */, gravity: c_double /* double */) {
        unsafe {
            wxSplitterWindow_SetSashGravity(_obj, gravity)
        }
    }
}
trait wxWindowDC {
    fn Create(win: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindowDC_Create(win)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxWindowDC_Delete(_obj)
        }
    }
}
trait wxRegion {
    fn Assign(_obj: *u8 /* void* */, region: *u8 /* void* */) {
        unsafe {
            wxRegion_Assign(_obj, region)
        }
    }
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxRegion_Clear(_obj)
        }
    }
    fn ContainsPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_ContainsPoint(_obj, arg0, arg1)
        }
    }
    fn ContainsRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_ContainsRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxRegion_CreateDefault()
        }
    }
    fn CreateFromRect(arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxRegion_CreateFromRect(arg0, arg1, arg2, arg3)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxRegion_Delete(_obj)
        }
    }
    fn IsEmpty(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegion_IsEmpty(_obj)
        }
    }
    fn GetBox(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxRegion_GetBox(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn IntersectRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_IntersectRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn IntersectRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegion_IntersectRegion(_obj, region)
        }
    }
    fn SubtractRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_SubtractRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn SubtractRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegion_SubtractRegion(_obj, region)
        }
    }
    fn UnionRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_UnionRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn UnionRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegion_UnionRegion(_obj, region)
        }
    }
    fn XorRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_XorRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn XorRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegion_XorRegion(_obj, region)
        }
    }
}
trait wxTablesInUse {
}
trait wxProtocol {
}
trait wxMiniFrame {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMiniFrame_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait wxGridCellCoordsArray {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxGridCellCoordsArray_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellCoordsArray_Delete(_obj)
        }
    }
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridCellCoordsArray_GetCount(_obj)
        }
    }
    fn Item(_obj: *u8 /* void* */, _idx: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGridCellCoordsArray_Item(_obj, _idx, arg0, arg1)
        }
    }
}
trait wxSocketEvent {
}
trait wxGridCellTextEnterEditor {
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellTextEnterEditor_Ctor()
        }
    }
    fn wxLogStderr_Create() -> *u8 /* void* */ {
        unsafe {
            wxLogStderr_Create()
        }
    }
    fn wxLogStderr_CreateStdOut() -> *u8 /* void* */ {
        unsafe {
            wxLogStderr_CreateStdOut()
        }
    }
    fn wxLogNull_Create() -> *u8 /* void* */ {
        unsafe {
            wxLogNull_Create()
        }
    }
    fn wxLogTextCtrl_Create(text: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLogTextCtrl_Create(text)
        }
    }
    fn wxLogWindow_Create(parent: *u8 /* void* */, title: *wchar_t /* wchar_t* */, showit: bool /* bool */, passthrough: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxLogWindow_Create(parent, title, showit, passthrough)
        }
    }
    fn wxLogWindow_GetFrame(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLogWindow_GetFrame(obj)
        }
    }
    fn LogError(_msg: *u8 /* void* */) {
        unsafe {
            LogError(_msg)
        }
    }
    fn LogFatalError(_msg: *u8 /* void* */) {
        unsafe {
            LogFatalError(_msg)
        }
    }
    fn LogWarning(_msg: *u8 /* void* */) {
        unsafe {
            LogWarning(_msg)
        }
    }
    fn LogMessage(_msg: *u8 /* void* */) {
        unsafe {
            LogMessage(_msg)
        }
    }
    fn LogVerbose(_msg: *u8 /* void* */) {
        unsafe {
            LogVerbose(_msg)
        }
    }
    fn LogStatus(_msg: *u8 /* void* */) {
        unsafe {
            LogStatus(_msg)
        }
    }
    fn LogSysError(_msg: *u8 /* void* */) {
        unsafe {
            LogSysError(_msg)
        }
    }
    fn LogDebug(_msg: *u8 /* void* */) {
        unsafe {
            LogDebug(_msg)
        }
    }
    fn LogTrace(mask: *u8 /* void* */, _msg: *u8 /* void* */) {
        unsafe {
            LogTrace(mask, _msg)
        }
    }
    fn wxLog_AddTraceMask(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxLog_AddTraceMask(_obj, str)
        }
    }
    fn wxLog_Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_Delete(_obj)
        }
    }
    fn wxLog_DontCreateOnDemand(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_DontCreateOnDemand(_obj)
        }
    }
    fn wxLog_Flush(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_Flush(_obj)
        }
    }
    fn wxLog_FlushActive(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_FlushActive(_obj)
        }
    }
    fn wxLog_GetActiveTarget() -> *u8 /* void* */ {
        unsafe {
            wxLog_GetActiveTarget()
        }
    }
    fn wxLog_GetTimestamp(_obj: *u8 /* void* */) -> *char /* char* */ {
        unsafe {
            wxLog_GetTimestamp(_obj)
        }
    }
    fn wxLog_GetTraceMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxLog_GetTraceMask(_obj)
        }
    }
    fn wxLog_GetVerbose(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxLog_GetVerbose(_obj)
        }
    }
    fn wxLog_HasPendingMessages(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLog_HasPendingMessages(_obj)
        }
    }
    fn wxLog_IsAllowedTraceMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLog_IsAllowedTraceMask(_obj, mask)
        }
    }
    fn wxLog_OnLog(_obj: *u8 /* void* */, level: c_int /* int */, szString: *wchar_t /* wchar_t* */, t: c_int /* int */) {
        unsafe {
            wxLog_OnLog(_obj, level, szString, t)
        }
    }
    fn wxLog_RemoveTraceMask(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxLog_RemoveTraceMask(_obj, str)
        }
    }
    fn wxLog_Resume(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_Resume(_obj)
        }
    }
    fn wxLog_SetActiveTarget(pLogger: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLog_SetActiveTarget(pLogger)
        }
    }
    fn wxLog_SetTimestamp(_obj: *u8 /* void* */, ts: *wchar_t /* wchar_t* */) {
        unsafe {
            wxLog_SetTimestamp(_obj, ts)
        }
    }
    fn wxLog_SetTraceMask(_obj: *u8 /* void* */, ulMask: c_int /* int */) {
        unsafe {
            wxLog_SetTraceMask(_obj, ulMask)
        }
    }
    fn wxLog_SetVerbose(_obj: *u8 /* void* */, bVerbose: c_int /* int */) {
        unsafe {
            wxLog_SetVerbose(_obj, bVerbose)
        }
    }
    fn wxLog_Suspend(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_Suspend(_obj)
        }
    }
    fn wxProcess_Open(cmd: *u8 /* void* */, flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_Open(cmd, flags)
        }
    }
    fn wxProcess_IsErrorAvailable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProcess_IsErrorAvailable(_obj)
        }
    }
    fn wxProcess_IsInputAvailable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProcess_IsInputAvailable(_obj)
        }
    }
    fn wxProcess_IsInputOpened(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProcess_IsInputOpened(_obj)
        }
    }
    fn wxKill(pid: c_int /* int */, signal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxKill(pid, signal)
        }
    }
    fn wxStreamBase_Delete(obj: *u8 /* void* */) {
        unsafe {
            wxStreamBase_Delete(obj)
        }
    }
    fn wxGetColourFromUser(parent: *u8 /* void* */, colInit: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGetColourFromUser(parent, colInit, colour)
        }
    }
    fn wxGetFontFromUser(parent: *u8 /* void* */, fontInit: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxGetFontFromUser(parent, fontInit, font)
        }
    }
    fn wxGetPasswordFromUser(message: *wchar_t /* wchar_t* */, caption: *wchar_t /* wchar_t* */, defaultText: *wchar_t /* wchar_t* */, parent: *u8 /* void* */, _buf: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxGetPasswordFromUser(message, caption, defaultText, parent, _buf)
        }
    }
    fn wxGetTextFromUser(message: *wchar_t /* wchar_t* */, caption: *wchar_t /* wchar_t* */, defaultText: *wchar_t /* wchar_t* */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, center: bool /* bool */, _buf: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxGetTextFromUser(message, caption, defaultText, parent, arg0, arg1, center, _buf)
        }
    }
    fn wxGetNumberFromUser(message: *u8 /* void* */, prompt: *u8 /* void* */, caption: *u8 /* void* */, value: c_long /* long */, min: c_long /* long */, max: c_long /* long */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_long /* long */ {
        unsafe {
            wxGetNumberFromUser(message, prompt, caption, value, min, max, parent, arg0, arg1)
        }
    }
    fn wxcBell() {
        unsafe {
            wxcBell()
        }
    }
    fn wxcBeginBusyCursor() {
        unsafe {
            wxcBeginBusyCursor()
        }
    }
    fn wxcEndBusyCursor() {
        unsafe {
            wxcEndBusyCursor()
        }
    }
    fn wxcIsBusy() {
        unsafe {
            wxcIsBusy()
        }
    }
    fn wxTextCtrl_EmulateKeyPress(_obj: *u8 /* void* */, keyevent: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_EmulateKeyPress(_obj, keyevent)
        }
    }
    fn wxTextCtrl_GetDefaultStyle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetDefaultStyle(_obj)
        }
    }
    fn wxTextCtrl_GetRange(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetRange(_obj, from, to)
        }
    }
    fn wxTextCtrl_GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetStringSelection(_obj)
        }
    }
    fn wxTextCtrl_IsMultiLine(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_IsMultiLine(_obj)
        }
    }
    fn wxTextCtrl_IsSingleLine(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_IsSingleLine(_obj)
        }
    }
    fn wxTextCtrl_SetDefaultStyle(_obj: *u8 /* void* */, style: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_SetDefaultStyle(_obj, style)
        }
    }
    fn wxTextCtrl_SetMaxLength(_obj: *u8 /* void* */, len: c_long /* long */) {
        unsafe {
            wxTextCtrl_SetMaxLength(_obj, len)
        }
    }
    fn wxTextCtrl_SetStyle(_obj: *u8 /* void* */, start: c_long /* long */, end: c_long /* long */, style: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_SetStyle(_obj, start, end, style)
        }
    }
    fn wxTextAttr_Create(colText: *u8 /* void* */, colBack: *u8 /* void* */, font: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextAttr_Create(colText, colBack, font)
        }
    }
    fn wxTextAttr_CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxTextAttr_CreateDefault()
        }
    }
    fn wxTextAttr_Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxTextAttr_Delete(_obj)
        }
    }
    fn wxTextAttr_GetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxTextAttr_GetBackgroundColour(_obj, colour)
        }
    }
    fn wxTextAttr_GetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxTextAttr_GetFont(_obj, font)
        }
    }
    fn wxTextAttr_GetTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxTextAttr_GetTextColour(_obj, colour)
        }
    }
    fn wxTextAttr_HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextAttr_HasBackgroundColour(_obj)
        }
    }
    fn wxTextAttr_HasFont(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextAttr_HasFont(_obj)
        }
    }
    fn wxTextAttr_HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextAttr_HasTextColour(_obj)
        }
    }
    fn wxTextAttr_IsDefault(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextAttr_IsDefault(_obj)
        }
    }
    fn wxTextAttr_SetTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxTextAttr_SetTextColour(_obj, colour)
        }
    }
    fn wxTextAttr_SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxTextAttr_SetBackgroundColour(_obj, colour)
        }
    }
    fn wxTextAttr_SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxTextAttr_SetFont(_obj, font)
        }
    }
}
trait wxDebugContext {
}
trait wxLayoutAlgorithm {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxLayoutAlgorithm_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxLayoutAlgorithm_Delete(_obj)
        }
    }
    fn LayoutFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLayoutAlgorithm_LayoutFrame(_obj, frame, mainWindow)
        }
    }
    fn LayoutMDIFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, use_: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxLayoutAlgorithm_LayoutMDIFrame(_obj, frame, arg0, arg1, arg2, arg3, use_)
        }
    }
    fn LayoutWindow(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLayoutAlgorithm_LayoutWindow(_obj, frame, mainWindow)
        }
    }
}
trait wxStringProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStringProperty_Create(label, name, value)
        }
    }
}
trait wxObjectRefData {
}
trait wxRadioBox {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _dim: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxRadioBox_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, arg4, arg5, _dim, _stl)
        }
    }
    fn EnableItem(_obj: *u8 /* void* */, item: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxRadioBox_EnableItem(_obj, item, enable)
        }
    }
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRadioBox_FindString(_obj, s)
        }
    }
    fn GetItemLabel(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxRadioBox_GetItemLabel(_obj, item)
        }
    }
    fn GetNumberOfRowsOrCols(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRadioBox_GetNumberOfRowsOrCols(_obj)
        }
    }
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRadioBox_GetSelection(_obj)
        }
    }
    fn GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxRadioBox_GetStringSelection(_obj)
        }
    }
    fn Number(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRadioBox_Number(_obj)
        }
    }
    fn SetItemBitmap(_obj: *u8 /* void* */, item: c_int /* int */, bitmap: *u8 /* void* */) {
        unsafe {
            wxRadioBox_SetItemBitmap(_obj, item, bitmap)
        }
    }
    fn SetItemLabel(_obj: *u8 /* void* */, item: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxRadioBox_SetItemLabel(_obj, item, label)
        }
    }
    fn SetNumberOfRowsOrCols(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxRadioBox_SetNumberOfRowsOrCols(_obj, n)
        }
    }
    fn SetSelection(_obj: *u8 /* void* */, _n: c_int /* int */) {
        unsafe {
            wxRadioBox_SetSelection(_obj, _n)
        }
    }
    fn SetStringSelection(_obj: *u8 /* void* */, s: *u8 /* void* */) {
        unsafe {
            wxRadioBox_SetStringSelection(_obj, s)
        }
    }
    fn ShowItem(_obj: *u8 /* void* */, item: c_int /* int */, show: bool /* bool */) {
        unsafe {
            wxRadioBox_ShowItem(_obj, item, show)
        }
    }
}
trait wxArrayString {
}
trait wxTopLevelWindow {
    fn EnableCloseButton(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_EnableCloseButton(_obj, enable)
        }
    }
    fn GetDefaultButton(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTopLevelWindow_GetDefaultButton(_obj)
        }
    }
    fn GetDefaultItem(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTopLevelWindow_GetDefaultItem(_obj)
        }
    }
    fn GetIcon(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTopLevelWindow_GetIcon(_obj)
        }
    }
    fn GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTopLevelWindow_GetTitle(_obj)
        }
    }
    fn Iconize(_obj: *u8 /* void* */, iconize: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_Iconize(_obj, iconize)
        }
    }
    fn IsActive(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_IsActive(_obj)
        }
    }
    fn IsIconized(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_IsIconized(_obj)
        }
    }
    fn IsMaximized(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_IsMaximized(_obj)
        }
    }
    fn Maximize(_obj: *u8 /* void* */, maximize: bool /* bool */) {
        unsafe {
            wxTopLevelWindow_Maximize(_obj, maximize)
        }
    }
    fn RequestUserAttention(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxTopLevelWindow_RequestUserAttention(_obj, flags)
        }
    }
    fn SetDefaultButton(_obj: *u8 /* void* */, pBut: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetDefaultButton(_obj, pBut)
        }
    }
    fn SetDefaultItem(_obj: *u8 /* void* */, pBut: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetDefaultItem(_obj, pBut)
        }
    }
    fn SetIcon(_obj: *u8 /* void* */, pIcon: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetIcon(_obj, pIcon)
        }
    }
    fn SetIcons(_obj: *u8 /* void* */, _icons: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetIcons(_obj, _icons)
        }
    }
    fn SetMaxSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxTopLevelWindow_SetMaxSize(_obj, arg0, arg1)
        }
    }
    fn SetMinSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxTopLevelWindow_SetMinSize(_obj, arg0, arg1)
        }
    }
    fn SetTitle(_obj: *u8 /* void* */, pString: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetTitle(_obj, pString)
        }
    }
}
trait ELJConnection {
    // missing: ELJConnection_Advise
    // missing: ELJConnection_Compress
    // missing: ELJConnection_Create
    // missing: ELJConnection_CreateDefault
    // missing: ELJConnection_Delete
    // missing: ELJConnection_Disconnect
    // missing: ELJConnection_Execute
    // missing: ELJConnection_Poke
    // missing: ELJConnection_Request
    // missing: ELJConnection_SetOnAdvise
    // missing: ELJConnection_SetOnDisconnect
    // missing: ELJConnection_SetOnExecute
    // missing: ELJConnection_SetOnPoke
    // missing: ELJConnection_SetOnRequest
    // missing: ELJConnection_SetOnStartAdvise
    // missing: ELJConnection_SetOnStopAdvise
    // missing: ELJConnection_StartAdvise
    // missing: ELJConnection_StopAdvise
}
trait wxProcess {
    fn CloseOutput(_obj: *u8 /* void* */) {
        unsafe {
            wxProcess_CloseOutput(_obj)
        }
    }
    fn CreateDefault(_prt: *u8 /* void* */, _id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_CreateDefault(_prt, _id)
        }
    }
    fn CreateRedirect(_prt: *u8 /* void* */, _rdr: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_CreateRedirect(_prt, _rdr)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxProcess_Delete(_obj)
        }
    }
    fn Detach(_obj: *u8 /* void* */) {
        unsafe {
            wxProcess_Detach(_obj)
        }
    }
    fn GetErrorStream(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_GetErrorStream(_obj)
        }
    }
    fn GetInputStream(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_GetInputStream(_obj)
        }
    }
    fn GetOutputStream(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_GetOutputStream(_obj)
        }
    }
    fn IsRedirected(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProcess_IsRedirected(_obj)
        }
    }
    fn Redirect(_obj: *u8 /* void* */) {
        unsafe {
            wxProcess_Redirect(_obj)
        }
    }
}
trait wxDbConnectInf {
}
trait wxHtmlWinTagHandler {
}
trait wxVariantData {
}
trait wxServerBase {
}
trait wxDbTable {
}
trait wxDbInf {
}
trait cbRowLayoutPlugin {
    // missing: cbRowLayoutPlugin_Create
    // missing: cbRowLayoutPlugin_CreateDefault
    // missing: cbRowLayoutPlugin_Delete
}
trait cbResizeRowEvent {
    // missing: cbResizeRowEvent_ForUpperHandle
    // missing: cbResizeRowEvent_HandleOfs
    // missing: cbResizeRowEvent_Row
}
trait wxInputStream {
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxInputStream_Delete(_obj)
        }
    }
    fn Eof(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxInputStream_Eof(_obj)
        }
    }
    fn GetC(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxInputStream_GetC(_obj)
        }
    }
    fn LastRead(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputStream_LastRead(_obj)
        }
    }
    fn Peek(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxInputStream_Peek(_obj)
        }
    }
    fn Read(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */) {
        unsafe {
            wxInputStream_Read(_obj, buffer, size)
        }
    }
    fn SeekI(_obj: *u8 /* void* */, pos: c_int /* int */, mode: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxInputStream_SeekI(_obj, pos, mode)
        }
    }
    fn Tell(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputStream_Tell(_obj)
        }
    }
    fn UngetBuffer(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxInputStream_UngetBuffer(_obj, buffer, size)
        }
    }
    fn Ungetch(_obj: *u8 /* void* */, c: wchar_t /* wchar_t */) -> c_int /* int */ {
        unsafe {
            wxInputStream_Ungetch(_obj, c)
        }
    }
}
trait wxLogGUI {
}
trait wxSlider {
    fn ClearSel(_obj: *u8 /* void* */) {
        unsafe {
            wxSlider_ClearSel(_obj)
        }
    }
    fn ClearTicks(_obj: *u8 /* void* */) {
        unsafe {
            wxSlider_ClearTicks(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _init: c_int /* int */, _min: c_int /* int */, _max: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxSlider_Create(_prt, _id, _init, _min, _max, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetLineSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetLineSize(_obj)
        }
    }
    fn GetMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetMax(_obj)
        }
    }
    fn GetMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetMin(_obj)
        }
    }
    fn GetPageSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetPageSize(_obj)
        }
    }
    fn GetSelEnd(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetSelEnd(_obj)
        }
    }
    fn GetSelStart(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetSelStart(_obj)
        }
    }
    fn GetThumbLength(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetThumbLength(_obj)
        }
    }
    fn GetTickFreq(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetTickFreq(_obj)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetValue(_obj)
        }
    }
    fn SetLineSize(_obj: *u8 /* void* */, lineSize: c_int /* int */) {
        unsafe {
            wxSlider_SetLineSize(_obj, lineSize)
        }
    }
    fn SetPageSize(_obj: *u8 /* void* */, pageSize: c_int /* int */) {
        unsafe {
            wxSlider_SetPageSize(_obj, pageSize)
        }
    }
    fn SetRange(_obj: *u8 /* void* */, minValue: c_int /* int */, maxValue: c_int /* int */) {
        unsafe {
            wxSlider_SetRange(_obj, minValue, maxValue)
        }
    }
    fn SetSelection(_obj: *u8 /* void* */, minPos: c_int /* int */, maxPos: c_int /* int */) {
        unsafe {
            wxSlider_SetSelection(_obj, minPos, maxPos)
        }
    }
    fn SetThumbLength(_obj: *u8 /* void* */, len: c_int /* int */) {
        unsafe {
            wxSlider_SetThumbLength(_obj, len)
        }
    }
    fn SetTick(_obj: *u8 /* void* */, tickPos: c_int /* int */) {
        unsafe {
            wxSlider_SetTick(_obj, tickPos)
        }
    }
    fn SetTickFreq(_obj: *u8 /* void* */, n: c_int /* int */, pos: c_int /* int */) {
        unsafe {
            wxSlider_SetTickFreq(_obj, n, pos)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, value: c_int /* int */) {
        unsafe {
            wxSlider_SetValue(_obj, value)
        }
    }
}
trait wxAcceleratorTable {
    fn Create(n: c_int /* int */, entries: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxAcceleratorTable_Create(n, entries)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxAcceleratorTable_Delete(_obj)
        }
    }
}
trait wxStringList {
}
trait wxCalendarEvent {
    fn GetDate(_obj: *u8 /* void* */, _dte: *u8 /* void* */) {
        unsafe {
            wxCalendarEvent_GetDate(_obj, _dte)
        }
    }
    fn GetWeekDay(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCalendarEvent_GetWeekDay(_obj)
        }
    }
}
trait wxPlotOnOffCurve {
    // missing: wxPlotOnOffCurve_Add
    // missing: wxPlotOnOffCurve_Create
    // missing: wxPlotOnOffCurve_Delete
    // missing: wxPlotOnOffCurve_DrawOffLine
    // missing: wxPlotOnOffCurve_DrawOnLine
    // missing: wxPlotOnOffCurve_GetAt
    // missing: wxPlotOnOffCurve_GetClientData
    // missing: wxPlotOnOffCurve_GetCount
    // missing: wxPlotOnOffCurve_GetEndX
    // missing: wxPlotOnOffCurve_GetOff
    // missing: wxPlotOnOffCurve_GetOffsetY
    // missing: wxPlotOnOffCurve_GetOn
    // missing: wxPlotOnOffCurve_GetStartX
    // missing: wxPlotOnOffCurve_SetOffsetY
}
trait wxStaticText {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStaticText_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait wxClient {
}
trait wxSplashScreen {
}
trait wxHtmlHelpController {
    fn AddBook(_obj: *u8 /* void* */, book: *u8 /* void* */, show_wait_msg: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_AddBook(_obj, book, show_wait_msg)
        }
    }
    fn Create(_style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlHelpController_Create(_style)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_Delete(_obj)
        }
    }
    fn Display(_obj: *u8 /* void* */, x: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxHtmlHelpController_Display(_obj, x)
        }
    }
    fn DisplayBlock(_obj: *u8 /* void* */, blockNo: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_DisplayBlock(_obj, blockNo)
        }
    }
    fn DisplayContents(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxHtmlHelpController_DisplayContents(_obj)
        }
    }
    fn DisplayIndex(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxHtmlHelpController_DisplayIndex(_obj)
        }
    }
    fn DisplayNumber(_obj: *u8 /* void* */, id: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxHtmlHelpController_DisplayNumber(_obj, id)
        }
    }
    fn DisplaySection(_obj: *u8 /* void* */, section: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_DisplaySection(_obj, section)
        }
    }
    fn DisplaySectionNumber(_obj: *u8 /* void* */, sectionNo: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_DisplaySectionNumber(_obj, sectionNo)
        }
    }
    fn GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlHelpController_GetFrame(_obj)
        }
    }
    fn GetFrameParameters(_obj: *u8 /* void* */, title: *u8 /* void* */, width: *c_int /* int* */, height: *c_int /* int* */, pos_x: *c_int /* int* */, pos_y: *c_int /* int* */, newFrameEachTime: *c_int /* int* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlHelpController_GetFrameParameters(_obj, title, width, height, pos_x, pos_y, newFrameEachTime)
        }
    }
    fn Initialize(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_Initialize(_obj, file)
        }
    }
    fn KeywordSearch(_obj: *u8 /* void* */, keyword: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_KeywordSearch(_obj, keyword)
        }
    }
    fn LoadFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_LoadFile(_obj, file)
        }
    }
    fn Quit(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_Quit(_obj)
        }
    }
    fn ReadCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_ReadCustomization(_obj, cfg, path)
        }
    }
    fn SetFrameParameters(_obj: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, pos_x: c_int /* int */, pos_y: c_int /* int */, newFrameEachTime: bool /* bool */) {
        unsafe {
            wxHtmlHelpController_SetFrameParameters(_obj, title, arg0, arg1, pos_x, pos_y, newFrameEachTime)
        }
    }
    fn SetTempDir(_obj: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_SetTempDir(_obj, path)
        }
    }
    fn SetTitleFormat(_obj: *u8 /* void* */, format: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_SetTitleFormat(_obj, format)
        }
    }
    fn SetViewer(_obj: *u8 /* void* */, viewer: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxHtmlHelpController_SetViewer(_obj, viewer, flags)
        }
    }
    fn UseConfig(_obj: *u8 /* void* */, config: *u8 /* void* */, rootpath: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_UseConfig(_obj, config, rootpath)
        }
    }
    fn WriteCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_WriteCustomization(_obj, cfg, path)
        }
    }
}
trait wxSocketBase {
}
trait wxRemotelyScrolledTreeCtrl {
    // missing: wxRemotelyScrolledTreeCtrl_AdjustRemoteScrollbars
    // missing: wxRemotelyScrolledTreeCtrl_CalcTreeSize
    // missing: wxRemotelyScrolledTreeCtrl_CalcTreeSizeItem
    // missing: wxRemotelyScrolledTreeCtrl_Create
    // missing: wxRemotelyScrolledTreeCtrl_Delete
    // missing: wxRemotelyScrolledTreeCtrl_GetCompanionWindow
    // missing: wxRemotelyScrolledTreeCtrl_GetScrollPos
    // missing: wxRemotelyScrolledTreeCtrl_GetScrolledWindow
    // missing: wxRemotelyScrolledTreeCtrl_GetViewStart
    // missing: wxRemotelyScrolledTreeCtrl_HideVScrollbar
    // missing: wxRemotelyScrolledTreeCtrl_PrepareDC
    // missing: wxRemotelyScrolledTreeCtrl_ScrollToLine
    // missing: wxRemotelyScrolledTreeCtrl_SetCompanionWindow
    // missing: wxRemotelyScrolledTreeCtrl_SetScrollbars
}
trait ELJTextDropTarget {
    fn Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJTextDropTarget_Create(_obj, _func)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_Delete(_obj)
        }
    }
    fn SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnData(_obj, _func)
        }
    }
    fn SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnDragOver(_obj, _func)
        }
    }
    fn SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnDrop(_obj, _func)
        }
    }
    fn SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnEnter(_obj, _func)
        }
    }
    fn SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnLeave(_obj, _func)
        }
    }
}
trait wxAutoBufferedPaintDC {
    fn Create(window: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxAutoBufferedPaintDC_Create(window)
        }
    }
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxAutoBufferedPaintDC_Delete(self_)
        }
    }
}
trait wxFontData {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxFontData_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFontData_Delete(_obj)
        }
    }
    fn EnableEffects(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxFontData_EnableEffects(_obj, flag)
        }
    }
    fn GetAllowSymbols(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontData_GetAllowSymbols(_obj)
        }
    }
    fn GetChosenFont(_obj: *u8 /* void* */, ref_: *u8 /* void* */) {
        unsafe {
            wxFontData_GetChosenFont(_obj, ref_)
        }
    }
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxFontData_GetColour(_obj, _ref)
        }
    }
    fn GetEnableEffects(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontData_GetEnableEffects(_obj)
        }
    }
    fn GetEncoding(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFontData_GetEncoding(_obj)
        }
    }
    fn GetInitialFont(_obj: *u8 /* void* */, ref_: *u8 /* void* */) {
        unsafe {
            wxFontData_GetInitialFont(_obj, ref_)
        }
    }
    fn GetShowHelp(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFontData_GetShowHelp(_obj)
        }
    }
    fn SetAllowSymbols(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxFontData_SetAllowSymbols(_obj, flag)
        }
    }
    fn SetChosenFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxFontData_SetChosenFont(_obj, font)
        }
    }
    fn SetColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxFontData_SetColour(_obj, colour)
        }
    }
    fn SetEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */) {
        unsafe {
            wxFontData_SetEncoding(_obj, encoding)
        }
    }
    fn SetInitialFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxFontData_SetInitialFont(_obj, font)
        }
    }
    fn SetRange(_obj: *u8 /* void* */, minRange: c_int /* int */, maxRange: c_int /* int */) {
        unsafe {
            wxFontData_SetRange(_obj, minRange, maxRange)
        }
    }
    fn SetShowHelp(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxFontData_SetShowHelp(_obj, flag)
        }
    }
}
trait wxButton {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxButton_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxButton_SetBackgroundColour(_obj, colour)
        }
    }
    fn SetDefault(_obj: *u8 /* void* */) {
        unsafe {
            wxButton_SetDefault(_obj)
        }
    }
}
trait wxGauge {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _rng: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGauge_Create(_prt, _id, _rng, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetBezelFace(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGauge_GetBezelFace(_obj)
        }
    }
    fn GetRange(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGauge_GetRange(_obj)
        }
    }
    fn GetShadowWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGauge_GetShadowWidth(_obj)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGauge_GetValue(_obj)
        }
    }
    fn SetBezelFace(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxGauge_SetBezelFace(_obj, w)
        }
    }
    fn SetRange(_obj: *u8 /* void* */, r: c_int /* int */) {
        unsafe {
            wxGauge_SetRange(_obj, r)
        }
    }
    fn SetShadowWidth(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxGauge_SetShadowWidth(_obj, w)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxGauge_SetValue(_obj, pos)
        }
    }
}
trait wxBrush {
    fn Assign(_obj: *u8 /* void* */, brush: *u8 /* void* */) {
        unsafe {
            wxBrush_Assign(_obj, brush)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxBrush_CreateDefault()
        }
    }
    fn CreateFromBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBrush_CreateFromBitmap(bitmap)
        }
    }
    fn CreateFromColour(col: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBrush_CreateFromColour(col, style)
        }
    }
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBrush_CreateFromStock(id)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxBrush_Delete(_obj)
        }
    }
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBrush_GetColour(_obj, _ref)
        }
    }
    fn GetStipple(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBrush_GetStipple(_obj, _ref)
        }
    }
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBrush_GetStyle(_obj)
        }
    }
    fn IsEqual(_obj: *u8 /* void* */, brush: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBrush_IsEqual(_obj, brush)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBrush_IsOk(_obj)
        }
    }
    fn SetColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxBrush_SetColour(_obj, col)
        }
    }
    fn SetColourSingle(_obj: *u8 /* void* */, r: wchar_t /* wchar_t */, g: wchar_t /* wchar_t */, b: wchar_t /* wchar_t */) {
        unsafe {
            wxBrush_SetColourSingle(_obj, r, g, b)
        }
    }
    fn SetStipple(_obj: *u8 /* void* */, stipple: *u8 /* void* */) {
        unsafe {
            wxBrush_SetStipple(_obj, stipple)
        }
    }
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxBrush_SetStyle(_obj, style)
        }
    }
}
trait wxTreeCtrl {
    fn AddRoot(_obj: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_AddRoot(_obj, text, image, selectedImage, data, _item)
        }
    }
    fn AppendItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_AppendItem(_obj, parent, text, image, selectedImage, data, _item)
        }
    }
    fn Collapse(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Collapse(_obj, item)
        }
    }
    fn CollapseAndReset(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_CollapseAndReset(_obj, item)
        }
    }
    fn Create(_obj: *u8 /* void* */, _cmp: *u8 /* void* */, _prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_Create(_obj, _cmp, _prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn Delete(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Delete(_obj, item)
        }
    }
    fn DeleteAllItems(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_DeleteAllItems(_obj)
        }
    }
    fn DeleteChildren(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_DeleteChildren(_obj, item)
        }
    }
    fn EditLabel(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_EditLabel(_obj, item)
        }
    }
    fn EndEditLabel(_obj: *u8 /* void* */, item: *u8 /* void* */, discardChanges: bool /* bool */) {
        unsafe {
            wxTreeCtrl_EndEditLabel(_obj, item, discardChanges)
        }
    }
    fn EnsureVisible(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_EnsureVisible(_obj, item)
        }
    }
    fn Expand(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Expand(_obj, item)
        }
    }
    fn GetBoundingRect(_obj: *u8 /* void* */, item: *u8 /* void* */, textOnly: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetBoundingRect(_obj, item, textOnly)
        }
    }
    fn GetChildrenCount(_obj: *u8 /* void* */, item: *u8 /* void* */, recursively: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetChildrenCount(_obj, item, recursively)
        }
    }
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetCount(_obj)
        }
    }
    fn GetEditControl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetEditControl(_obj)
        }
    }
    fn GetFirstChild(_obj: *u8 /* void* */, item: *u8 /* void* */, cookie: *c_int /* int* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetFirstChild(_obj, item, cookie, _item)
        }
    }
    fn GetFirstVisibleItem(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetFirstVisibleItem(_obj, item, _item)
        }
    }
    fn GetImageList(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetImageList(_obj)
        }
    }
    fn GetIndent(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetIndent(_obj)
        }
    }
    fn GetItemData(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetItemData(_obj, item)
        }
    }
    fn GetItemImage(_obj: *u8 /* void* */, item: *u8 /* void* */, which: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetItemImage(_obj, item, which)
        }
    }
    fn GetItemText(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetItemText(_obj, item)
        }
    }
    fn GetLastChild(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetLastChild(_obj, item, _item)
        }
    }
    fn GetNextChild(_obj: *u8 /* void* */, item: *u8 /* void* */, cookie: *c_int /* int* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetNextChild(_obj, item, cookie, _item)
        }
    }
    fn GetNextSibling(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetNextSibling(_obj, item, _item)
        }
    }
    fn GetNextVisible(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetNextVisible(_obj, item, _item)
        }
    }
    fn GetParent(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetParent(_obj, item, _item)
        }
    }
    fn GetPrevSibling(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetPrevSibling(_obj, item, _item)
        }
    }
    fn GetPrevVisible(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetPrevVisible(_obj, item, _item)
        }
    }
    fn GetRootItem(_obj: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetRootItem(_obj, _item)
        }
    }
    fn GetSelection(_obj: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetSelection(_obj, _item)
        }
    }
    fn GetSelections(_obj: *u8 /* void* */, selections: *intptr_t /* intptr_t* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetSelections(_obj, selections)
        }
    }
    fn GetSpacing(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetSpacing(_obj)
        }
    }
    fn GetStateImageList(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetStateImageList(_obj)
        }
    }
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *c_int /* int* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_HitTest(_obj, arg0, arg1, flags, _item)
        }
    }
    fn InsertItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, idPrevious: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_InsertItem(_obj, parent, idPrevious, text, image, selectedImage, data, _item)
        }
    }
    fn InsertItemByIndex(_obj: *u8 /* void* */, parent: *u8 /* void* */, index: c_int /* int */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_InsertItemByIndex(_obj, parent, index, text, image, selectedImage, data, _item)
        }
    }
    fn IsBold(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeCtrl_IsBold(_obj, item)
        }
    }
    fn IsExpanded(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeCtrl_IsExpanded(_obj, item)
        }
    }
    fn IsSelected(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeCtrl_IsSelected(_obj, item)
        }
    }
    fn IsVisible(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeCtrl_IsVisible(_obj, item)
        }
    }
    fn ItemHasChildren(_obj: *u8 /* void* */, item: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_ItemHasChildren(_obj, item)
        }
    }
    fn OnCompareItems(_obj: *u8 /* void* */, item1: *u8 /* void* */, item2: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_OnCompareItems(_obj, item1, item2)
        }
    }
    fn PrependItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_PrependItem(_obj, parent, text, image, selectedImage, data, _item)
        }
    }
    fn ScrollTo(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_ScrollTo(_obj, item)
        }
    }
    fn SelectItem(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SelectItem(_obj, item)
        }
    }
    fn SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetImageList(_obj, imageList)
        }
    }
    fn SetIndent(_obj: *u8 /* void* */, indent: c_int /* int */) {
        unsafe {
            wxTreeCtrl_SetIndent(_obj, indent)
        }
    }
    fn SetItemBackgroundColour(_obj: *u8 /* void* */, item: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemBackgroundColour(_obj, item, col)
        }
    }
    fn SetItemBold(_obj: *u8 /* void* */, item: *u8 /* void* */, bold: bool /* bool */) {
        unsafe {
            wxTreeCtrl_SetItemBold(_obj, item, bold)
        }
    }
    fn SetItemData(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemData(_obj, item, data)
        }
    }
    fn SetItemDropHighlight(_obj: *u8 /* void* */, item: *u8 /* void* */, highlight: bool /* bool */) {
        unsafe {
            wxTreeCtrl_SetItemDropHighlight(_obj, item, highlight)
        }
    }
    fn SetItemFont(_obj: *u8 /* void* */, item: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemFont(_obj, item, font)
        }
    }
    fn SetItemHasChildren(_obj: *u8 /* void* */, item: *u8 /* void* */, hasChildren: bool /* bool */) {
        unsafe {
            wxTreeCtrl_SetItemHasChildren(_obj, item, hasChildren)
        }
    }
    fn SetItemImage(_obj: *u8 /* void* */, item: *u8 /* void* */, image: c_int /* int */, which: c_int /* int */) {
        unsafe {
            wxTreeCtrl_SetItemImage(_obj, item, image, which)
        }
    }
    fn SetItemText(_obj: *u8 /* void* */, item: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemText(_obj, item, text)
        }
    }
    fn SetItemTextColour(_obj: *u8 /* void* */, item: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemTextColour(_obj, item, col)
        }
    }
    fn SetSpacing(_obj: *u8 /* void* */, spacing: c_int /* int */) {
        unsafe {
            wxTreeCtrl_SetSpacing(_obj, spacing)
        }
    }
    fn SetStateImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetStateImageList(_obj, imageList)
        }
    }
    fn SortChildren(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SortChildren(_obj, item)
        }
    }
    fn Toggle(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Toggle(_obj, item)
        }
    }
    fn Unselect(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Unselect(_obj)
        }
    }
    fn UnselectAll(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_UnselectAll(_obj)
        }
    }
}
trait wxNotifyEvent {
    fn Allow(_obj: *u8 /* void* */) {
        unsafe {
            wxNotifyEvent_Allow(_obj)
        }
    }
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxNotifyEvent_CopyObject(_obj, object_dest)
        }
    }
    fn IsAllowed(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNotifyEvent_IsAllowed(_obj)
        }
    }
    fn Veto(_obj: *u8 /* void* */) {
        unsafe {
            wxNotifyEvent_Veto(_obj)
        }
    }
}
trait wxQuantize {
}
trait cbMiniButton {
    // missing: cbMiniButton_Create
    // missing: cbMiniButton_Delete
    // missing: cbMiniButton_Dim
    // missing: cbMiniButton_DragStarted
    // missing: cbMiniButton_Enable
    // missing: cbMiniButton_Enabled
    // missing: cbMiniButton_HitTest
    // missing: cbMiniButton_IsPressed
    // missing: cbMiniButton_Layout
    // missing: cbMiniButton_Pane
    // missing: cbMiniButton_Plugin
    // missing: cbMiniButton_Pos
    // missing: cbMiniButton_Pressed
    // missing: cbMiniButton_Refresh
    // missing: cbMiniButton_Reset
    // missing: cbMiniButton_SetPos
    // missing: cbMiniButton_Visible
    // missing: cbMiniButton_WasClicked
    // missing: cbMiniButton_Wnd
}
trait wxPopupWindow {
}
trait wxFileOutputStream {
}
trait wxFileConfig {
    fn wxConfigBase_Get() -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_Get()
        }
    }
    fn wxConfigBase_Set(self_: *u8 /* void* */) {
        unsafe {
            wxConfigBase_Set(self_)
        }
    }
    fn Create(inp: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileConfig_Create(inp)
        }
    }
    fn wxBitmap_CreateFromImage(image: *u8 /* void* */, depth: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateFromImage(image, depth)
        }
    }
    fn wxImage_CreateFromDataEx(arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */, isStaticData: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromDataEx(arg0, arg1, data, isStaticData)
        }
    }
    fn wxImage_Delete(image: *u8 /* void* */) {
        unsafe {
            wxImage_Delete(image)
        }
    }
    fn wxColour_CreateFromInt(rgb: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateFromInt(rgb)
        }
    }
    fn wxColour_GetInt(colour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxColour_GetInt(colour)
        }
    }
    fn wxColour_CreateFromUnsignedInt(rgba: uint32_t /* uint32_t */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateFromUnsignedInt(rgba)
        }
    }
    fn wxColour_GetUnsignedInt(colour: *u8 /* void* */) -> uint32_t /* uint32_t */ {
        unsafe {
            wxColour_GetUnsignedInt(colour)
        }
    }
    fn wxcSystemSettingsGetColour(systemColour: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxcSystemSettingsGetColour(systemColour)
        }
    }
    fn wxcSetPixelRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgb: c_int /* int */) {
        unsafe {
            wxcSetPixelRGB(buffer, width, arg0, arg1, rgb)
        }
    }
    fn wxcGetPixelRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxcGetPixelRGB(buffer, width, arg0, arg1)
        }
    }
    fn wxcSetPixelRowRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgbStart: c_int /* int */, rgbEnd: c_int /* int */, count: c_int /* int */) {
        unsafe {
            wxcSetPixelRowRGB(buffer, width, arg0, arg1, rgbStart, rgbEnd, count)
        }
    }
    fn wxcInitPixelsRGB(buffer: *uint8_t /* uint8_t* */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: c_int /* int */) {
        unsafe {
            wxcInitPixelsRGB(buffer, arg0, arg1, rgba)
        }
    }
    fn wxcSetPixelRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: uint32_t /* uint32_t */) {
        unsafe {
            wxcSetPixelRGBA(buffer, width, arg0, arg1, rgba)
        }
    }
    fn wxcGetPixelRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> uint32_t /* uint32_t */ {
        unsafe {
            wxcGetPixelRGBA(buffer, width, arg0, arg1)
        }
    }
    fn wxcSetPixelRowRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgbaStart: c_int /* int */, rgbEnd: c_int /* int */, count: uint32_t /* uint32_t */) {
        unsafe {
            wxcSetPixelRowRGBA(buffer, width, arg0, arg1, rgbaStart, rgbEnd, count)
        }
    }
    fn wxcInitPixelsRGBA(buffer: *uint8_t /* uint8_t* */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: uint32_t /* uint32_t */) {
        unsafe {
            wxcInitPixelsRGBA(buffer, arg0, arg1, rgba)
        }
    }
    fn wxcMalloc(size: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxcMalloc(size)
        }
    }
    fn wxcFree(p: *u8 /* void* */) {
        unsafe {
            wxcFree(p)
        }
    }
    fn wxcWakeUpIdle() {
        unsafe {
            wxcWakeUpIdle()
        }
    }
    fn wxGetApplicationDir() -> *u8 /* void* */ {
        unsafe {
            wxGetApplicationDir()
        }
    }
    fn wxGetApplicationPath() -> *u8 /* void* */ {
        unsafe {
            wxGetApplicationPath()
        }
    }
    fn ELJApp_InitializeC(closure: *u8 /* void* */, _argc: c_int /* int */, _argv: *wchar_t /* wchar_t* */) {
        unsafe {
            ELJApp_InitializeC(closure, _argc, _argv)
        }
    }
    fn ELJApp_GetIdleInterval() -> c_int /* int */ {
        unsafe {
            ELJApp_GetIdleInterval()
        }
    }
    fn ELJApp_SetIdleInterval(interval: c_int /* int */) {
        unsafe {
            ELJApp_SetIdleInterval(interval)
        }
    }
}
trait wxExprDatabase {
}
trait wxHelpEvent {
    fn GetLink(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpEvent_GetLink(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpEvent_GetPosition(_obj)
        }
    }
    fn GetTarget(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpEvent_GetTarget(_obj)
        }
    }
    fn SetLink(_obj: *u8 /* void* */, link: *u8 /* void* */) {
        unsafe {
            wxHelpEvent_SetLink(_obj, link)
        }
    }
    fn SetPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxHelpEvent_SetPosition(_obj, arg0, arg1)
        }
    }
    fn SetTarget(_obj: *u8 /* void* */, target: *u8 /* void* */) {
        unsafe {
            wxHelpEvent_SetTarget(_obj, target)
        }
    }
}
trait cbRightUpEvent {
    // missing: cbRightUpEvent_Pos
}
trait wxToolBar {
    fn AddControl(_obj: *u8 /* void* */, ctrl: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxToolBar_AddControl(_obj, ctrl)
        }
    }
    fn AddSeparator(_obj: *u8 /* void* */) {
        unsafe {
            wxToolBar_AddSeparator(_obj)
        }
    }
    fn AddTool(_obj: *u8 /* void* */, id: c_int /* int */, bmp: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */) {
        unsafe {
            wxToolBar_AddTool(_obj, id, bmp, shelp, lhelp)
        }
    }
    fn AddToolEx(_obj: *u8 /* void* */, id: c_int /* int */, bmp1: *u8 /* void* */, bmp2: *u8 /* void* */, isToggle: bool /* bool */, arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */) {
        unsafe {
            wxToolBar_AddToolEx(_obj, id, bmp1, bmp2, isToggle, arg0, arg1, data, shelp, lhelp)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxToolBar_Delete(_obj)
        }
    }
    fn DeleteTool(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxToolBar_DeleteTool(_obj, id)
        }
    }
    fn DeleteToolByPos(_obj: *u8 /* void* */, pos: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxToolBar_DeleteToolByPos(_obj, pos)
        }
    }
    fn EnableTool(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxToolBar_EnableTool(_obj, id, enable)
        }
    }
    fn GetMargins(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetMargins(_obj)
        }
    }
    fn GetToolBitmapSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolBitmapSize(_obj)
        }
    }
    fn GetToolClientData(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolClientData(_obj, id)
        }
    }
    fn GetToolEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxToolBar_GetToolEnabled(_obj, id)
        }
    }
    fn GetToolLongHelp(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolLongHelp(_obj, id)
        }
    }
    fn GetToolPacking(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxToolBar_GetToolPacking(_obj)
        }
    }
    fn GetToolShortHelp(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolShortHelp(_obj, id)
        }
    }
    fn GetToolSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolSize(_obj)
        }
    }
    fn GetToolState(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxToolBar_GetToolState(_obj, id)
        }
    }
    fn InsertControl(_obj: *u8 /* void* */, pos: c_int /* int */, ctrl: *u8 /* void* */) {
        unsafe {
            wxToolBar_InsertControl(_obj, pos, ctrl)
        }
    }
    fn InsertSeparator(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxToolBar_InsertSeparator(_obj, pos)
        }
    }
    fn InsertTool(_obj: *u8 /* void* */, pos: c_int /* int */, id: c_int /* int */, bmp1: *u8 /* void* */, bmp2: *u8 /* void* */, isToggle: bool /* bool */, data: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */) {
        unsafe {
            wxToolBar_InsertTool(_obj, pos, id, bmp1, bmp2, isToggle, data, shelp, lhelp)
        }
    }
    fn Realize(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxToolBar_Realize(_obj)
        }
    }
    fn RemoveTool(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxToolBar_RemoveTool(_obj, id)
        }
    }
    fn SetMargins(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxToolBar_SetMargins(_obj, arg0, arg1)
        }
    }
    fn SetToolBitmapSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxToolBar_SetToolBitmapSize(_obj, arg0, arg1)
        }
    }
    fn SetToolClientData(_obj: *u8 /* void* */, id: c_int /* int */, data: *u8 /* void* */) {
        unsafe {
            wxToolBar_SetToolClientData(_obj, id, data)
        }
    }
    fn SetToolLongHelp(_obj: *u8 /* void* */, id: c_int /* int */, str: *u8 /* void* */) {
        unsafe {
            wxToolBar_SetToolLongHelp(_obj, id, str)
        }
    }
    fn SetToolPacking(_obj: *u8 /* void* */, packing: c_int /* int */) {
        unsafe {
            wxToolBar_SetToolPacking(_obj, packing)
        }
    }
    fn SetToolSeparation(_obj: *u8 /* void* */, separation: c_int /* int */) {
        unsafe {
            wxToolBar_SetToolSeparation(_obj, separation)
        }
    }
    fn SetToolShortHelp(_obj: *u8 /* void* */, id: c_int /* int */, str: *u8 /* void* */) {
        unsafe {
            wxToolBar_SetToolShortHelp(_obj, id, str)
        }
    }
    fn ToggleTool(_obj: *u8 /* void* */, id: c_int /* int */, toggle: bool /* bool */) {
        unsafe {
            wxToolBar_ToggleTool(_obj, id, toggle)
        }
    }
}
trait wxIdleEvent {
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxIdleEvent_CopyObject(_obj, object_dest)
        }
    }
    fn MoreRequested(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIdleEvent_MoreRequested(_obj)
        }
    }
    fn RequestMore(_obj: *u8 /* void* */, needMore: bool /* bool */) {
        unsafe {
            wxIdleEvent_RequestMore(_obj, needMore)
        }
    }
}
trait wxProgressDialog {
}
trait wxGridCellRenderer {
}
trait wxHtmlEasyPrinting {
}
trait wxFloatProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: float /* float */) -> *u8 /* void* */ {
        unsafe {
            wxFloatProperty_Create(label, name, value)
        }
    }
}
trait wxContextHelp {
    fn BeginContextHelp(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxContextHelp_BeginContextHelp(_obj, win)
        }
    }
    fn Create(win: *u8 /* void* */, beginHelp: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxContextHelp_Create(win, beginHelp)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxContextHelp_Delete(_obj)
        }
    }
    fn EndContextHelp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxContextHelp_EndContextHelp(_obj)
        }
    }
}
trait wxPoint {
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPoint_Create(arg0, arg1)
        }
    }
    // missing: wxPoint_Destroy
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPoint_GetX(_obj)
        }
    }
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPoint_GetY(_obj)
        }
    }
    fn SetX(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxPoint_SetX(_obj, w)
        }
    }
    fn SetY(_obj: *u8 /* void* */, h: c_int /* int */) {
        unsafe {
            wxPoint_SetY(_obj, h)
        }
    }
}
trait wxRegEx {
}
trait wxDropFilesEvent {
}
trait wxSocketOutputStream {
}
trait wxSpinCtrl {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */, _min: c_int /* int */, _max: c_int /* int */, _init: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSpinCtrl_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl, _min, _max, _init)
        }
    }
    fn GetMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinCtrl_GetMax(_obj)
        }
    }
    fn GetMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinCtrl_GetMin(_obj)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinCtrl_GetValue(_obj)
        }
    }
    fn SetRange(_obj: *u8 /* void* */, min_val: c_int /* int */, max_val: c_int /* int */) {
        unsafe {
            wxSpinCtrl_SetRange(_obj, min_val, max_val)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxSpinCtrl_SetValue(_obj, val)
        }
    }
}
trait wxScrollEvent {
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollEvent_GetOrientation(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollEvent_GetPosition(_obj)
        }
    }
}
trait wxPrintPreview {
    fn CreateFromData(printout: *u8 /* void* */, printoutForPrinting: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_CreateFromData(printout, printoutForPrinting, data)
        }
    }
    fn CreateFromDialogData(printout: *u8 /* void* */, printoutForPrinting: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_CreateFromDialogData(printout, printoutForPrinting, data)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_Delete(_obj)
        }
    }
    fn DetermineScaling(_obj: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_DetermineScaling(_obj)
        }
    }
    fn DrawBlankPage(_obj: *u8 /* void* */, canvas: *u8 /* void* */, dc: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_DrawBlankPage(_obj, canvas, dc)
        }
    }
    fn GetCanvas(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_GetCanvas(_obj)
        }
    }
    fn GetCurrentPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintPreview_GetCurrentPage(_obj)
        }
    }
    fn GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_GetFrame(_obj)
        }
    }
    fn GetMaxPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintPreview_GetMaxPage(_obj)
        }
    }
    fn GetMinPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintPreview_GetMinPage(_obj)
        }
    }
    fn GetPrintDialogData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_GetPrintDialogData(_obj, _ref)
        }
    }
    fn GetPrintout(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_GetPrintout(_obj)
        }
    }
    fn GetPrintoutForPrinting(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_GetPrintoutForPrinting(_obj)
        }
    }
    fn GetZoom(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintPreview_GetZoom(_obj)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_IsOk(_obj)
        }
    }
    fn PaintPage(_obj: *u8 /* void* */, canvas: *u8 /* void* */, dc: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_PaintPage(_obj, canvas, dc)
        }
    }
    fn Print(_obj: *u8 /* void* */, interactive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_Print(_obj, interactive)
        }
    }
    fn RenderPage(_obj: *u8 /* void* */, pageNum: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_RenderPage(_obj, pageNum)
        }
    }
    fn SetCanvas(_obj: *u8 /* void* */, canvas: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_SetCanvas(_obj, canvas)
        }
    }
    fn SetCurrentPage(_obj: *u8 /* void* */, pageNum: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_SetCurrentPage(_obj, pageNum)
        }
    }
    fn SetFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_SetFrame(_obj, frame)
        }
    }
    fn SetOk(_obj: *u8 /* void* */, ok: bool /* bool */) {
        unsafe {
            wxPrintPreview_SetOk(_obj, ok)
        }
    }
    fn SetPrintout(_obj: *u8 /* void* */, printout: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_SetPrintout(_obj, printout)
        }
    }
    fn SetZoom(_obj: *u8 /* void* */, percent: c_int /* int */) {
        unsafe {
            wxPrintPreview_SetZoom(_obj, percent)
        }
    }
}
trait wxPalette {
    fn Assign(_obj: *u8 /* void* */, palette: *u8 /* void* */) {
        unsafe {
            wxPalette_Assign(_obj, palette)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxPalette_CreateDefault()
        }
    }
    fn CreateRGB(n: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPalette_CreateRGB(n, red, green, blue)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPalette_Delete(_obj)
        }
    }
    fn GetPixel(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) -> c_int /* int */ {
        unsafe {
            wxPalette_GetPixel(_obj, arg0, arg1, arg2)
        }
    }
    fn GetRGB(_obj: *u8 /* void* */, pixel: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPalette_GetRGB(_obj, pixel, red, green, blue)
        }
    }
    fn IsEqual(_obj: *u8 /* void* */, palette: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPalette_IsEqual(_obj, palette)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPalette_IsOk(_obj)
        }
    }
}
trait wxGridRangeSelectEvent {
    fn GetTopLeftCoords(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGridRangeSelectEvent_GetTopLeftCoords(_obj, arg0, arg1)
        }
    }
    fn GetBottomRightCoords(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGridRangeSelectEvent_GetBottomRightCoords(_obj, arg0, arg1)
        }
    }
    fn GetTopRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridRangeSelectEvent_GetTopRow(_obj)
        }
    }
    fn GetBottomRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridRangeSelectEvent_GetBottomRow(_obj)
        }
    }
    fn GetLeftCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridRangeSelectEvent_GetLeftCol(_obj)
        }
    }
    fn GetRightCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridRangeSelectEvent_GetRightCol(_obj)
        }
    }
    fn Selecting(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridRangeSelectEvent_Selecting(_obj)
        }
    }
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridRangeSelectEvent_ControlDown(_obj)
        }
    }
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridRangeSelectEvent_MetaDown(_obj)
        }
    }
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridRangeSelectEvent_ShiftDown(_obj)
        }
    }
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridRangeSelectEvent_AltDown(_obj)
        }
    }
}
trait cbSimpleUpdatesMgr {
}
trait wxGenericDirCtrl {
}
trait wxMetafile {
    fn Create(_file: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMetafile_Create(_file)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMetafile_Delete(_obj)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMetafile_IsOk(_obj)
        }
    }
    fn Play(_obj: *u8 /* void* */, _dc: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMetafile_Play(_obj, _dc)
        }
    }
    fn SetClipboard(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMetafile_SetClipboard(_obj, arg0, arg1)
        }
    }
}
trait wxListItem {
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxListItem_Clear(_obj)
        }
    }
    fn ClearAttributes(_obj: *u8 /* void* */) {
        unsafe {
            wxListItem_ClearAttributes(_obj)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxListItem_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxListItem_Delete(_obj)
        }
    }
    fn GetAlign(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetAlign(_obj)
        }
    }
    fn GetAttributes(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListItem_GetAttributes(_obj)
        }
    }
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListItem_GetBackgroundColour(_obj, _ref)
        }
    }
    fn GetColumn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetColumn(_obj)
        }
    }
    fn GetData(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetData(_obj)
        }
    }
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListItem_GetFont(_obj, _ref)
        }
    }
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetId(_obj)
        }
    }
    fn GetImage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetImage(_obj)
        }
    }
    fn GetMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetMask(_obj)
        }
    }
    fn GetState(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetState(_obj)
        }
    }
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListItem_GetText(_obj)
        }
    }
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListItem_GetTextColour(_obj, _ref)
        }
    }
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetWidth(_obj)
        }
    }
    fn HasAttributes(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListItem_HasAttributes(_obj)
        }
    }
    fn SetAlign(_obj: *u8 /* void* */, align: c_int /* int */) {
        unsafe {
            wxListItem_SetAlign(_obj, align)
        }
    }
    fn SetBackgroundColour(_obj: *u8 /* void* */, colBack: *u8 /* void* */) {
        unsafe {
            wxListItem_SetBackgroundColour(_obj, colBack)
        }
    }
    fn SetColumn(_obj: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxListItem_SetColumn(_obj, col)
        }
    }
    fn SetData(_obj: *u8 /* void* */, data: c_int /* int */) {
        unsafe {
            wxListItem_SetData(_obj, data)
        }
    }
    fn SetDataPointer(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxListItem_SetDataPointer(_obj, data)
        }
    }
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxListItem_SetFont(_obj, font)
        }
    }
    fn SetId(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxListItem_SetId(_obj, id)
        }
    }
    fn SetImage(_obj: *u8 /* void* */, image: c_int /* int */) {
        unsafe {
            wxListItem_SetImage(_obj, image)
        }
    }
    fn SetMask(_obj: *u8 /* void* */, mask: c_int /* int */) {
        unsafe {
            wxListItem_SetMask(_obj, mask)
        }
    }
    fn SetState(_obj: *u8 /* void* */, state: c_int /* int */) {
        unsafe {
            wxListItem_SetState(_obj, state)
        }
    }
    fn SetStateMask(_obj: *u8 /* void* */, stateMask: c_int /* int */) {
        unsafe {
            wxListItem_SetStateMask(_obj, stateMask)
        }
    }
    fn SetText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxListItem_SetText(_obj, text)
        }
    }
    fn SetTextColour(_obj: *u8 /* void* */, colText: *u8 /* void* */) {
        unsafe {
            wxListItem_SetTextColour(_obj, colText)
        }
    }
    fn SetWidth(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxListItem_SetWidth(_obj, width)
        }
    }
}
trait wxScopedArray {
}
trait wxDataObject {
}
trait wxCommandProcessor {
    // missing: wxCommandProcessor_CanRedo
    // missing: wxCommandProcessor_CanUndo
    // missing: wxCommandProcessor_ClearCommands
    // missing: wxCommandProcessor_Delete
    // missing: wxCommandProcessor_GetCommands
    // missing: wxCommandProcessor_GetEditMenu
    // missing: wxCommandProcessor_GetMaxCommands
    // missing: wxCommandProcessor_Initialize
    // missing: wxCommandProcessor_Redo
    // missing: wxCommandProcessor_SetEditMenu
    // missing: wxCommandProcessor_SetMenuStrings
    // missing: wxCommandProcessor_Submit
    // missing: wxCommandProcessor_Undo
    // missing: wxCommandProcessor_wxCommandProcessor
}
trait wxInitDialogEvent {
}
trait wxTreeItemId {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxTreeItemId_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeItemId_Delete(_obj)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeItemId_IsOk(_obj)
        }
    }
}
trait wxSashLayoutWindow {
    fn Create(_par: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSashLayoutWindow_Create(_par, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashLayoutWindow_GetAlignment(_obj)
        }
    }
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashLayoutWindow_GetOrientation(_obj)
        }
    }
    fn SetAlignment(_obj: *u8 /* void* */, align: c_int /* int */) {
        unsafe {
            wxSashLayoutWindow_SetAlignment(_obj, align)
        }
    }
    fn SetDefaultSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSashLayoutWindow_SetDefaultSize(_obj, arg0, arg1)
        }
    }
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */) {
        unsafe {
            wxSashLayoutWindow_SetOrientation(_obj, orient)
        }
    }
}
trait cbHintAnimationPlugin {
    // missing: cbHintAnimationPlugin_Create
    // missing: cbHintAnimationPlugin_CreateDefault
    // missing: cbHintAnimationPlugin_Delete
}
trait wxPrinterDC {
    fn Create(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinterDC_Create(data)
        }
    }
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxPrinterDC_Delete(self_)
        }
    }
    fn GetPaperRect(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinterDC_GetPaperRect(self_)
        }
    }
}
trait wxWindowDestroyEvent {
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindowDestroyEvent_GetWindow(_obj)
        }
    }
}
trait wxIcon {
    fn Assign(_obj: *u8 /* void* */, other: *u8 /* void* */) {
        unsafe {
            wxIcon_Assign(_obj, other)
        }
    }
    fn CopyFromBitmap(_obj: *u8 /* void* */, bmp: *u8 /* void* */) {
        unsafe {
            wxIcon_CopyFromBitmap(_obj, bmp)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxIcon_CreateDefault()
        }
    }
    fn CreateLoad(name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxIcon_CreateLoad(name, type_, arg0, arg1)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxIcon_Delete(_obj)
        }
    }
    fn FromRaw(data: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxIcon_FromRaw(data, arg0, arg1)
        }
    }
    fn FromXPM(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxIcon_FromXPM(data)
        }
    }
    fn GetDepth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIcon_GetDepth(_obj)
        }
    }
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIcon_GetHeight(_obj)
        }
    }
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIcon_GetWidth(_obj)
        }
    }
    fn IsEqual(_obj: *u8 /* void* */, other: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIcon_IsEqual(_obj, other)
        }
    }
    fn Load(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxIcon_Load(_obj, name, type_, arg0, arg1)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIcon_IsOk(_obj)
        }
    }
    fn SetDepth(_obj: *u8 /* void* */, depth: c_int /* int */) {
        unsafe {
            wxIcon_SetDepth(_obj, depth)
        }
    }
    fn SetHeight(_obj: *u8 /* void* */, height: c_int /* int */) {
        unsafe {
            wxIcon_SetHeight(_obj, height)
        }
    }
    fn SetWidth(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxIcon_SetWidth(_obj, width)
        }
    }
}
trait cbPluginBase {
    // missing: cbPluginBase_Delete
    // missing: cbPluginBase_GetPaneMask
    // missing: cbPluginBase_IsReady
    // missing: cbPluginBase_Plugin
    // missing: cbPluginBase_ProcessEvent
}
trait wxBufferedPaintDC {
    fn Create(window: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBufferedPaintDC_Create(window, style)
        }
    }
    fn CreateWithBitmap(window: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBufferedPaintDC_CreateWithBitmap(window, bitmap, style)
        }
    }
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxBufferedPaintDC_Delete(self_)
        }
    }
}
trait ELJPlotCurve {
    // missing: ELJPlotCurve_Create
    // missing: ELJPlotCurve_Delete
    // missing: ELJPlotCurve_GetEndY
    // missing: ELJPlotCurve_GetOffsetY
    // missing: ELJPlotCurve_GetStartY
    // missing: ELJPlotCurve_SetEndY
    // missing: ELJPlotCurve_SetOffsetY
    // missing: ELJPlotCurve_SetPenNormal
    // missing: ELJPlotCurve_SetPenSelected
    // missing: ELJPlotCurve_SetStartY
}
trait wxToolTip {
}
trait wxcHtmlWindow {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */, _txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl, _txt)
        }
    }
    fn wxHtmlWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */, _txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl, _txt)
        }
    }
    fn wxHtmlWindow_AppendToPage(_obj: *u8 /* void* */, source: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_AppendToPage(_obj, source)
        }
    }
    fn wxHtmlWindow_GetInternalRepresentation(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetInternalRepresentation(_obj)
        }
    }
    fn wxHtmlWindow_GetOpenedAnchor(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetOpenedAnchor(_obj)
        }
    }
    fn wxHtmlWindow_GetOpenedPage(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetOpenedPage(_obj)
        }
    }
    fn wxHtmlWindow_GetOpenedPageTitle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetOpenedPageTitle(_obj)
        }
    }
    fn wxHtmlWindow_GetRelatedFrame(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetRelatedFrame(_obj)
        }
    }
    fn wxHtmlWindow_HistoryBack(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_HistoryBack(_obj)
        }
    }
    fn wxHtmlWindow_HistoryCanBack(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_HistoryCanBack(_obj)
        }
    }
    fn wxHtmlWindow_HistoryCanForward(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_HistoryCanForward(_obj)
        }
    }
    fn wxHtmlWindow_HistoryClear(_obj: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_HistoryClear(_obj)
        }
    }
    fn wxHtmlWindow_HistoryForward(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_HistoryForward(_obj)
        }
    }
    fn wxHtmlWindow_LoadPage(_obj: *u8 /* void* */, location: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_LoadPage(_obj, location)
        }
    }
    fn wxHtmlWindow_ReadCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_ReadCustomization(_obj, cfg, path)
        }
    }
    fn wxHtmlWindow_SetBorders(_obj: *u8 /* void* */, b: c_int /* int */) {
        unsafe {
            wxHtmlWindow_SetBorders(_obj, b)
        }
    }
    fn wxHtmlWindow_SetFonts(_obj: *u8 /* void* */, normal_face: *u8 /* void* */, fixed_face: *u8 /* void* */, sizes: *c_int /* int* */) {
        unsafe {
            wxHtmlWindow_SetFonts(_obj, normal_face, fixed_face, sizes)
        }
    }
    fn wxHtmlWindow_SetPage(_obj: *u8 /* void* */, source: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_SetPage(_obj, source)
        }
    }
    fn wxHtmlWindow_SetRelatedFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, format: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_SetRelatedFrame(_obj, frame, format)
        }
    }
    fn wxHtmlWindow_SetRelatedStatusBar(_obj: *u8 /* void* */, bar: c_int /* int */) {
        unsafe {
            wxHtmlWindow_SetRelatedStatusBar(_obj, bar)
        }
    }
    fn wxHtmlWindow_WriteCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_WriteCustomization(_obj, cfg, path)
        }
    }
}
trait wxValidator {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxValidator_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxValidator_Delete(_obj)
        }
    }
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxValidator_GetWindow(_obj)
        }
    }
    fn SetBellOnError(doIt: bool /* bool */) {
        unsafe {
            wxValidator_SetBellOnError(doIt)
        }
    }
    fn SetWindow(_obj: *u8 /* void* */, win: *u8 /* void* */) {
        unsafe {
            wxValidator_SetWindow(_obj, win)
        }
    }
    fn TransferFromWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxValidator_TransferFromWindow(_obj)
        }
    }
    fn TransferToWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxValidator_TransferToWindow(_obj)
        }
    }
    fn Validate(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxValidator_Validate(_obj, parent)
        }
    }
}
trait wxNodeBase {
}
trait wxDbTableInfo {
}
trait wxDropTarget {
    fn GetData(_obj: *u8 /* void* */) {
        unsafe {
            wxDropTarget_GetData(_obj)
        }
    }
    fn SetDataObject(_obj: *u8 /* void* */, _dat: *u8 /* void* */) {
        unsafe {
            wxDropTarget_SetDataObject(_obj, _dat)
        }
    }
}
trait cbDockPane {
    // missing: cbDockPane_BarPresent
    // missing: cbDockPane_Create
    // missing: cbDockPane_CreateDefault
    // missing: cbDockPane_Delete
    // missing: cbDockPane_GetAlignment
    // missing: cbDockPane_GetBarInfoByWindow
    // missing: cbDockPane_GetBarResizeRange
    // missing: cbDockPane_GetDockingState
    // missing: cbDockPane_GetFirstRow
    // missing: cbDockPane_GetPaneHeight
    // missing: cbDockPane_GetRealRect
    // missing: cbDockPane_GetRowList
    // missing: cbDockPane_GetRowResizeRange
    // missing: cbDockPane_HitTestPaneItems
    // missing: cbDockPane_InsertBarByCoord
    // missing: cbDockPane_InsertBarByInfo
    // missing: cbDockPane_InsertBarToRow
    // missing: cbDockPane_InsertRow
    // missing: cbDockPane_IsHorizontal
    // missing: cbDockPane_MatchesMask
    // missing: cbDockPane_RemoveBar
    // missing: cbDockPane_RemoveRow
    // missing: cbDockPane_SetBoundsInParent
    // missing: cbDockPane_SetMargins
    // missing: cbDockPane_SetPaneWidth
}
trait wxToggleButton {
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToggleButton_Create(parent, id, label, arg0, arg1, arg2, arg3, style)
        }
    }
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxToggleButton_Enable(_obj, enable)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxToggleButton_GetValue(_obj)
        }
    }
    fn SetLabel(_obj: *u8 /* void* */, label: *u8 /* void* */) {
        unsafe {
            wxToggleButton_SetLabel(_obj, label)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, state: bool /* bool */) {
        unsafe {
            wxToggleButton_SetValue(_obj, state)
        }
    }
}
trait wxControl {
    fn Command(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxControl_Command(_obj, event)
        }
    }
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxControl_GetLabel(_obj)
        }
    }
    fn SetLabel(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxControl_SetLabel(_obj, text)
        }
    }
}
trait wxContextHelpButton {
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxContextHelpButton_Create(parent, id, arg0, arg1, arg2, arg3, style)
        }
    }
}
trait wxHtmlWidgetCell {
}
trait wxZipInputStream {
}
trait wxHtmlHelpData {
}
trait wxComboBox {
    fn Append(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxComboBox_Append(_obj, item)
        }
    }
    fn AppendData(_obj: *u8 /* void* */, item: *u8 /* void* */, d: *u8 /* void* */) {
        unsafe {
            wxComboBox_AppendData(_obj, item, d)
        }
    }
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_Clear(_obj)
        }
    }
    fn Copy(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_Copy(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, arg4, arg5, _stl)
        }
    }
    fn Cut(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_Cut(_obj)
        }
    }
    fn Delete(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxComboBox_Delete(_obj, n)
        }
    }
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_FindString(_obj, s)
        }
    }
    fn GetClientData(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_GetClientData(_obj, n)
        }
    }
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_GetCount(_obj)
        }
    }
    fn GetInsertionPoint(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_GetInsertionPoint(_obj)
        }
    }
    fn GetLastPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_GetLastPosition(_obj)
        }
    }
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_GetSelection(_obj)
        }
    }
    fn GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_GetString(_obj, n)
        }
    }
    fn GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_GetStringSelection(_obj)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_GetValue(_obj)
        }
    }
    fn Paste(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_Paste(_obj)
        }
    }
    fn Remove(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */) {
        unsafe {
            wxComboBox_Remove(_obj, from, to)
        }
    }
    fn Replace(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */, value: *u8 /* void* */) {
        unsafe {
            wxComboBox_Replace(_obj, from, to, value)
        }
    }
    fn SetClientData(_obj: *u8 /* void* */, n: c_int /* int */, clientData: *u8 /* void* */) {
        unsafe {
            wxComboBox_SetClientData(_obj, n, clientData)
        }
    }
    fn SetEditable(_obj: *u8 /* void* */, editable: bool /* bool */) {
        unsafe {
            wxComboBox_SetEditable(_obj, editable)
        }
    }
    fn SetInsertionPoint(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxComboBox_SetInsertionPoint(_obj, pos)
        }
    }
    fn SetInsertionPointEnd(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_SetInsertionPointEnd(_obj)
        }
    }
    fn SetSelection(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxComboBox_SetSelection(_obj, n)
        }
    }
    fn SetTextSelection(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */) {
        unsafe {
            wxComboBox_SetTextSelection(_obj, from, to)
        }
    }
}
trait wxHtmlCell {
}
trait wxFont {
    fn Create(pointSize: c_int /* int */, family: c_int /* int */, style: c_int /* int */, weight: c_int /* int */, underlined: bool /* bool */, face: *u8 /* void* */, enc: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFont_Create(pointSize, family, style, weight, underlined, face, enc)
        }
    }
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFont_CreateFromStock(id)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxFont_CreateDefault()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFont_Delete(_obj)
        }
    }
    fn GetDefaultEncoding(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetDefaultEncoding(_obj)
        }
    }
    fn GetEncoding(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetEncoding(_obj)
        }
    }
    fn GetFaceName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFont_GetFaceName(_obj)
        }
    }
    fn GetFamily(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetFamily(_obj)
        }
    }
    fn GetFamilyString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFont_GetFamilyString(_obj)
        }
    }
    fn GetPointSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetPointSize(_obj)
        }
    }
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetStyle(_obj)
        }
    }
    fn GetStyleString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFont_GetStyleString(_obj)
        }
    }
    fn GetUnderlined(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetUnderlined(_obj)
        }
    }
    fn GetWeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetWeight(_obj)
        }
    }
    fn GetWeightString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFont_GetWeightString(_obj)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFont_IsOk(_obj)
        }
    }
    fn SetDefaultEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */) {
        unsafe {
            wxFont_SetDefaultEncoding(_obj, encoding)
        }
    }
    fn SetEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */) {
        unsafe {
            wxFont_SetEncoding(_obj, encoding)
        }
    }
    fn SetFaceName(_obj: *u8 /* void* */, faceName: *u8 /* void* */) {
        unsafe {
            wxFont_SetFaceName(_obj, faceName)
        }
    }
    fn SetFamily(_obj: *u8 /* void* */, family: c_int /* int */) {
        unsafe {
            wxFont_SetFamily(_obj, family)
        }
    }
    fn SetPointSize(_obj: *u8 /* void* */, pointSize: c_int /* int */) {
        unsafe {
            wxFont_SetPointSize(_obj, pointSize)
        }
    }
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxFont_SetStyle(_obj, style)
        }
    }
    fn SetUnderlined(_obj: *u8 /* void* */, underlined: c_int /* int */) {
        unsafe {
            wxFont_SetUnderlined(_obj, underlined)
        }
    }
    fn SetWeight(_obj: *u8 /* void* */, weight: c_int /* int */) {
        unsafe {
            wxFont_SetWeight(_obj, weight)
        }
    }
}
trait cbAntiflickerPlugin {
    // missing: cbAntiflickerPlugin_Create
    // missing: cbAntiflickerPlugin_CreateDefault
    // missing: cbAntiflickerPlugin_Delete
}
trait wxMemoryOutputStream {
}
trait wxNavigationKeyEvent {
    fn GetCurrentFocus(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxNavigationKeyEvent_GetCurrentFocus(_obj)
        }
    }
    fn GetDirection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNavigationKeyEvent_GetDirection(_obj)
        }
    }
    fn IsWindowChange(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNavigationKeyEvent_IsWindowChange(_obj)
        }
    }
    fn SetCurrentFocus(_obj: *u8 /* void* */, win: *u8 /* void* */) {
        unsafe {
            wxNavigationKeyEvent_SetCurrentFocus(_obj, win)
        }
    }
    fn SetDirection(_obj: *u8 /* void* */, bForward: bool /* bool */) {
        unsafe {
            wxNavigationKeyEvent_SetDirection(_obj, bForward)
        }
    }
    fn SetWindowChange(_obj: *u8 /* void* */, bIs: bool /* bool */) {
        unsafe {
            wxNavigationKeyEvent_SetWindowChange(_obj, bIs)
        }
    }
    fn ShouldPropagate(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxNavigationKeyEvent_ShouldPropagate(_obj)
        }
    }
}
trait wxDateProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateProperty_Create(label, name, value)
        }
    }
}
trait wxZlibInputStream {
}
trait cbPaneDrawPlugin {
    // missing: cbPaneDrawPlugin_Create
    // missing: cbPaneDrawPlugin_CreateDefault
    // missing: cbPaneDrawPlugin_Delete
}
trait wxProcessEvent {
    fn GetExitCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxProcessEvent_GetExitCode(_obj)
        }
    }
    fn GetPid(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxProcessEvent_GetPid(_obj)
        }
    }
}
trait wxPrinter {
    fn Create(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinter_Create(data)
        }
    }
    fn CreateAbortWindow(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinter_CreateAbortWindow(_obj, parent, printout)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPrinter_Delete(_obj)
        }
    }
    fn GetAbort(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrinter_GetAbort(_obj)
        }
    }
    fn GetLastError(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrinter_GetLastError(_obj)
        }
    }
    fn GetPrintDialogData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPrinter_GetPrintDialogData(_obj, _ref)
        }
    }
    fn Print(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */, prompt: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxPrinter_Print(_obj, parent, printout, prompt)
        }
    }
    fn PrintDialog(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinter_PrintDialog(_obj, parent)
        }
    }
    fn ReportError(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */, message: *u8 /* void* */) {
        unsafe {
            wxPrinter_ReportError(_obj, parent, printout, message)
        }
    }
    fn Setup(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrinter_Setup(_obj, parent)
        }
    }
}
trait wxView {
}
trait wxDbColInf {
}
trait wxColourDatabase {
}
trait wxGridCellNumberEditor {
    fn Ctor(min: c_int /* int */, max: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellNumberEditor_Ctor(min, max)
        }
    }
}
trait wxFlexGridSizer {
    fn AddGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */) {
        unsafe {
            wxFlexGridSizer_AddGrowableCol(_obj, idx)
        }
    }
    fn AddGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */) {
        unsafe {
            wxFlexGridSizer_AddGrowableRow(_obj, idx)
        }
    }
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFlexGridSizer_CalcMin(_obj)
        }
    }
    fn Create(rows: c_int /* int */, cols: c_int /* int */, vgap: c_int /* int */, hgap: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFlexGridSizer_Create(rows, cols, vgap, hgap)
        }
    }
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxFlexGridSizer_RecalcSizes(_obj)
        }
    }
    fn RemoveGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */) {
        unsafe {
            wxFlexGridSizer_RemoveGrowableCol(_obj, idx)
        }
    }
    fn RemoveGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */) {
        unsafe {
            wxFlexGridSizer_RemoveGrowableRow(_obj, idx)
        }
    }
}
trait wxMouseCaptureChangedEvent {
}
trait wxInputSinkEvent {
    fn LastError(obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputSinkEvent_LastError(obj)
        }
    }
    fn LastRead(obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputSinkEvent_LastRead(obj)
        }
    }
    fn LastInput(obj: *u8 /* void* */) -> *char /* char* */ {
        unsafe {
            wxInputSinkEvent_LastInput(obj)
        }
    }
}
trait wxPrintout {
}
trait wxXmlResource {
    fn AddHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxXmlResource_AddHandler(_obj, handler)
        }
    }
    fn AddSubclassFactory(_obj: *u8 /* void* */, factory: *u8 /* void* */) {
        unsafe {
            wxXmlResource_AddSubclassFactory(_obj, factory)
        }
    }
    fn AttachUnknownControl(_obj: *u8 /* void* */, control: *u8 /* void* */, parent: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxXmlResource_AttachUnknownControl(_obj, control, parent)
        }
    }
    fn ClearHandlers(_obj: *u8 /* void* */) {
        unsafe {
            wxXmlResource_ClearHandlers(_obj)
        }
    }
    fn CompareVersion(_obj: *u8 /* void* */, major: c_int /* int */, minor: c_int /* int */, release: c_int /* int */, revision: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxXmlResource_CompareVersion(_obj, major, minor, release, revision)
        }
    }
    fn Create(flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_Create(flags)
        }
    }
    fn CreateFromFile(filemask: *u8 /* void* */, flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_CreateFromFile(filemask, flags)
        }
    }
    // missing: wxXmlResource_Delete
    fn Get() -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_Get()
        }
    }
    fn GetDomain(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetDomain(_obj)
        }
    }
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxXmlResource_GetFlags(_obj)
        }
    }
    fn GetVersion(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxXmlResource_GetVersion(_obj)
        }
    }
    fn GetXRCID(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxXmlResource_GetXRCID(_obj, str_id)
        }
    }
    fn InitAllHandlers(_obj: *u8 /* void* */) {
        unsafe {
            wxXmlResource_InitAllHandlers(_obj)
        }
    }
    fn InsertHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxXmlResource_InsertHandler(_obj, handler)
        }
    }
    fn Load(_obj: *u8 /* void* */, filemask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxXmlResource_Load(_obj, filemask)
        }
    }
    fn LoadBitmap(_obj: *u8 /* void* */, name: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxXmlResource_LoadBitmap(_obj, name, _ref)
        }
    }
    fn LoadDialog(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadDialog(_obj, parent, name)
        }
    }
    fn LoadFrame(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadFrame(_obj, parent, name)
        }
    }
    fn LoadIcon(_obj: *u8 /* void* */, name: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxXmlResource_LoadIcon(_obj, name, _ref)
        }
    }
    fn LoadMenu(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadMenu(_obj, name)
        }
    }
    fn LoadMenuBar(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadMenuBar(_obj, parent, name)
        }
    }
    fn LoadPanel(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadPanel(_obj, parent, name)
        }
    }
    fn LoadToolBar(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadToolBar(_obj, parent, name)
        }
    }
    fn GetSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSizer(_obj, str_id)
        }
    }
    fn GetBoxSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetBoxSizer(_obj, str_id)
        }
    }
    fn GetStaticBoxSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticBoxSizer(_obj, str_id)
        }
    }
    fn GetGridSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetGridSizer(_obj, str_id)
        }
    }
    fn GetFlexGridSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetFlexGridSizer(_obj, str_id)
        }
    }
    fn GetBitmapButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetBitmapButton(_obj, str_id)
        }
    }
    fn GetButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetButton(_obj, str_id)
        }
    }
    fn GetCalendarCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetCalendarCtrl(_obj, str_id)
        }
    }
    fn GetCheckBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetCheckBox(_obj, str_id)
        }
    }
    fn GetCheckListBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetCheckListBox(_obj, str_id)
        }
    }
    fn GetChoice(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetChoice(_obj, str_id)
        }
    }
    fn GetComboBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetComboBox(_obj, str_id)
        }
    }
    fn GetGauge(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetGauge(_obj, str_id)
        }
    }
    fn GetGrid(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetGrid(_obj, str_id)
        }
    }
    fn GetHtmlWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetHtmlWindow(_obj, str_id)
        }
    }
    fn GetListBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetListBox(_obj, str_id)
        }
    }
    fn GetListCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetListCtrl(_obj, str_id)
        }
    }
    fn GetMDIChildFrame(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMDIChildFrame(_obj, str_id)
        }
    }
    fn GetMDIParentFrame(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMDIParentFrame(_obj, str_id)
        }
    }
    fn GetMenu(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMenu(_obj, str_id)
        }
    }
    fn GetMenuBar(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMenuBar(_obj, str_id)
        }
    }
    fn GetMenuItem(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMenuItem(_obj, str_id)
        }
    }
    fn GetNotebook(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetNotebook(_obj, str_id)
        }
    }
    fn GetPanel(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetPanel(_obj, str_id)
        }
    }
    fn GetRadioButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetRadioButton(_obj, str_id)
        }
    }
    fn GetRadioBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetRadioBox(_obj, str_id)
        }
    }
    fn GetScrollBar(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetScrollBar(_obj, str_id)
        }
    }
    fn GetScrolledWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetScrolledWindow(_obj, str_id)
        }
    }
    fn GetSlider(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSlider(_obj, str_id)
        }
    }
    fn GetSpinButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSpinButton(_obj, str_id)
        }
    }
    fn GetSpinCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSpinCtrl(_obj, str_id)
        }
    }
    fn GetSplitterWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSplitterWindow(_obj, str_id)
        }
    }
    fn GetStaticBitmap(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticBitmap(_obj, str_id)
        }
    }
    fn GetStaticBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticBox(_obj, str_id)
        }
    }
    fn GetStaticLine(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticLine(_obj, str_id)
        }
    }
    fn GetStaticText(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticText(_obj, str_id)
        }
    }
    fn GetTextCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetTextCtrl(_obj, str_id)
        }
    }
    fn GetTreeCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetTreeCtrl(_obj, str_id)
        }
    }
    fn Unload(_obj: *u8 /* void* */, filemask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxXmlResource_Unload(_obj, filemask)
        }
    }
    fn Set(_obj: *u8 /* void* */, res: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_Set(_obj, res)
        }
    }
    fn SetDomain(_obj: *u8 /* void* */, domain: *u8 /* void* */) {
        unsafe {
            wxXmlResource_SetDomain(_obj, domain)
        }
    }
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxXmlResource_SetFlags(_obj, flags)
        }
    }
}
trait wxClientBase {
}
trait wxPrintDialog {
    fn Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintDialog_Create(parent, data)
        }
    }
    fn GetPrintDC(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintDialog_GetPrintDC(_obj)
        }
    }
    fn GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPrintDialog_GetPrintData(_obj, _ref)
        }
    }
    fn GetPrintDialogData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintDialog_GetPrintDialogData(_obj)
        }
    }
}
trait wxDDEClient {
}
trait wxPathList {
}
trait wxStaticBitmap {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, bitmap: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBitmap_Create(_prt, _id, bitmap, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_Delete(_obj)
        }
    }
    fn GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_GetBitmap(_obj, _ref)
        }
    }
    fn GetIcon(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_GetIcon(_obj, _ref)
        }
    }
    fn SetBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_SetBitmap(_obj, bitmap)
        }
    }
    fn SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_SetIcon(_obj, icon)
        }
    }
}
trait wxSVGFileDC {
    fn Create(fileName: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSVGFileDC_Create(fileName)
        }
    }
    fn CreateWithSize(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSVGFileDC_CreateWithSize(fileName, arg0, arg1)
        }
    }
    fn CreateWithSizeAndResolution(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, a_dpi: float /* float */) -> *u8 /* void* */ {
        unsafe {
            wxSVGFileDC_CreateWithSizeAndResolution(fileName, arg0, arg1, a_dpi)
        }
    }
    fn Delete(obj: *u8 /* void* */) {
        unsafe {
            wxSVGFileDC_Delete(obj)
        }
    }
}
trait wxStringClientData {
}
trait wxQueryLayoutInfoEvent {
    fn Create(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxQueryLayoutInfoEvent_Create(id)
        }
    }
    fn GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetAlignment(_obj)
        }
    }
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetFlags(_obj)
        }
    }
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetOrientation(_obj)
        }
    }
    fn GetRequestedLength(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetRequestedLength(_obj)
        }
    }
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetSize(_obj)
        }
    }
    fn SetAlignment(_obj: *u8 /* void* */, align: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetAlignment(_obj, align)
        }
    }
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetFlags(_obj, flags)
        }
    }
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetOrientation(_obj, orient)
        }
    }
    fn SetRequestedLength(_obj: *u8 /* void* */, length: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetRequestedLength(_obj, length)
        }
    }
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetSize(_obj, arg0, arg1)
        }
    }
}
trait wxGridCellAutoWrapStringRenderer {
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellAutoWrapStringRenderer_Ctor()
        }
    }
}
trait wxFileDialog {
    fn Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _dir: *u8 /* void* */, _fle: *u8 /* void* */, _wcd: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_Create(_prt, _msg, _dir, _fle, _wcd, arg0, arg1, _stl)
        }
    }
    fn GetDirectory(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetDirectory(_obj)
        }
    }
    fn GetFilename(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetFilename(_obj)
        }
    }
    fn GetFilenames(_obj: *u8 /* void* */, paths: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxFileDialog_GetFilenames(_obj, paths)
        }
    }
    fn GetFilterIndex(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileDialog_GetFilterIndex(_obj)
        }
    }
    fn GetMessage(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetMessage(_obj)
        }
    }
    fn GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetPath(_obj)
        }
    }
    fn GetPaths(_obj: *u8 /* void* */, paths: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxFileDialog_GetPaths(_obj, paths)
        }
    }
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileDialog_GetStyle(_obj)
        }
    }
    fn GetWildcard(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetWildcard(_obj)
        }
    }
    fn SetDirectory(_obj: *u8 /* void* */, dir: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetDirectory(_obj, dir)
        }
    }
    fn SetFilename(_obj: *u8 /* void* */, name: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetFilename(_obj, name)
        }
    }
    fn SetFilterIndex(_obj: *u8 /* void* */, filterIndex: c_int /* int */) {
        unsafe {
            wxFileDialog_SetFilterIndex(_obj, filterIndex)
        }
    }
    fn SetMessage(_obj: *u8 /* void* */, message: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetMessage(_obj, message)
        }
    }
    fn SetPath(_obj: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetPath(_obj, path)
        }
    }
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxFileDialog_SetStyle(_obj, style)
        }
    }
    fn SetWildcard(_obj: *u8 /* void* */, wildCard: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetWildcard(_obj, wildCard)
        }
    }
}
trait wxDllLoader {
}
trait ELJLog {
    fn AddTraceMask(_obj: *u8 /* void* */, str: *wchar_t /* wchar_t* */) {
        unsafe {
            ELJLog_AddTraceMask(_obj, str)
        }
    }
    fn Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJLog_Create(_obj, _fnc)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_Delete(_obj)
        }
    }
    fn DontCreateOnDemand(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_DontCreateOnDemand(_obj)
        }
    }
    fn EnableLogging(_obj: *u8 /* void* */, doIt: bool /* bool */) -> c_int /* int */ {
        unsafe {
            ELJLog_EnableLogging(_obj, doIt)
        }
    }
    fn Flush(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_Flush(_obj)
        }
    }
    fn FlushActive(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_FlushActive(_obj)
        }
    }
    fn GetActiveTarget() -> *u8 /* void* */ {
        unsafe {
            ELJLog_GetActiveTarget()
        }
    }
    fn GetTimestamp(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJLog_GetTimestamp(_obj)
        }
    }
    fn GetTraceMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJLog_GetTraceMask(_obj)
        }
    }
    fn GetVerbose(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJLog_GetVerbose(_obj)
        }
    }
    fn HasPendingMessages(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJLog_HasPendingMessages(_obj)
        }
    }
    fn IsAllowedTraceMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJLog_IsAllowedTraceMask(_obj, mask)
        }
    }
    fn IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJLog_IsEnabled(_obj)
        }
    }
    fn OnLog(_obj: *u8 /* void* */, level: c_int /* int */, szString: *u8 /* void* */, t: c_int /* int */) {
        unsafe {
            ELJLog_OnLog(_obj, level, szString, t)
        }
    }
    fn RemoveTraceMask(_obj: *u8 /* void* */, str: *wchar_t /* wchar_t* */) {
        unsafe {
            ELJLog_RemoveTraceMask(_obj, str)
        }
    }
    fn Resume(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_Resume(_obj)
        }
    }
    fn SetActiveTarget(pLogger: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJLog_SetActiveTarget(pLogger)
        }
    }
    fn SetTimestamp(_obj: *u8 /* void* */, ts: *u8 /* void* */) {
        unsafe {
            ELJLog_SetTimestamp(_obj, ts)
        }
    }
    fn SetTraceMask(_obj: *u8 /* void* */, ulMask: c_int /* int */) {
        unsafe {
            ELJLog_SetTraceMask(_obj, ulMask)
        }
    }
    fn SetVerbose(_obj: *u8 /* void* */, bVerbose: c_int /* int */) {
        unsafe {
            ELJLog_SetVerbose(_obj, bVerbose)
        }
    }
    fn Suspend(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_Suspend(_obj)
        }
    }
}
trait wxConnectionBase {
}
trait wxMenuItem {
    fn Check(_obj: *u8 /* void* */, check: bool /* bool */) {
        unsafe {
            wxMenuItem_Check(_obj, check)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMenuItem_Delete(_obj)
        }
    }
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxMenuItem_Enable(_obj, enable)
        }
    }
    fn GetHelp(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetHelp(_obj)
        }
    }
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuItem_GetId(_obj)
        }
    }
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetLabel(_obj)
        }
    }
    fn GetLabelFromText(text: *wchar_t /* wchar_t* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetLabelFromText(text)
        }
    }
    fn GetMenu(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetMenu(_obj)
        }
    }
    fn GetSubMenu(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetSubMenu(_obj)
        }
    }
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetText(_obj)
        }
    }
    fn IsCheckable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsCheckable(_obj)
        }
    }
    fn IsChecked(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsChecked(_obj)
        }
    }
    fn IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsEnabled(_obj)
        }
    }
    fn IsSeparator(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsSeparator(_obj)
        }
    }
    fn IsSubMenu(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsSubMenu(_obj)
        }
    }
    fn SetCheckable(_obj: *u8 /* void* */, checkable: bool /* bool */) {
        unsafe {
            wxMenuItem_SetCheckable(_obj, checkable)
        }
    }
    fn SetHelp(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxMenuItem_SetHelp(_obj, str)
        }
    }
    fn SetId(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxMenuItem_SetId(_obj, id)
        }
    }
    fn SetSubMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxMenuItem_SetSubMenu(_obj, menu)
        }
    }
    fn SetText(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxMenuItem_SetText(_obj, str)
        }
    }
}
trait wxDynamicLibrary {
}
trait wxSound {
}
trait wxMoveEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxMoveEvent_CopyObject(_obj, obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMoveEvent_GetPosition(_obj)
        }
    }
}
trait wxToolWindow {
    // missing: wxToolWindow_AddMiniButton
    // missing: wxToolWindow_Create
    // missing: wxToolWindow_GetClient
    // missing: wxToolWindow_SetClient
    // missing: wxToolWindow_SetTitleFont
}
trait wxFontEnumerator {
    fn Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFontEnumerator_Create(_obj, _fnc)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFontEnumerator_Delete(_obj)
        }
    }
    fn EnumerateEncodings(_obj: *u8 /* void* */, facename: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontEnumerator_EnumerateEncodings(_obj, facename)
        }
    }
    fn EnumerateFacenames(_obj: *u8 /* void* */, encoding: c_int /* int */, fixedWidthOnly: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxFontEnumerator_EnumerateFacenames(_obj, encoding, fixedWidthOnly)
        }
    }
}
trait wxMultiCellCanvas {
    // missing: wxMultiCellCanvas_Add
    // missing: wxMultiCellCanvas_CalculateConstraints
    // missing: wxMultiCellCanvas_Create
    // missing: wxMultiCellCanvas_MaxCols
    // missing: wxMultiCellCanvas_MaxRows
    // missing: wxMultiCellCanvas_SetMinCellSize
}
trait cbStartBarDraggingEvent {
    // missing: cbStartBarDraggingEvent_Bar
    // missing: cbStartBarDraggingEvent_Pos
}
trait wxGridSizeEvent {
    fn GetRowOrCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizeEvent_GetRowOrCol(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridSizeEvent_GetPosition(_obj)
        }
    }
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridSizeEvent_ControlDown(_obj)
        }
    }
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridSizeEvent_MetaDown(_obj)
        }
    }
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridSizeEvent_ShiftDown(_obj)
        }
    }
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridSizeEvent_AltDown(_obj)
        }
    }
}
trait wxFTP {
}
trait wxWizard {
    fn Chain(f: *u8 /* void* */, s: *u8 /* void* */) {
        unsafe {
            wxWizard_Chain(f, s)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWizard_Create(_prt, _id, _txt, _bmp, arg0, arg1, arg2, arg3)
        }
    }
    fn GetCurrentPage(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizard_GetCurrentPage(_obj)
        }
    }
    fn GetPageSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizard_GetPageSize(_obj)
        }
    }
    fn RunWizard(_obj: *u8 /* void* */, firstPage: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWizard_RunWizard(_obj, firstPage)
        }
    }
    fn SetPageSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWizard_SetPageSize(_obj, arg0, arg1)
        }
    }
}
trait wxHtmlWinParser {
}
trait wxFontMapper {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxFontMapper_Create()
        }
    }
    fn GetAltForEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */, alt_encoding: *u8 /* void* */, _buf: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontMapper_GetAltForEncoding(_obj, encoding, alt_encoding, _buf)
        }
    }
    fn IsEncodingAvailable(_obj: *u8 /* void* */, encoding: c_int /* int */, _buf: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontMapper_IsEncodingAvailable(_obj, encoding, _buf)
        }
    }
}
trait wxPrintData {
    fn Assign(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxPrintData_Assign(_obj, data)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxPrintData_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPrintData_Delete(_obj)
        }
    }
    fn GetCollate(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintData_GetCollate(_obj)
        }
    }
    fn GetColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintData_GetColour(_obj)
        }
    }
    fn GetDuplex(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetDuplex(_obj)
        }
    }
    fn GetFilename(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetFilename(_obj)
        }
    }
    fn GetFontMetricPath(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetFontMetricPath(_obj)
        }
    }
    fn GetNoCopies(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetNoCopies(_obj)
        }
    }
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetOrientation(_obj)
        }
    }
    fn GetPaperId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetPaperId(_obj)
        }
    }
    fn GetPaperSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPaperSize(_obj)
        }
    }
    fn GetPreviewCommand(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPreviewCommand(_obj)
        }
    }
    fn GetPrintMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetPrintMode(_obj)
        }
    }
    fn GetPrinterCommand(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPrinterCommand(_obj)
        }
    }
    fn GetPrinterName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPrinterName(_obj)
        }
    }
    fn GetPrinterOptions(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPrinterOptions(_obj)
        }
    }
    fn GetPrinterScaleX(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxPrintData_GetPrinterScaleX(_obj)
        }
    }
    fn GetPrinterScaleY(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxPrintData_GetPrinterScaleY(_obj)
        }
    }
    fn GetPrinterTranslateX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetPrinterTranslateX(_obj)
        }
    }
    fn GetPrinterTranslateY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetPrinterTranslateY(_obj)
        }
    }
    fn GetQuality(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetQuality(_obj)
        }
    }
    fn SetCollate(_obj: *u8 /* void* */, flag: c_int /* int */) {
        unsafe {
            wxPrintData_SetCollate(_obj, flag)
        }
    }
    fn SetColour(_obj: *u8 /* void* */, colour: c_int /* int */) {
        unsafe {
            wxPrintData_SetColour(_obj, colour)
        }
    }
    fn SetDuplex(_obj: *u8 /* void* */, duplex: c_int /* int */) {
        unsafe {
            wxPrintData_SetDuplex(_obj, duplex)
        }
    }
    fn SetFilename(_obj: *u8 /* void* */, filename: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetFilename(_obj, filename)
        }
    }
    fn SetFontMetricPath(_obj: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetFontMetricPath(_obj, path)
        }
    }
    fn SetNoCopies(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintData_SetNoCopies(_obj, v)
        }
    }
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */) {
        unsafe {
            wxPrintData_SetOrientation(_obj, orient)
        }
    }
    fn SetPaperId(_obj: *u8 /* void* */, sizeId: c_int /* int */) {
        unsafe {
            wxPrintData_SetPaperId(_obj, sizeId)
        }
    }
    fn SetPaperSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPrintData_SetPaperSize(_obj, arg0, arg1)
        }
    }
    fn SetPreviewCommand(_obj: *u8 /* void* */, command: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetPreviewCommand(_obj, command)
        }
    }
    fn SetPrintMode(_obj: *u8 /* void* */, printMode: c_int /* int */) {
        unsafe {
            wxPrintData_SetPrintMode(_obj, printMode)
        }
    }
    fn SetPrinterCommand(_obj: *u8 /* void* */, command: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetPrinterCommand(_obj, command)
        }
    }
    fn SetPrinterName(_obj: *u8 /* void* */, name: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetPrinterName(_obj, name)
        }
    }
    fn SetPrinterOptions(_obj: *u8 /* void* */, options: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetPrinterOptions(_obj, options)
        }
    }
    fn SetPrinterScaleX(_obj: *u8 /* void* */, x: c_double /* double */) {
        unsafe {
            wxPrintData_SetPrinterScaleX(_obj, x)
        }
    }
    fn SetPrinterScaleY(_obj: *u8 /* void* */, y: c_double /* double */) {
        unsafe {
            wxPrintData_SetPrinterScaleY(_obj, y)
        }
    }
    fn SetPrinterScaling(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */) {
        unsafe {
            wxPrintData_SetPrinterScaling(_obj, x, y)
        }
    }
    fn SetPrinterTranslateX(_obj: *u8 /* void* */, x: c_int /* int */) {
        unsafe {
            wxPrintData_SetPrinterTranslateX(_obj, x)
        }
    }
    fn SetPrinterTranslateY(_obj: *u8 /* void* */, y: c_int /* int */) {
        unsafe {
            wxPrintData_SetPrinterTranslateY(_obj, y)
        }
    }
    fn SetPrinterTranslation(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPrintData_SetPrinterTranslation(_obj, arg0, arg1)
        }
    }
    fn SetQuality(_obj: *u8 /* void* */, quality: c_int /* int */) {
        unsafe {
            wxPrintData_SetQuality(_obj, quality)
        }
    }
}
trait wxSashEvent {
    fn Create(id: c_int /* int */, edge: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSashEvent_Create(id, edge)
        }
    }
    fn GetDragRect(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSashEvent_GetDragRect(_obj)
        }
    }
    fn GetDragStatus(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashEvent_GetDragStatus(_obj)
        }
    }
    fn GetEdge(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashEvent_GetEdge(_obj)
        }
    }
    fn SetDragRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxSashEvent_SetDragRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn SetDragStatus(_obj: *u8 /* void* */, status: c_int /* int */) {
        unsafe {
            wxSashEvent_SetDragStatus(_obj, status)
        }
    }
    fn SetEdge(_obj: *u8 /* void* */, edge: c_int /* int */) {
        unsafe {
            wxSashEvent_SetEdge(_obj, edge)
        }
    }
}
trait cbPluginEvent {
    // missing: cbPluginEvent_Pane
}
trait wxPropertyGridEvent {
    fn HasProperty(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPropertyGridEvent_HasProperty(_obj)
        }
    }
    fn GetProperty(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPropertyGridEvent_GetProperty(_obj)
        }
    }
}
trait wxSingleChoiceDialog {
}
trait wxObject {
    fn GetClassInfo(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxObject_GetClassInfo(_obj)
        }
    }
    fn IsKindOf(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxObject_IsKindOf(_obj, classInfo)
        }
    }
    fn IsScrolledWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxObject_IsScrolledWindow(_obj)
        }
    }
}
trait wxDropSource {
    fn DropSource_Create(data: *u8 /* void* */, win: *u8 /* void* */, copy: *u8 /* void* */, move: *u8 /* void* */, none: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            DropSource_Create(data, win, copy, move, none)
        }
    }
    fn DropSource_Delete(_obj: *u8 /* void* */) {
        unsafe {
            DropSource_Delete(_obj)
        }
    }
    fn DropSource_DoDragDrop(_obj: *u8 /* void* */, _move: c_int /* int */) -> c_int /* int */ {
        unsafe {
            DropSource_DoDragDrop(_obj, _move)
        }
    }
}
trait wxHtmlParser {
}
trait wxFindReplaceData {
    fn Create(flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceData_Create(flags)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceData_CreateDefault()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFindReplaceData_Delete(_obj)
        }
    }
    fn GetFindString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceData_GetFindString(_obj)
        }
    }
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFindReplaceData_GetFlags(_obj)
        }
    }
    fn GetReplaceString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceData_GetReplaceString(_obj)
        }
    }
    fn SetFindString(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxFindReplaceData_SetFindString(_obj, str)
        }
    }
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxFindReplaceData_SetFlags(_obj, flags)
        }
    }
    fn SetReplaceString(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxFindReplaceData_SetReplaceString(_obj, str)
        }
    }
}
trait wxToolLayoutItem {
    // missing: wxToolLayoutItem_IsSeparator
    // missing: wxToolLayoutItem_Rect
}
trait cbLeftDownEvent {
    // missing: cbLeftDownEvent_Pos
}
trait wxPlotCurve {
}
trait wxMBConv {
}
trait cbUpdatesManagerBase {
}
trait wxLogTextCtrl {
}
trait wxStatusBar {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStatusBar_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetBorderX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStatusBar_GetBorderX(_obj)
        }
    }
    fn GetBorderY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStatusBar_GetBorderY(_obj)
        }
    }
    fn GetFieldsCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStatusBar_GetFieldsCount(_obj)
        }
    }
    fn GetStatusText(_obj: *u8 /* void* */, number: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStatusBar_GetStatusText(_obj, number)
        }
    }
    fn SetFieldsCount(_obj: *u8 /* void* */, number: c_int /* int */, widths: *c_int /* int* */) {
        unsafe {
            wxStatusBar_SetFieldsCount(_obj, number, widths)
        }
    }
    fn SetMinHeight(_obj: *u8 /* void* */, height: c_int /* int */) {
        unsafe {
            wxStatusBar_SetMinHeight(_obj, height)
        }
    }
    fn SetStatusText(_obj: *u8 /* void* */, text: *u8 /* void* */, number: c_int /* int */) {
        unsafe {
            wxStatusBar_SetStatusText(_obj, text, number)
        }
    }
    fn SetStatusWidths(_obj: *u8 /* void* */, n: c_int /* int */, widths: *c_int /* int* */) {
        unsafe {
            wxStatusBar_SetStatusWidths(_obj, n, widths)
        }
    }
}
trait wxTextDataObject {
    fn TextDataObject_Create(_txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            TextDataObject_Create(_txt)
        }
    }
    fn TextDataObject_Delete(_obj: *u8 /* void* */) {
        unsafe {
            TextDataObject_Delete(_obj)
        }
    }
    fn TextDataObject_GetTextLength(_obj: *u8 /* void* */) -> size_t /* size_t */ {
        unsafe {
            TextDataObject_GetTextLength(_obj)
        }
    }
    fn TextDataObject_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            TextDataObject_GetText(_obj)
        }
    }
    fn TextDataObject_SetText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            TextDataObject_SetText(_obj, text)
        }
    }
}
trait wxSizer {
    fn Add(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_Add(_obj, arg0, arg1, option, flag, border, userData)
        }
    }
    fn AddSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_AddSizer(_obj, sizer, option, flag, border, userData)
        }
    }
    fn AddWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_AddWindow(_obj, window, option, flag, border, userData)
        }
    }
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_CalcMin(_obj)
        }
    }
    fn Fit(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizer_Fit(_obj, window)
        }
    }
    fn GetChildren(_obj: *u8 /* void* */, _res: *u8 /* void* */, _cnt: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxSizer_GetChildren(_obj, _res, _cnt)
        }
    }
    fn GetMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetMinSize(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetPosition(_obj)
        }
    }
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetSize(_obj)
        }
    }
    fn Insert(_obj: *u8 /* void* */, before: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_Insert(_obj, before, arg0, arg1, option, flag, border, userData)
        }
    }
    fn InsertSizer(_obj: *u8 /* void* */, before: c_int /* int */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_InsertSizer(_obj, before, sizer, option, flag, border, userData)
        }
    }
    fn InsertWindow(_obj: *u8 /* void* */, before: c_int /* int */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_InsertWindow(_obj, before, window, option, flag, border, userData)
        }
    }
    fn Layout(_obj: *u8 /* void* */) {
        unsafe {
            wxSizer_Layout(_obj)
        }
    }
    fn Prepend(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_Prepend(_obj, arg0, arg1, option, flag, border, userData)
        }
    }
    fn PrependSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_PrependSizer(_obj, sizer, option, flag, border, userData)
        }
    }
    fn PrependWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_PrependWindow(_obj, window, option, flag, border, userData)
        }
    }
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxSizer_RecalcSizes(_obj)
        }
    }
    fn SetDimension(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxSizer_SetDimension(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn SetItemMinSize(_obj: *u8 /* void* */, pos: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizer_SetItemMinSize(_obj, pos, arg0, arg1)
        }
    }
    fn SetItemMinSizeSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizer_SetItemMinSizeSizer(_obj, sizer, arg0, arg1)
        }
    }
    fn SetItemMinSizeWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizer_SetItemMinSizeWindow(_obj, window, arg0, arg1)
        }
    }
    fn SetMinSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizer_SetMinSize(_obj, arg0, arg1)
        }
    }
    fn SetSizeHints(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizer_SetSizeHints(_obj, window)
        }
    }
    fn AddSpacer(_obj: *u8 /* void* */, size: c_int /* int */) {
        unsafe {
            wxSizer_AddSpacer(_obj, size)
        }
    }
    fn AddStretchSpacer(_obj: *u8 /* void* */, size: c_int /* int */) {
        unsafe {
            wxSizer_AddStretchSpacer(_obj, size)
        }
    }
    fn Clear(_obj: *u8 /* void* */, delete_windows: bool /* bool */) {
        unsafe {
            wxSizer_Clear(_obj, delete_windows)
        }
    }
    fn DetachWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_DetachWindow(_obj, window)
        }
    }
    fn DetachSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_DetachSizer(_obj, sizer)
        }
    }
    fn Detach(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSizer_Detach(_obj, index)
        }
    }
    fn FitInside(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizer_FitInside(_obj, window)
        }
    }
    fn GetContainingWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetContainingWindow(_obj)
        }
    }
    fn GetItemWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, recursive: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetItemWindow(_obj, window, recursive)
        }
    }
    fn GetItemSizer(_obj: *u8 /* void* */, window: *u8 /* void* */, recursive: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetItemSizer(_obj, window, recursive)
        }
    }
    fn GetItem(_obj: *u8 /* void* */, index: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetItem(_obj, index)
        }
    }
    fn HideWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_HideWindow(_obj, window)
        }
    }
    fn HideSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_HideSizer(_obj, sizer)
        }
    }
    fn Hide(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSizer_Hide(_obj, index)
        }
    }
    fn InsertSpacer(_obj: *u8 /* void* */, index: c_int /* int */, size: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_InsertSpacer(_obj, index, size)
        }
    }
    fn InsertStretchSpacer(_obj: *u8 /* void* */, index: c_int /* int */, prop: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_InsertStretchSpacer(_obj, index, prop)
        }
    }
    fn IsShownWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_IsShownWindow(_obj, window)
        }
    }
    fn IsShownSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_IsShownSizer(_obj, sizer)
        }
    }
    fn IsShown(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSizer_IsShown(_obj, index)
        }
    }
    fn PrependSpacer(_obj: *u8 /* void* */, size: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_PrependSpacer(_obj, size)
        }
    }
    fn PrependStretchSpacer(_obj: *u8 /* void* */, prop: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_PrependStretchSpacer(_obj, prop)
        }
    }
    fn ReplaceWindow(_obj: *u8 /* void* */, oldwin: *u8 /* void* */, newwin: *u8 /* void* */, recursive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_ReplaceWindow(_obj, oldwin, newwin, recursive)
        }
    }
    fn ReplaceSizer(_obj: *u8 /* void* */, oldsz: *u8 /* void* */, newsz: *u8 /* void* */, recursive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_ReplaceSizer(_obj, oldsz, newsz, recursive)
        }
    }
    fn Replace(_obj: *u8 /* void* */, oldindex: c_int /* int */, newitem: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_Replace(_obj, oldindex, newitem)
        }
    }
    fn SetVirtualSizeHints(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizer_SetVirtualSizeHints(_obj, window)
        }
    }
    fn ShowWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, show: bool /* bool */, recursive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_ShowWindow(_obj, window, show, recursive)
        }
    }
    fn ShowSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, show: bool /* bool */, recursive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_ShowSizer(_obj, sizer, show, recursive)
        }
    }
    fn Show(_obj: *u8 /* void* */, sizer: *u8 /* void* */, index: c_int /* int */, show: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_Show(_obj, sizer, index, show)
        }
    }
}
trait wxMenu {
    fn Append(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */) {
        unsafe {
            wxMenu_Append(_obj, id, text, help, isCheckable)
        }
    }
    fn AppendItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_AppendItem(_obj, _itm)
        }
    }
    fn AppendSeparator(_obj: *u8 /* void* */) {
        unsafe {
            wxMenu_AppendSeparator(_obj)
        }
    }
    fn AppendSub(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */) {
        unsafe {
            wxMenu_AppendSub(_obj, id, text, submenu, help)
        }
    }
    fn Break(_obj: *u8 /* void* */) {
        unsafe {
            wxMenu_Break(_obj)
        }
    }
    fn Check(_obj: *u8 /* void* */, id: c_int /* int */, check: bool /* bool */) {
        unsafe {
            wxMenu_Check(_obj, id, check)
        }
    }
    fn Create(title: *u8 /* void* */, style: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_Create(title, style)
        }
    }
    fn DeleteById(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxMenu_DeleteById(_obj, id)
        }
    }
    fn DeleteByItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_DeleteByItem(_obj, _itm)
        }
    }
    fn DeletePointer(_obj: *u8 /* void* */) {
        unsafe {
            wxMenu_DeletePointer(_obj)
        }
    }
    fn DestroyById(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxMenu_DestroyById(_obj, id)
        }
    }
    fn DestroyByItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_DestroyByItem(_obj, _itm)
        }
    }
    fn Enable(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxMenu_Enable(_obj, id, enable)
        }
    }
    fn FindItem(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_FindItem(_obj, id)
        }
    }
    fn FindItemByLabel(_obj: *u8 /* void* */, itemString: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenu_FindItemByLabel(_obj, itemString)
        }
    }
    fn GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetClientData(_obj)
        }
    }
    fn GetHelpString(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetHelpString(_obj, id)
        }
    }
    fn GetInvokingWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetInvokingWindow(_obj)
        }
    }
    fn GetLabel(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetLabel(_obj, id)
        }
    }
    fn GetMenuItemCount(_obj: *u8 /* void* */) -> size_t /* size_t */ {
        unsafe {
            wxMenu_GetMenuItemCount(_obj)
        }
    }
    fn GetMenuItems(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenu_GetMenuItems(_obj, _lst)
        }
    }
    fn GetParent(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetParent(_obj)
        }
    }
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenu_GetStyle(_obj)
        }
    }
    fn GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetTitle(_obj)
        }
    }
    fn Insert(_obj: *u8 /* void* */, pos: size_t /* size_t */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */) {
        unsafe {
            wxMenu_Insert(_obj, pos, id, text, help, isCheckable)
        }
    }
    fn InsertItem(_obj: *u8 /* void* */, pos: size_t /* size_t */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_InsertItem(_obj, pos, _itm)
        }
    }
    fn InsertSub(_obj: *u8 /* void* */, pos: size_t /* size_t */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */) {
        unsafe {
            wxMenu_InsertSub(_obj, pos, id, text, submenu, help)
        }
    }
    fn IsAttached(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenu_IsAttached(_obj)
        }
    }
    fn IsChecked(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMenu_IsChecked(_obj, id)
        }
    }
    fn IsEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMenu_IsEnabled(_obj, id)
        }
    }
    fn Prepend(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */) {
        unsafe {
            wxMenu_Prepend(_obj, id, text, help, isCheckable)
        }
    }
    fn PrependItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_PrependItem(_obj, _itm)
        }
    }
    fn PrependSub(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */) {
        unsafe {
            wxMenu_PrependSub(_obj, id, text, submenu, help)
        }
    }
    fn RemoveById(_obj: *u8 /* void* */, id: c_int /* int */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_RemoveById(_obj, id, _itm)
        }
    }
    fn RemoveByItem(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxMenu_RemoveByItem(_obj, item)
        }
    }
    fn SetClientData(_obj: *u8 /* void* */, clientData: *u8 /* void* */) {
        unsafe {
            wxMenu_SetClientData(_obj, clientData)
        }
    }
    fn SetEventHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxMenu_SetEventHandler(_obj, handler)
        }
    }
    fn SetHelpString(_obj: *u8 /* void* */, id: c_int /* int */, helpString: *u8 /* void* */) {
        unsafe {
            wxMenu_SetHelpString(_obj, id, helpString)
        }
    }
    fn SetInvokingWindow(_obj: *u8 /* void* */, win: *u8 /* void* */) {
        unsafe {
            wxMenu_SetInvokingWindow(_obj, win)
        }
    }
    fn SetLabel(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxMenu_SetLabel(_obj, id, label)
        }
    }
    fn SetParent(_obj: *u8 /* void* */, parent: *u8 /* void* */) {
        unsafe {
            wxMenu_SetParent(_obj, parent)
        }
    }
    fn SetTitle(_obj: *u8 /* void* */, title: *u8 /* void* */) {
        unsafe {
            wxMenu_SetTitle(_obj, title)
        }
    }
    fn UpdateUI(_obj: *u8 /* void* */, source: *u8 /* void* */) {
        unsafe {
            wxMenu_UpdateUI(_obj, source)
        }
    }
}
trait wxFileType {
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFileType_Delete(_obj)
        }
    }
    fn ExpandCommand(_obj: *u8 /* void* */, _cmd: *u8 /* void* */, _params: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileType_ExpandCommand(_obj, _cmd, _params)
        }
    }
    fn GetDescription(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileType_GetDescription(_obj)
        }
    }
    fn GetExtensions(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetExtensions(_obj, _lst)
        }
    }
    fn GetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetIcon(_obj, icon)
        }
    }
    fn GetMimeType(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileType_GetMimeType(_obj)
        }
    }
    fn GetMimeTypes(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetMimeTypes(_obj, _lst)
        }
    }
    fn GetOpenCommand(_obj: *u8 /* void* */, _buf: *u8 /* void* */, _params: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetOpenCommand(_obj, _buf, _params)
        }
    }
    fn GetPrintCommand(_obj: *u8 /* void* */, _buf: *u8 /* void* */, _params: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetPrintCommand(_obj, _buf, _params)
        }
    }
}
trait wxRadioButton {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxRadioButton_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRadioButton_GetValue(_obj)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, value: bool /* bool */) {
        unsafe {
            wxRadioButton_SetValue(_obj, value)
        }
    }
}
trait wxFontList {
}
trait cbDrawRowHandlesEvent {
    // missing: cbDrawRowHandlesEvent_Dc
    // missing: cbDrawRowHandlesEvent_Row
}
trait wxCheckListBox {
    fn Check(_obj: *u8 /* void* */, item: c_int /* int */, check: bool /* bool */) {
        unsafe {
            wxCheckListBox_Check(_obj, item, check)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCheckListBox_Create(_prt, _id, arg0, arg1, arg2, arg3, arg4, arg5, _stl)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCheckListBox_Delete(_obj)
        }
    }
    fn IsChecked(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxCheckListBox_IsChecked(_obj, item)
        }
    }
}
trait wxTreeLayoutStored {
}
trait wxSocketInputStream {
}
trait wxDocChildFrame {
}
trait wxCloseEvent {
    fn CanVeto(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCloseEvent_CanVeto(_obj)
        }
    }
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxCloseEvent_CopyObject(_obj, obj)
        }
    }
    fn GetLoggingOff(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCloseEvent_GetLoggingOff(_obj)
        }
    }
    fn GetVeto(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCloseEvent_GetVeto(_obj)
        }
    }
    fn SetCanVeto(_obj: *u8 /* void* */, canVeto: bool /* bool */) {
        unsafe {
            wxCloseEvent_SetCanVeto(_obj, canVeto)
        }
    }
    fn SetLoggingOff(_obj: *u8 /* void* */, logOff: bool /* bool */) {
        unsafe {
            wxCloseEvent_SetLoggingOff(_obj, logOff)
        }
    }
    fn Veto(_obj: *u8 /* void* */, veto: bool /* bool */) {
        unsafe {
            wxCloseEvent_Veto(_obj, veto)
        }
    }
}
trait wxFileProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileProperty_Create(label, name, value)
        }
    }
}
trait wxChoice {
    fn Append(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxChoice_Append(_obj, item)
        }
    }
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxChoice_Clear(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxChoice_Create(_prt, _id, arg0, arg1, arg2, arg3, arg4, arg5, _stl)
        }
    }
    fn Delete(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxChoice_Delete(_obj, n)
        }
    }
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxChoice_FindString(_obj, s)
        }
    }
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxChoice_GetCount(_obj)
        }
    }
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxChoice_GetSelection(_obj)
        }
    }
    fn GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxChoice_GetString(_obj, n)
        }
    }
    fn SetSelection(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxChoice_SetSelection(_obj, n)
        }
    }
    fn SetString(_obj: *u8 /* void* */, n: c_int /* int */, s: *u8 /* void* */) {
        unsafe {
            wxChoice_SetString(_obj, n, s)
        }
    }
}
trait wxIntProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxIntProperty_Create(label, name, value)
        }
    }
}
trait wxDbColFor {
}
trait wxMenuEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxMenuEvent_CopyObject(_obj, obj)
        }
    }
    fn GetMenuId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuEvent_GetMenuId(_obj)
        }
    }
}
trait wxCalendarDateAttr {
    fn Create(_ctxt: *u8 /* void* */, _cbck: *u8 /* void* */, _cbrd: *u8 /* void* */, _fnt: *u8 /* void* */, _brd: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxCalendarDateAttr_CreateDefault()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_Delete(_obj)
        }
    }
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_GetBackgroundColour(_obj, _ref)
        }
    }
    fn GetBorder(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCalendarDateAttr_GetBorder(_obj)
        }
    }
    fn GetBorderColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_GetBorderColour(_obj, _ref)
        }
    }
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_GetFont(_obj, _ref)
        }
    }
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_GetTextColour(_obj, _ref)
        }
    }
    fn HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasBackgroundColour(_obj)
        }
    }
    fn HasBorder(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasBorder(_obj)
        }
    }
    fn HasBorderColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasBorderColour(_obj)
        }
    }
    fn HasFont(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasFont(_obj)
        }
    }
    fn HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasTextColour(_obj)
        }
    }
    fn IsHoliday(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_IsHoliday(_obj)
        }
    }
    fn SetBackgroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_SetBackgroundColour(_obj, col)
        }
    }
    fn SetBorder(_obj: *u8 /* void* */, border: c_int /* int */) {
        unsafe {
            wxCalendarDateAttr_SetBorder(_obj, border)
        }
    }
    fn SetBorderColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_SetBorderColour(_obj, col)
        }
    }
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_SetFont(_obj, font)
        }
    }
    fn SetHoliday(_obj: *u8 /* void* */, holiday: c_int /* int */) {
        unsafe {
            wxCalendarDateAttr_SetHoliday(_obj, holiday)
        }
    }
    fn SetTextColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_SetTextColour(_obj, col)
        }
    }
}
trait wxGridCellNumberRenderer {
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellNumberRenderer_Ctor()
        }
    }
}
trait wxMessageDialog {
    fn Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _cap: *u8 /* void* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMessageDialog_Create(_prt, _msg, _cap, _stl)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMessageDialog_Delete(_obj)
        }
    }
    fn ShowModal(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMessageDialog_ShowModal(_obj)
        }
    }
}
trait wxPrivateDropTarget {
}
trait wxEncodingConverter {
    fn Convert(_obj: *u8 /* void* */, input: *u8 /* void* */, output: *u8 /* void* */) {
        unsafe {
            wxEncodingConverter_Convert(_obj, input, output)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxEncodingConverter_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxEncodingConverter_Delete(_obj)
        }
    }
    fn GetAllEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEncodingConverter_GetAllEquivalents(_obj, enc, _lst)
        }
    }
    fn GetPlatformEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, platform: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEncodingConverter_GetPlatformEquivalents(_obj, enc, platform, _lst)
        }
    }
    fn Init(_obj: *u8 /* void* */, input_enc: c_int /* int */, output_enc: c_int /* int */, method: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxEncodingConverter_Init(_obj, input_enc, output_enc, method)
        }
    }
}
trait wxCriticalSection {
    // missing: wxCriticalSection_Create
    // missing: wxCriticalSection_Delete
    // missing: wxCriticalSection_Enter
    // missing: wxCriticalSection_Leave
}
trait wxTextFile {
}
trait wxUpdateUIEvent {
    fn Check(_obj: *u8 /* void* */, check: bool /* bool */) {
        unsafe {
            wxUpdateUIEvent_Check(_obj, check)
        }
    }
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxUpdateUIEvent_CopyObject(_obj, obj)
        }
    }
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxUpdateUIEvent_Enable(_obj, enable)
        }
    }
    fn GetChecked(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetChecked(_obj)
        }
    }
    fn GetEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetEnabled(_obj)
        }
    }
    fn GetSetChecked(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetSetChecked(_obj)
        }
    }
    fn GetSetEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetSetEnabled(_obj)
        }
    }
    fn GetSetText(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetSetText(_obj)
        }
    }
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxUpdateUIEvent_GetText(_obj)
        }
    }
    fn SetText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxUpdateUIEvent_SetText(_obj, text)
        }
    }
}
trait wxTimeSpan {
}
trait wxGridSizer {
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridSizer_CalcMin(_obj)
        }
    }
    fn Create(rows: c_int /* int */, cols: c_int /* int */, vgap: c_int /* int */, hgap: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridSizer_Create(rows, cols, vgap, hgap)
        }
    }
    fn GetCols(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizer_GetCols(_obj)
        }
    }
    fn GetHGap(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizer_GetHGap(_obj)
        }
    }
    fn GetRows(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizer_GetRows(_obj)
        }
    }
    fn GetVGap(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizer_GetVGap(_obj)
        }
    }
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxGridSizer_RecalcSizes(_obj)
        }
    }
    fn SetCols(_obj: *u8 /* void* */, cols: c_int /* int */) {
        unsafe {
            wxGridSizer_SetCols(_obj, cols)
        }
    }
    fn SetHGap(_obj: *u8 /* void* */, gap: c_int /* int */) {
        unsafe {
            wxGridSizer_SetHGap(_obj, gap)
        }
    }
    fn SetRows(_obj: *u8 /* void* */, rows: c_int /* int */) {
        unsafe {
            wxGridSizer_SetRows(_obj, rows)
        }
    }
    fn SetVGap(_obj: *u8 /* void* */, gap: c_int /* int */) {
        unsafe {
            wxGridSizer_SetVGap(_obj, gap)
        }
    }
}
trait wxThread {
}
trait ELJMessageParameters {
    // missing: wxMessageParameters_Create
    // missing: wxMessageParameters_Delete
}
trait wxPopupTransientWindow {
}
trait ELJLocale {
}
trait wxEditableListBox {
    // missing: wxEditableListBox_Create
    // missing: wxEditableListBox_GetDelButton
    // missing: wxEditableListBox_GetDownButton
    // missing: wxEditableListBox_GetEditButton
    // missing: wxEditableListBox_GetListCtrl
    // missing: wxEditableListBox_GetNewButton
    // missing: wxEditableListBox_GetStrings
    // missing: wxEditableListBox_GetUpButton
    // missing: wxEditableListBox_SetStrings
}
trait cbDrawPaneBkGroundEvent {
    // missing: cbDrawPaneBkGroundEvent_Dc
}
trait wxSize {
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSize_Create(arg0, arg1)
        }
    }
    // missing: wxSize_Destroy
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSize_GetHeight(_obj)
        }
    }
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSize_GetWidth(_obj)
        }
    }
    fn SetHeight(_obj: *u8 /* void* */, h: c_int /* int */) {
        unsafe {
            wxSize_SetHeight(_obj, h)
        }
    }
    fn SetWidth(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxSize_SetWidth(_obj, w)
        }
    }
}
trait cbDynToolBarDimHandler {
    // missing: cbDynToolBarDimHandler_Create
    // missing: cbDynToolBarDimHandler_Delete
}
trait cbGCUpdatesMgr {
    // missing: cbGCUpdatesMgr_Create
    // missing: cbGCUpdatesMgr_CreateDefault
    // missing: cbGCUpdatesMgr_Delete
    // missing: cbGCUpdatesMgr_UpdateNow
}
trait wxFileDataObject {
    fn FileDataObject_AddFile(_obj: *u8 /* void* */, _fle: *u8 /* void* */) {
        unsafe {
            FileDataObject_AddFile(_obj, _fle)
        }
    }
    fn FileDataObject_Create(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */) -> *u8 /* void* */ {
        unsafe {
            FileDataObject_Create(arg0, arg1)
        }
    }
    fn FileDataObject_Delete(_obj: *u8 /* void* */) {
        unsafe {
            FileDataObject_Delete(_obj)
        }
    }
    fn FileDataObject_GetFilenames(_obj: *u8 /* void* */, _lst: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            FileDataObject_GetFilenames(_obj, _lst)
        }
    }
}
trait wxDocParentFrame {
}
trait wxSliderMSW {
    fn wxObject_Delete(obj: *u8 /* void* */) {
        unsafe {
            wxObject_Delete(obj)
        }
    }
    fn wxFrame_GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_GetTitle(_obj)
        }
    }
    fn wxFrame_SetTitle(_frame: *u8 /* void* */, _txt: *u8 /* void* */) {
        unsafe {
            wxFrame_SetTitle(_frame, _txt)
        }
    }
    fn wxFrame_SetShape(self_: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFrame_SetShape(self_, region)
        }
    }
    fn wxFrame_ShowFullScreen(self_: *u8 /* void* */, show: bool /* bool */, style: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxFrame_ShowFullScreen(self_, show, style)
        }
    }
    fn wxFrame_IsFullScreen(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFrame_IsFullScreen(self_)
        }
    }
    fn wxFrame_Centre(self_: *u8 /* void* */, orientation: c_int /* int */) {
        unsafe {
            wxFrame_Centre(self_, orientation)
        }
    }
    fn wxCursor_Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCursor_Delete(_obj)
        }
    }
    fn wxDateTime_Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDateTime_Delete(_obj)
        }
    }
    fn wxMouseEvent_GetWheelDelta(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetWheelDelta(_obj)
        }
    }
    fn wxMouseEvent_GetWheelRotation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetWheelRotation(_obj)
        }
    }
    fn wxMouseEvent_GetButton(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetButton(_obj)
        }
    }
    fn wxcGetMousePosition() -> *u8 /* void* */ {
        unsafe {
            wxcGetMousePosition()
        }
    }
    fn wxDC_GetUserScaleX(dc: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxDC_GetUserScaleX(dc)
        }
    }
    fn wxDC_GetUserScaleY(dc: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxDC_GetUserScaleY(dc)
        }
    }
    fn wxWindow_ConvertDialogToPixelsEx(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ConvertDialogToPixelsEx(_obj)
        }
    }
    fn wxWindow_ConvertPixelsToDialogEx(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ConvertPixelsToDialogEx(_obj)
        }
    }
    fn wxWindow_ScreenToClient2(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ScreenToClient2(_obj, arg0, arg1)
        }
    }
    fn wxString_Create(buffer: *wchar_t /* wchar_t* */) -> *u8 /* void* */ {
        unsafe {
            wxString_Create(buffer)
        }
    }
    fn wxString_CreateLen(buffer: *wchar_t /* wchar_t* */, len: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxString_CreateLen(buffer, len)
        }
    }
    fn wxString_Delete(s: *u8 /* void* */) {
        unsafe {
            wxString_Delete(s)
        }
    }
    fn wxString_GetString(s: *u8 /* void* */, buffer: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxString_GetString(s, buffer)
        }
    }
    fn wxString_Length(s: *u8 /* void* */) -> size_t /* size_t */ {
        unsafe {
            wxString_Length(s)
        }
    }
    fn wxMenu_GetMenuBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetMenuBar(_obj)
        }
    }
    fn wxMenuBar_GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetFrame(_obj)
        }
    }
    fn wxListEvent_GetCacheFrom(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetCacheFrom(_obj)
        }
    }
    fn wxListEvent_GetCacheTo(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetCacheTo(_obj)
        }
    }
    fn wxListCtrl_AssignImageList(_obj: *u8 /* void* */, images: *u8 /* void* */, which: c_int /* int */) {
        unsafe {
            wxListCtrl_AssignImageList(_obj, images, which)
        }
    }
    fn wxListCtrl_GetColumn2(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) {
        unsafe {
            wxListCtrl_GetColumn2(_obj, col, item)
        }
    }
    fn wxListCtrl_GetItem2(_obj: *u8 /* void* */, info: *u8 /* void* */) {
        unsafe {
            wxListCtrl_GetItem2(_obj, info)
        }
    }
    fn wxListCtrl_GetItemPosition2(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemPosition2(_obj, item)
        }
    }
    fn wxListCtrl_SortItems2(_obj: *u8 /* void* */, closure: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SortItems2(_obj, closure)
        }
    }
}
trait wxURL {
}
trait wxHtmlContainerCell {
}
trait wxFrameLayout {
    // missing: wxFrameLayout_Activate
    // missing: wxFrameLayout_AddBar
    // missing: wxFrameLayout_AddPlugin
    // missing: wxFrameLayout_AddPluginBefore
    // missing: wxFrameLayout_ApplyBarProperties
    // missing: wxFrameLayout_CaptureEventsForPane
    // missing: wxFrameLayout_CaptureEventsForPlugin
    // missing: wxFrameLayout_Create
    // missing: wxFrameLayout_Deactivate
    // missing: wxFrameLayout_Delete
    // missing: wxFrameLayout_DestroyBarWindows
    // missing: wxFrameLayout_EnableFloating
    // missing: wxFrameLayout_FindBarByName
    // missing: wxFrameLayout_FindBarByWindow
    // missing: wxFrameLayout_FindPlugin
    // missing: wxFrameLayout_FirePluginEvent
    // missing: wxFrameLayout_GetBars
    // missing: wxFrameLayout_GetClientHeight
    // missing: wxFrameLayout_GetClientRect
    // missing: wxFrameLayout_GetClientWidth
    // missing: wxFrameLayout_GetFrameClient
    // missing: wxFrameLayout_GetPane
    // missing: wxFrameLayout_GetPaneProperties
    // missing: wxFrameLayout_GetParentFrame
    // missing: wxFrameLayout_GetTopPlugin
    // missing: wxFrameLayout_GetUpdatesManager
    // missing: wxFrameLayout_HasTopPlugin
    // missing: wxFrameLayout_HideBarWindows
    // missing: wxFrameLayout_InverseVisibility
    // missing: wxFrameLayout_OnLButtonDown
    // missing: wxFrameLayout_OnLButtonUp
    // missing: wxFrameLayout_OnLDblClick
    // missing: wxFrameLayout_OnMouseMove
    // missing: wxFrameLayout_OnRButtonDown
    // missing: wxFrameLayout_OnRButtonUp
    // missing: wxFrameLayout_OnSize
    // missing: wxFrameLayout_PopAllPlugins
    // missing: wxFrameLayout_PopPlugin
    // missing: wxFrameLayout_PushDefaultPlugins
    // missing: wxFrameLayout_PushPlugin
    // missing: wxFrameLayout_RecalcLayout
    // missing: wxFrameLayout_RedockBar
    // missing: wxFrameLayout_RefreshNow
    // missing: wxFrameLayout_ReleaseEventsFromPane
    // missing: wxFrameLayout_ReleaseEventsFromPlugin
    // missing: wxFrameLayout_RemoveBar
    // missing: wxFrameLayout_RemovePlugin
    // missing: wxFrameLayout_SetBarState
    // missing: wxFrameLayout_SetFrameClient
    // missing: wxFrameLayout_SetMargins
    // missing: wxFrameLayout_SetPaneBackground
    // missing: wxFrameLayout_SetPaneProperties
    // missing: wxFrameLayout_SetTopPlugin
    // missing: wxFrameLayout_SetUpdatesManager
}
trait wxDDEServer {
}
trait cbDrawRowDecorEvent {
    // missing: cbDrawRowDecorEvent_Dc
    // missing: cbDrawRowDecorEvent_Row
}
trait wxcTreeItemData {
    fn Create(closure: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcTreeItemData_Create(closure)
        }
    }
    fn GetClientClosure(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcTreeItemData_GetClientClosure(self_)
        }
    }
    fn SetClientClosure(self_: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxcTreeItemData_SetClientClosure(self_, closure)
        }
    }
    fn wxTreeItemId_Clone(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeItemId_Clone(_obj)
        }
    }
    fn wxTreeItemId_CreateFromValue(value: intptr_t /* intptr_t */) -> *u8 /* void* */ {
        unsafe {
            wxTreeItemId_CreateFromValue(value)
        }
    }
    fn wxTreeItemId_GetValue(_obj: *u8 /* void* */) -> intptr_t /* intptr_t */ {
        unsafe {
            wxTreeItemId_GetValue(_obj)
        }
    }
    fn wxTreeEvent_GetKeyEvent(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeEvent_GetKeyEvent(_obj)
        }
    }
    fn wxTreeEvent_IsEditCancelled(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeEvent_IsEditCancelled(_obj)
        }
    }
    fn wxTreeEvent_Allow(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeEvent_Allow(_obj)
        }
    }
    fn wxTreeCtrl_Create2(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_Create2(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn wxTreeCtrl_InsertItem2(_obj: *u8 /* void* */, parent: *u8 /* void* */, idPrevious: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, closure: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_InsertItem2(_obj, parent, idPrevious, text, image, selectedImage, closure, _item)
        }
    }
    fn wxTreeCtrl_InsertItemByIndex2(_obj: *u8 /* void* */, parent: *u8 /* void* */, index: c_int /* int */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, closure: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_InsertItemByIndex2(_obj, parent, index, text, image, selectedImage, closure, _item)
        }
    }
    fn wxTreeCtrl_GetItemClientClosure(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetItemClientClosure(_obj, item)
        }
    }
    fn wxTreeCtrl_SetItemClientClosure(_obj: *u8 /* void* */, item: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemClientClosure(_obj, item, closure)
        }
    }
    fn wxTreeCtrl_AssignImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_AssignImageList(_obj, imageList)
        }
    }
    fn wxTreeCtrl_AssignStateImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_AssignStateImageList(_obj, imageList)
        }
    }
    fn wxDC_GetPixel2(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */) {
        unsafe {
            wxDC_GetPixel2(_obj, arg0, arg1, col)
        }
    }
    fn wxScrolledWindow_SetScrollRate(_obj: *u8 /* void* */, xstep: c_int /* int */, ystep: c_int /* int */) {
        unsafe {
            wxScrolledWindow_SetScrollRate(_obj, xstep, ystep)
        }
    }
}
trait wxEvtHandler {
    fn AddPendingEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_AddPendingEvent(_obj, event)
        }
    }
    fn Connect(_obj: *u8 /* void* */, first: c_int /* int */, last: c_int /* int */, type_: c_int /* int */, data: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEvtHandler_Connect(_obj, first, last, type_, data)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_Delete(_obj)
        }
    }
    fn Disconnect(_obj: *u8 /* void* */, first: c_int /* int */, last: c_int /* int */, type_: c_int /* int */, id: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxEvtHandler_Disconnect(_obj, first, last, type_, id)
        }
    }
    fn GetEvtHandlerEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxEvtHandler_GetEvtHandlerEnabled(_obj)
        }
    }
    fn GetNextHandler(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_GetNextHandler(_obj)
        }
    }
    fn GetPreviousHandler(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_GetPreviousHandler(_obj)
        }
    }
    fn ProcessEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxEvtHandler_ProcessEvent(_obj, event)
        }
    }
    fn ProcessPendingEvents(_obj: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_ProcessPendingEvents(_obj)
        }
    }
    fn SetEvtHandlerEnabled(_obj: *u8 /* void* */, enabled: bool /* bool */) {
        unsafe {
            wxEvtHandler_SetEvtHandlerEnabled(_obj, enabled)
        }
    }
    fn SetNextHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_SetNextHandler(_obj, handler)
        }
    }
    fn SetPreviousHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_SetPreviousHandler(_obj, handler)
        }
    }
}
trait wxSpinButton {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxSpinButton_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinButton_GetMax(_obj)
        }
    }
    fn GetMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinButton_GetMin(_obj)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinButton_GetValue(_obj)
        }
    }
    fn SetRange(_obj: *u8 /* void* */, minVal: c_int /* int */, maxVal: c_int /* int */) {
        unsafe {
            wxSpinButton_SetRange(_obj, minVal, maxVal)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxSpinButton_SetValue(_obj, val)
        }
    }
}
trait ELJDropTarget {
    fn Create(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJDropTarget_Create(_obj)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_Delete(_obj)
        }
    }
    fn SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnData(_obj, _func)
        }
    }
    fn SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnDragOver(_obj, _func)
        }
    }
    fn SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnDrop(_obj, _func)
        }
    }
    fn SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnEnter(_obj, _func)
        }
    }
    fn SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnLeave(_obj, _func)
        }
    }
}
trait wxFindReplaceDialog {
    fn Create(parent: *u8 /* void* */, data: *u8 /* void* */, title: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceDialog_Create(parent, data, title, style)
        }
    }
    fn GetData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceDialog_GetData(_obj)
        }
    }
    fn SetData(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxFindReplaceDialog_SetData(_obj, data)
        }
    }
}
trait wxFileInputStream {
}
trait wxIconizeEvent {
}
trait wxShowEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxShowEvent_CopyObject(_obj, obj)
        }
    }
    fn IsShown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxShowEvent_IsShown(_obj)
        }
    }
    fn SetShow(_obj: *u8 /* void* */, show: bool /* bool */) {
        unsafe {
            wxShowEvent_SetShow(_obj, show)
        }
    }
}
trait cbResizeBarEvent {
    // missing: cbResizeBarEvent_Bar
    // missing: cbResizeBarEvent_Row
}
trait wxHtmlTag {
}
trait wxMemoryDC {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxMemoryDC_Create()
        }
    }
    fn CreateCompatible(dc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMemoryDC_CreateCompatible(dc)
        }
    }
    fn CreateWithBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMemoryDC_CreateWithBitmap(bitmap)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMemoryDC_Delete(_obj)
        }
    }
    fn SelectObject(_obj: *u8 /* void* */, bitmap: *u8 /* void* */) {
        unsafe {
            wxMemoryDC_SelectObject(_obj, bitmap)
        }
    }
}
trait wxStringTokenizer {
}
trait wxFileSystem {
}
trait wxSlider95 {
}
trait wxPen {
    fn Assign(_obj: *u8 /* void* */, pen: *u8 /* void* */) {
        unsafe {
            wxPen_Assign(_obj, pen)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxPen_CreateDefault()
        }
    }
    fn CreateFromBitmap(stipple: *u8 /* void* */, width: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPen_CreateFromBitmap(stipple, width)
        }
    }
    fn CreateFromColour(col: *u8 /* void* */, width: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPen_CreateFromColour(col, width, style)
        }
    }
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPen_CreateFromStock(id)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPen_Delete(_obj)
        }
    }
    fn GetCap(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetCap(_obj)
        }
    }
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPen_GetColour(_obj, _ref)
        }
    }
    fn GetDashes(_obj: *u8 /* void* */, ptr: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetDashes(_obj, ptr)
        }
    }
    fn GetJoin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetJoin(_obj)
        }
    }
    fn GetStipple(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPen_GetStipple(_obj, _ref)
        }
    }
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetStyle(_obj)
        }
    }
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetWidth(_obj)
        }
    }
    fn IsEqual(_obj: *u8 /* void* */, pen: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPen_IsEqual(_obj, pen)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPen_IsOk(_obj)
        }
    }
    fn SetCap(_obj: *u8 /* void* */, cap: c_int /* int */) {
        unsafe {
            wxPen_SetCap(_obj, cap)
        }
    }
    fn SetColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxPen_SetColour(_obj, col)
        }
    }
    fn SetColourSingle(_obj: *u8 /* void* */, r: wchar_t /* wchar_t */, g: wchar_t /* wchar_t */, b: wchar_t /* wchar_t */) {
        unsafe {
            wxPen_SetColourSingle(_obj, r, g, b)
        }
    }
    fn SetDashes(_obj: *u8 /* void* */, nb_dashes: c_int /* int */, dash: *u8 /* void* */) {
        unsafe {
            wxPen_SetDashes(_obj, nb_dashes, dash)
        }
    }
    fn SetJoin(_obj: *u8 /* void* */, join: c_int /* int */) {
        unsafe {
            wxPen_SetJoin(_obj, join)
        }
    }
    fn SetStipple(_obj: *u8 /* void* */, stipple: *u8 /* void* */) {
        unsafe {
            wxPen_SetStipple(_obj, stipple)
        }
    }
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxPen_SetStyle(_obj, style)
        }
    }
    fn SetWidth(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxPen_SetWidth(_obj, width)
        }
    }
}
trait wxArtProvider {
    fn PopProvider() -> bool /* bool */ {
        unsafe {
            PopProvider()
        }
    }
    fn PushProvider(provider: *u8 /* void* */) {
        unsafe {
            PushProvider(provider)
        }
    }
    fn RemoveProvider(provider: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            RemoveProvider(provider)
        }
    }
}
trait wxLongLong {
}
trait wxTreeItemData {
}
trait wxListBox {
    fn Append(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxListBox_Append(_obj, item)
        }
    }
    fn AppendData(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxListBox_AppendData(_obj, item, data)
        }
    }
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxListBox_Clear(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListBox_Create(_prt, _id, arg0, arg1, arg2, arg3, arg4, arg5, _stl)
        }
    }
    fn Delete(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxListBox_Delete(_obj, n)
        }
    }
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListBox_FindString(_obj, s)
        }
    }
    fn GetClientData(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListBox_GetClientData(_obj, n)
        }
    }
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListBox_GetCount(_obj)
        }
    }
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListBox_GetSelection(_obj)
        }
    }
    fn GetSelections(_obj: *u8 /* void* */, aSelections: *c_int /* int* */, allocated: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListBox_GetSelections(_obj, aSelections, allocated)
        }
    }
    fn GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListBox_GetString(_obj, n)
        }
    }
    fn InsertItems(_obj: *u8 /* void* */, items: *u8 /* void* */, pos: c_int /* int */, count: c_int /* int */) {
        unsafe {
            wxListBox_InsertItems(_obj, items, pos, count)
        }
    }
    fn IsSelected(_obj: *u8 /* void* */, n: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListBox_IsSelected(_obj, n)
        }
    }
    fn SetClientData(_obj: *u8 /* void* */, n: c_int /* int */, clientData: *u8 /* void* */) {
        unsafe {
            wxListBox_SetClientData(_obj, n, clientData)
        }
    }
    fn SetFirstItem(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxListBox_SetFirstItem(_obj, n)
        }
    }
    fn SetSelection(_obj: *u8 /* void* */, n: c_int /* int */, select: c_int /* int */) {
        unsafe {
            wxListBox_SetSelection(_obj, n, select)
        }
    }
    fn SetString(_obj: *u8 /* void* */, n: c_int /* int */, s: *u8 /* void* */) {
        unsafe {
            wxListBox_SetString(_obj, n, s)
        }
    }
    fn SetStringSelection(_obj: *u8 /* void* */, str: *u8 /* void* */, sel: bool /* bool */) {
        unsafe {
            wxListBox_SetStringSelection(_obj, str, sel)
        }
    }
}
trait wxSemaphore {
}
trait wxFileDropTarget {
}
trait wxMutexLocker {
}
trait wxLog {
}
trait wxTipWindow {
    fn Close(_obj: *u8 /* void* */) {
        unsafe {
            wxTipWindow_Close(_obj)
        }
    }
    fn Create(parent: *u8 /* void* */, text: *u8 /* void* */, maxLength: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTipWindow_Create(parent, text, maxLength)
        }
    }
    fn SetBoundingRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxTipWindow_SetBoundingRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn SetTipWindowPtr(_obj: *u8 /* void* */, windowPtr: *u8 /* void* */) {
        unsafe {
            wxTipWindow_SetTipWindowPtr(_obj, windowPtr)
        }
    }
}
trait cbRightDownEvent {
    // missing: cbRightDownEvent_Pos
}
trait wxSplitterEvent {
}
trait ELJFileDropTarget {
    fn Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJFileDropTarget_Create(_obj, _func)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_Delete(_obj)
        }
    }
    fn SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnData(_obj, _func)
        }
    }
    fn SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnDragOver(_obj, _func)
        }
    }
    fn SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnDrop(_obj, _func)
        }
    }
    fn SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnEnter(_obj, _func)
        }
    }
    fn SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnLeave(_obj, _func)
        }
    }
}
trait wxTreeLayout {
}
trait cbSimpleCustomizationPlugin {
    // missing: cbSimpleCustomizationPlugin_Create
    // missing: cbSimpleCustomizationPlugin_CreateDefault
    // missing: cbSimpleCustomizationPlugin_Delete
}
trait ELJServer {
    // missing: ELJServer_Create
    // missing: ELJServer_Delete
    // missing: ELJServer_Initialize
}
trait wxMemoryFSHandler {
}
trait cbDrawBarHandlesEvent {
    // missing: cbDrawBarHandlesEvent_Bar
    // missing: cbDrawBarHandlesEvent_Dc
}
trait wxGridCellAttr {
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellAttr_Ctor()
        }
    }
    fn DecRef(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_DecRef(_obj)
        }
    }
    fn GetAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGridCellAttr_GetAlignment(_obj, arg0, arg1)
        }
    }
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_GetBackgroundColour(_obj, _ref)
        }
    }
    fn GetEditor(_obj: *u8 /* void* */, grid: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellAttr_GetEditor(_obj, grid, row, col)
        }
    }
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_GetFont(_obj, _ref)
        }
    }
    fn GetRenderer(_obj: *u8 /* void* */, grid: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellAttr_GetRenderer(_obj, grid, row, col)
        }
    }
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_GetTextColour(_obj, _ref)
        }
    }
    fn HasAlignment(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasAlignment(_obj)
        }
    }
    fn HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasBackgroundColour(_obj)
        }
    }
    fn HasEditor(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasEditor(_obj)
        }
    }
    fn HasFont(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasFont(_obj)
        }
    }
    fn HasRenderer(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasRenderer(_obj)
        }
    }
    fn HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasTextColour(_obj)
        }
    }
    fn IncRef(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_IncRef(_obj)
        }
    }
    fn IsReadOnly(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_IsReadOnly(_obj)
        }
    }
    fn SetAlignment(_obj: *u8 /* void* */, hAlign: c_int /* int */, vAlign: c_int /* int */) {
        unsafe {
            wxGridCellAttr_SetAlignment(_obj, hAlign, vAlign)
        }
    }
    fn SetBackgroundColour(_obj: *u8 /* void* */, colBack: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetBackgroundColour(_obj, colBack)
        }
    }
    fn SetDefAttr(_obj: *u8 /* void* */, defAttr: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetDefAttr(_obj, defAttr)
        }
    }
    fn SetEditor(_obj: *u8 /* void* */, editor: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetEditor(_obj, editor)
        }
    }
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetFont(_obj, font)
        }
    }
    fn SetReadOnly(_obj: *u8 /* void* */, isReadOnly: bool /* bool */) {
        unsafe {
            wxGridCellAttr_SetReadOnly(_obj, isReadOnly)
        }
    }
    fn SetRenderer(_obj: *u8 /* void* */, renderer: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetRenderer(_obj, renderer)
        }
    }
    fn SetTextColour(_obj: *u8 /* void* */, colText: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetTextColour(_obj, colText)
        }
    }
}
trait wxCalculateLayoutEvent {
    fn Create(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCalculateLayoutEvent_Create(id)
        }
    }
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCalculateLayoutEvent_GetFlags(_obj)
        }
    }
    fn GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCalculateLayoutEvent_GetRect(_obj)
        }
    }
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxCalculateLayoutEvent_SetFlags(_obj, flags)
        }
    }
    fn SetRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxCalculateLayoutEvent_SetRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
}
trait wxBusyCursor {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxBusyCursor_Create()
        }
    }
    fn CreateWithCursor(_cur: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBusyCursor_CreateWithCursor(_cur)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxBusyCursor_Delete(_obj)
        }
    }
}
trait wxJoystick {
    // missing: wxJoystick_Create
    // missing: wxJoystick_Delete
    // missing: wxJoystick_GetButtonState
    // missing: wxJoystick_GetManufacturerId
    // missing: wxJoystick_GetMaxAxes
    // missing: wxJoystick_GetMaxButtons
    // missing: wxJoystick_GetMovementThreshold
    // missing: wxJoystick_GetNumberAxes
    // missing: wxJoystick_GetNumberButtons
    // missing: wxJoystick_GetNumberJoysticks
    // missing: wxJoystick_GetPOVCTSPosition
    // missing: wxJoystick_GetPOVPosition
    // missing: wxJoystick_GetPollingMax
    // missing: wxJoystick_GetPollingMin
    // missing: wxJoystick_GetPosition
    // missing: wxJoystick_GetProductId
    // missing: wxJoystick_GetProductName
    // missing: wxJoystick_GetRudderMax
    // missing: wxJoystick_GetRudderMin
    // missing: wxJoystick_GetRudderPosition
    // missing: wxJoystick_GetUMax
    // missing: wxJoystick_GetUMin
    // missing: wxJoystick_GetUPosition
    // missing: wxJoystick_GetVMax
    // missing: wxJoystick_GetVMin
    // missing: wxJoystick_GetVPosition
    // missing: wxJoystick_GetXMax
    // missing: wxJoystick_GetXMin
    // missing: wxJoystick_GetYMax
    // missing: wxJoystick_GetYMin
    // missing: wxJoystick_GetZMax
    // missing: wxJoystick_GetZMin
    // missing: wxJoystick_GetZPosition
    // missing: wxJoystick_HasPOV
    // missing: wxJoystick_HasPOV4Dir
    // missing: wxJoystick_HasPOVCTS
    // missing: wxJoystick_HasRudder
    // missing: wxJoystick_HasU
    // missing: wxJoystick_HasV
    // missing: wxJoystick_HasZ
    // missing: wxJoystick_IsOk
    // missing: wxJoystick_ReleaseCapture
    // missing: wxJoystick_SetCapture
    // missing: wxJoystick_SetMovementThreshold
}
trait wxPrintDialogData {
    fn Assign(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_Assign(_obj, data)
        }
    }
    fn AssignData(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_AssignData(_obj, data)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxPrintDialogData_CreateDefault()
        }
    }
    fn CreateFromData(printData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintDialogData_CreateFromData(printData)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_Delete(_obj)
        }
    }
    fn EnableHelp(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_EnableHelp(_obj, flag)
        }
    }
    fn EnablePageNumbers(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_EnablePageNumbers(_obj, flag)
        }
    }
    fn EnablePrintToFile(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_EnablePrintToFile(_obj, flag)
        }
    }
    fn EnableSelection(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_EnableSelection(_obj, flag)
        }
    }
    fn GetAllPages(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetAllPages(_obj)
        }
    }
    fn GetCollate(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetCollate(_obj)
        }
    }
    fn GetEnableHelp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetEnableHelp(_obj)
        }
    }
    fn GetEnablePageNumbers(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetEnablePageNumbers(_obj)
        }
    }
    fn GetEnablePrintToFile(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetEnablePrintToFile(_obj)
        }
    }
    fn GetEnableSelection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetEnableSelection(_obj)
        }
    }
    fn GetFromPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetFromPage(_obj)
        }
    }
    fn GetMaxPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetMaxPage(_obj)
        }
    }
    fn GetMinPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetMinPage(_obj)
        }
    }
    fn GetNoCopies(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetNoCopies(_obj)
        }
    }
    fn GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_GetPrintData(_obj, _ref)
        }
    }
    fn GetPrintToFile(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetPrintToFile(_obj)
        }
    }
    fn GetSelection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetSelection(_obj)
        }
    }
    fn GetToPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetToPage(_obj)
        }
    }
    fn SetAllPages(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_SetAllPages(_obj, flag)
        }
    }
    fn SetCollate(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_SetCollate(_obj, flag)
        }
    }
    fn SetFromPage(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintDialogData_SetFromPage(_obj, v)
        }
    }
    fn SetMaxPage(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintDialogData_SetMaxPage(_obj, v)
        }
    }
    fn SetMinPage(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintDialogData_SetMinPage(_obj, v)
        }
    }
    fn SetNoCopies(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintDialogData_SetNoCopies(_obj, v)
        }
    }
    fn SetPrintData(_obj: *u8 /* void* */, printData: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_SetPrintData(_obj, printData)
        }
    }
    fn SetPrintToFile(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_SetPrintToFile(_obj, flag)
        }
    }
    fn SetSelection(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_SetSelection(_obj, flag)
        }
    }
    fn SetToPage(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintDialogData_SetToPage(_obj, v)
        }
    }
}
trait wxBrushList {
}
trait wxTipProvider {
}
trait wxImage {
    fn CanRead(name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_CanRead(name)
        }
    }
    fn ConvertToBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */) {
        unsafe {
            wxImage_ConvertToBitmap(_obj, bitmap)
        }
    }
    fn ConvertToByteString(_obj: *u8 /* void* */, type_: c_int /* int */, data: *char /* char* */) -> c_int /* int */ {
        unsafe {
            wxImage_ConvertToByteString(_obj, type_, data)
        }
    }
    fn ConvertToLazyByteString(_obj: *u8 /* void* */, type_: c_int /* int */, data: *char /* char* */) -> c_int /* int */ {
        unsafe {
            wxImage_ConvertToLazyByteString(_obj, type_, data)
        }
    }
    fn CountColours(_obj: *u8 /* void* */, stopafter: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxImage_CountColours(_obj, stopafter)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateDefault()
        }
    }
    fn CreateFromBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromBitmap(bitmap)
        }
    }
    fn CreateFromByteString(arg0: *char /* char* */, arg1: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromByteString(arg0, arg1, type_)
        }
    }
    fn CreateFromLazyByteString(arg0: *char /* char* */, arg1: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromLazyByteString(arg0, arg1, type_)
        }
    }
    fn CreateFromData(arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromData(arg0, arg1, data)
        }
    }
    fn CreateFromFile(name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromFile(name)
        }
    }
    fn CreateSized(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateSized(arg0, arg1)
        }
    }
    fn Destroy(_obj: *u8 /* void* */) {
        unsafe {
            wxImage_Destroy(_obj)
        }
    }
    fn GetBlue(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetBlue(_obj, arg0, arg1)
        }
    }
    fn GetData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_GetData(_obj)
        }
    }
    fn GetGreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetGreen(_obj, arg0, arg1)
        }
    }
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImage_GetHeight(_obj)
        }
    }
    fn GetMaskBlue(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetMaskBlue(_obj)
        }
    }
    fn GetMaskGreen(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetMaskGreen(_obj)
        }
    }
    fn GetMaskRed(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetMaskRed(_obj)
        }
    }
    fn GetRed(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetRed(_obj, arg0, arg1)
        }
    }
    fn GetSubImage(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, image: *u8 /* void* */) {
        unsafe {
            wxImage_GetSubImage(_obj, arg0, arg1, arg2, arg3, image)
        }
    }
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImage_GetWidth(_obj)
        }
    }
    fn HasMask(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_HasMask(_obj)
        }
    }
    fn GetOption(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_GetOption(_obj, name)
        }
    }
    fn GetOptionInt(_obj: *u8 /* void* */, name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_GetOptionInt(_obj, name)
        }
    }
    fn HasOption(_obj: *u8 /* void* */, name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_HasOption(_obj, name)
        }
    }
    fn Initialize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxImage_Initialize(_obj, arg0, arg1)
        }
    }
    fn InitializeFromData(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */) {
        unsafe {
            wxImage_InitializeFromData(_obj, arg0, arg1, data)
        }
    }
    fn LoadFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxImage_LoadFile(_obj, name, type_)
        }
    }
    fn Mirror(_obj: *u8 /* void* */, horizontally: c_int /* int */, image: *u8 /* void* */) {
        unsafe {
            wxImage_Mirror(_obj, horizontally, image)
        }
    }
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_IsOk(_obj)
        }
    }
    fn Paste(_obj: *u8 /* void* */, image: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxImage_Paste(_obj, image, arg0, arg1)
        }
    }
    fn Replace(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */, arg5: u8 /* u8 */) {
        unsafe {
            wxImage_Replace(_obj, arg0, arg1, arg2, arg3, arg4, arg5)
        }
    }
    fn Rescale(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxImage_Rescale(_obj, arg0, arg1)
        }
    }
    fn Rotate(_obj: *u8 /* void* */, angle: c_double /* double */, arg0: c_int /* int */, arg1: c_int /* int */, interpolating: c_int /* int */, offset_after_rotation: *u8 /* void* */, image: *u8 /* void* */) {
        unsafe {
            wxImage_Rotate(_obj, angle, arg0, arg1, interpolating, offset_after_rotation, image)
        }
    }
    fn Rotate90(_obj: *u8 /* void* */, clockwise: c_int /* int */, image: *u8 /* void* */) {
        unsafe {
            wxImage_Rotate90(_obj, clockwise, image)
        }
    }
    fn SaveFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxImage_SaveFile(_obj, name, type_)
        }
    }
    fn Scale(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, image: *u8 /* void* */) {
        unsafe {
            wxImage_Scale(_obj, arg0, arg1, image)
        }
    }
    fn SetData(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxImage_SetData(_obj, data)
        }
    }
    fn SetDataAndSize(_obj: *u8 /* void* */, data: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxImage_SetDataAndSize(_obj, data, arg0, arg1)
        }
    }
    fn SetMask(_obj: *u8 /* void* */, mask: c_int /* int */) {
        unsafe {
            wxImage_SetMask(_obj, mask)
        }
    }
    fn SetMaskColour(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxImage_SetMaskColour(_obj, arg0, arg1, arg2)
        }
    }
    fn SetOption(_obj: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) {
        unsafe {
            wxImage_SetOption(_obj, name, value)
        }
    }
    fn SetOptionInt(_obj: *u8 /* void* */, name: *u8 /* void* */, value: c_int /* int */) {
        unsafe {
            wxImage_SetOptionInt(_obj, name, value)
        }
    }
    fn SetRGB(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */) {
        unsafe {
            wxImage_SetRGB(_obj, arg0, arg1, arg2, arg3, arg4)
        }
    }
}
trait wxFileHistory {
    fn AddFileToHistory(_obj: *u8 /* void* */, file: *u8 /* void* */) {
        unsafe {
            wxFileHistory_AddFileToHistory(_obj, file)
        }
    }
    fn AddFilesToMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxFileHistory_AddFilesToMenu(_obj, menu)
        }
    }
    fn Create(maxFiles: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFileHistory_Create(maxFiles)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFileHistory_Delete(_obj)
        }
    }
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileHistory_GetCount(_obj)
        }
    }
    fn GetHistoryFile(_obj: *u8 /* void* */, i: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFileHistory_GetHistoryFile(_obj, i)
        }
    }
    fn GetMaxFiles(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileHistory_GetMaxFiles(_obj)
        }
    }
    fn GetMenus(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileHistory_GetMenus(_obj, _ref)
        }
    }
    fn Load(_obj: *u8 /* void* */, config: *u8 /* void* */) {
        unsafe {
            wxFileHistory_Load(_obj, config)
        }
    }
    fn RemoveFileFromHistory(_obj: *u8 /* void* */, i: c_int /* int */) {
        unsafe {
            wxFileHistory_RemoveFileFromHistory(_obj, i)
        }
    }
    fn RemoveMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxFileHistory_RemoveMenu(_obj, menu)
        }
    }
    fn Save(_obj: *u8 /* void* */, config: *u8 /* void* */) {
        unsafe {
            wxFileHistory_Save(_obj, config)
        }
    }
    fn UseMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxFileHistory_UseMenu(_obj, menu)
        }
    }
}
trait wxLogNull {
}
trait wxNewBitmapButton {
    // missing: wxNewBitmapButton_Create
    // missing: wxNewBitmapButton_CreateFromFile
    // missing: wxNewBitmapButton_Delete
    // missing: wxNewBitmapButton_DrawDecorations
    // missing: wxNewBitmapButton_DrawLabel
    // missing: wxNewBitmapButton_Enable
    // missing: wxNewBitmapButton_Realize
    // missing: wxNewBitmapButton_RenderAllLabelImages
    // missing: wxNewBitmapButton_RenderLabelImage
    // missing: wxNewBitmapButton_RenderLabelImages
    // missing: wxNewBitmapButton_Reshape
    // missing: wxNewBitmapButton_SetAlignments
    // missing: wxNewBitmapButton_SetLabel
}
trait wxEvent {
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxEvent_CopyObject(_obj, object_dest)
        }
    }
    fn GetEventObject(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEvent_GetEventObject(_obj)
        }
    }
    fn GetEventType(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEvent_GetEventType(_obj)
        }
    }
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEvent_GetId(_obj)
        }
    }
    fn GetSkipped(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxEvent_GetSkipped(_obj)
        }
    }
    fn GetTimestamp(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEvent_GetTimestamp(_obj)
        }
    }
    fn IsCommandEvent(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxEvent_IsCommandEvent(_obj)
        }
    }
    fn NewEventType() -> c_int /* int */ {
        unsafe {
            wxEvent_NewEventType()
        }
    }
    fn SetEventObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxEvent_SetEventObject(_obj, obj)
        }
    }
    fn SetEventType(_obj: *u8 /* void* */, typ: c_int /* int */) {
        unsafe {
            wxEvent_SetEventType(_obj, typ)
        }
    }
    fn SetId(_obj: *u8 /* void* */, Id: c_int /* int */) {
        unsafe {
            wxEvent_SetId(_obj, Id)
        }
    }
    fn SetTimestamp(_obj: *u8 /* void* */, ts: c_int /* int */) {
        unsafe {
            wxEvent_SetTimestamp(_obj, ts)
        }
    }
    fn Skip(_obj: *u8 /* void* */) {
        unsafe {
            wxEvent_Skip(_obj)
        }
    }
}
trait wxTimerBase {
}
trait wxGridCellBoolRenderer {
}
trait wxMemoryInputStream {
}
trait wxGridEvent {
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_AltDown(_obj)
        }
    }
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_ControlDown(_obj)
        }
    }
    fn GetCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridEvent_GetCol(_obj)
        }
    }
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridEvent_GetPosition(_obj)
        }
    }
    fn GetRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridEvent_GetRow(_obj)
        }
    }
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_MetaDown(_obj)
        }
    }
    fn Selecting(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_Selecting(_obj)
        }
    }
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_ShiftDown(_obj)
        }
    }
}
trait wxFFileInputStream {
}
trait wxClipboard {
    fn AddData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_AddData(_obj, data)
        }
    }
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxClipboard_Clear(_obj)
        }
    }
    fn Close(_obj: *u8 /* void* */) {
        unsafe {
            wxClipboard_Close(_obj)
        }
    }
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxClipboard_Create()
        }
    }
    fn Flush(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_Flush(_obj)
        }
    }
    fn GetData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_GetData(_obj, data)
        }
    }
    fn IsOpened(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_IsOpened(_obj)
        }
    }
    fn IsSupported(_obj: *u8 /* void* */, format: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_IsSupported(_obj, format)
        }
    }
    fn Open(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_Open(_obj)
        }
    }
    fn SetData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_SetData(_obj, data)
        }
    }
    fn UsePrimarySelection(_obj: *u8 /* void* */, primary: bool /* bool */) {
        unsafe {
            wxClipboard_UsePrimarySelection(_obj, primary)
        }
    }
}
trait wxStaticBox {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBox_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait wxMutex {
    // missing: wxMutex_Create
    // missing: wxMutex_Delete
    // missing: wxMutex_IsLocked
    // missing: wxMutex_Lock
    // missing: wxMutex_TryLock
    // missing: wxMutex_Unlock
}
trait wxStreamToTextRedirector {
}
trait wxModule {
}
trait wxPenList {
}
trait wxHtmlPrintout {
}
trait wxConfigBase {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_Create()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxConfigBase_Delete(_obj)
        }
    }
    fn DeleteAll(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_DeleteAll(_obj)
        }
    }
    fn DeleteEntry(_obj: *u8 /* void* */, key: *u8 /* void* */, bDeleteGroupIfEmpty: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_DeleteEntry(_obj, key, bDeleteGroupIfEmpty)
        }
    }
    fn DeleteGroup(_obj: *u8 /* void* */, key: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_DeleteGroup(_obj, key)
        }
    }
    fn Exists(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_Exists(_obj, strName)
        }
    }
    fn ExpandEnvVars(_obj: *u8 /* void* */, str: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_ExpandEnvVars(_obj, str)
        }
    }
    fn Flush(_obj: *u8 /* void* */, bCurrentOnly: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_Flush(_obj, bCurrentOnly)
        }
    }
    fn GetAppName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetAppName(_obj)
        }
    }
    fn GetEntryType(_obj: *u8 /* void* */, name: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_GetEntryType(_obj, name)
        }
    }
    fn GetFirstEntry(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetFirstEntry(_obj, lIndex)
        }
    }
    fn GetFirstGroup(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetFirstGroup(_obj, lIndex)
        }
    }
    fn GetNextEntry(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetNextEntry(_obj, lIndex)
        }
    }
    fn GetNextGroup(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetNextGroup(_obj, lIndex)
        }
    }
    fn GetNumberOfEntries(_obj: *u8 /* void* */, bRecursive: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_GetNumberOfEntries(_obj, bRecursive)
        }
    }
    fn GetNumberOfGroups(_obj: *u8 /* void* */, bRecursive: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_GetNumberOfGroups(_obj, bRecursive)
        }
    }
    fn GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetPath(_obj)
        }
    }
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_GetStyle(_obj)
        }
    }
    fn GetVendorName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetVendorName(_obj)
        }
    }
    fn HasEntry(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_HasEntry(_obj, strName)
        }
    }
    fn HasGroup(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_HasGroup(_obj, strName)
        }
    }
    fn IsExpandingEnvVars(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_IsExpandingEnvVars(_obj)
        }
    }
    fn IsRecordingDefaults(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_IsRecordingDefaults(_obj)
        }
    }
    fn ReadBool(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_ReadBool(_obj, key, defVal)
        }
    }
    fn ReadDouble(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: c_double /* double */) -> c_double /* double */ {
        unsafe {
            wxConfigBase_ReadDouble(_obj, key, defVal)
        }
    }
    fn ReadInteger(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_ReadInteger(_obj, key, defVal)
        }
    }
    fn ReadString(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_ReadString(_obj, key, defVal)
        }
    }
    fn RenameEntry(_obj: *u8 /* void* */, oldName: *u8 /* void* */, newName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_RenameEntry(_obj, oldName, newName)
        }
    }
    fn RenameGroup(_obj: *u8 /* void* */, oldName: *u8 /* void* */, newName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_RenameGroup(_obj, oldName, newName)
        }
    }
    fn SetAppName(_obj: *u8 /* void* */, appName: *u8 /* void* */) {
        unsafe {
            wxConfigBase_SetAppName(_obj, appName)
        }
    }
    fn SetExpandEnvVars(_obj: *u8 /* void* */, bDoIt: bool /* bool */) {
        unsafe {
            wxConfigBase_SetExpandEnvVars(_obj, bDoIt)
        }
    }
    fn SetPath(_obj: *u8 /* void* */, strPath: *u8 /* void* */) {
        unsafe {
            wxConfigBase_SetPath(_obj, strPath)
        }
    }
    fn SetRecordDefaults(_obj: *u8 /* void* */, bDoIt: bool /* bool */) {
        unsafe {
            wxConfigBase_SetRecordDefaults(_obj, bDoIt)
        }
    }
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxConfigBase_SetStyle(_obj, style)
        }
    }
    fn SetVendorName(_obj: *u8 /* void* */, vendorName: *u8 /* void* */) {
        unsafe {
            wxConfigBase_SetVendorName(_obj, vendorName)
        }
    }
    fn WriteBool(_obj: *u8 /* void* */, key: *u8 /* void* */, value: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteBool(_obj, key, value)
        }
    }
    fn WriteDouble(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_double /* double */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteDouble(_obj, key, value)
        }
    }
    fn WriteInteger(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteInteger(_obj, key, value)
        }
    }
    fn WriteLong(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_long /* long */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteLong(_obj, key, value)
        }
    }
    fn WriteString(_obj: *u8 /* void* */, key: *u8 /* void* */, value: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteString(_obj, key, value)
        }
    }
}
trait wxDataInputStream {
}
trait wxGridCellFloatEditor {
    fn Ctor(width: c_int /* int */, precision: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellFloatEditor_Ctor(width, precision)
        }
    }
}
trait wxServer {
}
trait wxScrollBar {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxScrollBar_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn GetPageSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollBar_GetPageSize(_obj)
        }
    }
    fn GetRange(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollBar_GetRange(_obj)
        }
    }
    fn GetThumbPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollBar_GetThumbPosition(_obj)
        }
    }
    fn GetThumbSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollBar_GetThumbSize(_obj)
        }
    }
    fn SetScrollbar(_obj: *u8 /* void* */, position: c_int /* int */, thumbSize: c_int /* int */, range: c_int /* int */, pageSize: c_int /* int */, refresh: bool /* bool */) {
        unsafe {
            wxScrollBar_SetScrollbar(_obj, position, thumbSize, range, pageSize, refresh)
        }
    }
    fn SetThumbPosition(_obj: *u8 /* void* */, viewStart: c_int /* int */) {
        unsafe {
            wxScrollBar_SetThumbPosition(_obj, viewStart)
        }
    }
}
trait wxHtmlLinkInfo {
}
trait wxSimpleHelpProvider {
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxSimpleHelpProvider_Create()
        }
    }
}
trait wxGrid {
    fn AppendCols(_obj: *u8 /* void* */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_AppendCols(_obj, numCols, updateLabels)
        }
    }
    fn AppendRows(_obj: *u8 /* void* */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_AppendRows(_obj, numRows, updateLabels)
        }
    }
    fn AutoSize(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_AutoSize(_obj)
        }
    }
    fn AutoSizeColumn(_obj: *u8 /* void* */, col: c_int /* int */, setAsMin: c_int /* int */) {
        unsafe {
            wxGrid_AutoSizeColumn(_obj, col, setAsMin)
        }
    }
    fn AutoSizeColumns(_obj: *u8 /* void* */, setAsMin: c_int /* int */) {
        unsafe {
            wxGrid_AutoSizeColumns(_obj, setAsMin)
        }
    }
    fn AutoSizeRow(_obj: *u8 /* void* */, row: c_int /* int */, setAsMin: c_int /* int */) {
        unsafe {
            wxGrid_AutoSizeRow(_obj, row, setAsMin)
        }
    }
    fn AutoSizeRows(_obj: *u8 /* void* */, setAsMin: c_int /* int */) {
        unsafe {
            wxGrid_AutoSizeRows(_obj, setAsMin)
        }
    }
    fn BeginBatch(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_BeginBatch(_obj)
        }
    }
    fn BlockToDeviceRect(_obj: *u8 /* void* */, top: c_int /* int */, left: c_int /* int */, bottom: c_int /* int */, right: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_BlockToDeviceRect(_obj, top, left, bottom, right)
        }
    }
    fn CanDragColSize(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_CanDragColSize(_obj)
        }
    }
    fn CanDragGridSize(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_CanDragGridSize(_obj)
        }
    }
    fn CanDragRowSize(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_CanDragRowSize(_obj)
        }
    }
    fn CanEnableCellControl(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_CanEnableCellControl(_obj)
        }
    }
    fn CellToRect(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_CellToRect(_obj, row, col)
        }
    }
    fn ClearGrid(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_ClearGrid(_obj)
        }
    }
    fn ClearSelection(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_ClearSelection(_obj)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn CreateGrid(_obj: *u8 /* void* */, rows: c_int /* int */, cols: c_int /* int */, selmode: c_int /* int */) {
        unsafe {
            wxGrid_CreateGrid(_obj, rows, cols, selmode)
        }
    }
    fn DeleteCols(_obj: *u8 /* void* */, pos: c_int /* int */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_DeleteCols(_obj, pos, numCols, updateLabels)
        }
    }
    fn DeleteRows(_obj: *u8 /* void* */, pos: c_int /* int */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_DeleteRows(_obj, pos, numRows, updateLabels)
        }
    }
    fn DisableCellEditControl(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_DisableCellEditControl(_obj)
        }
    }
    fn DisableDragColSize(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_DisableDragColSize(_obj)
        }
    }
    fn DisableDragGridSize(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_DisableDragGridSize(_obj)
        }
    }
    fn DisableDragRowSize(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_DisableDragRowSize(_obj)
        }
    }
    fn DrawAllGridLines(_obj: *u8 /* void* */, dc: *u8 /* void* */, reg: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawAllGridLines(_obj, dc, reg)
        }
    }
    fn DrawCell(_obj: *u8 /* void* */, dc: *u8 /* void* */, _row: c_int /* int */, _col: c_int /* int */) {
        unsafe {
            wxGrid_DrawCell(_obj, dc, _row, _col)
        }
    }
    fn DrawCellBorder(_obj: *u8 /* void* */, dc: *u8 /* void* */, _row: c_int /* int */, _col: c_int /* int */) {
        unsafe {
            wxGrid_DrawCellBorder(_obj, dc, _row, _col)
        }
    }
    fn DrawCellHighlight(_obj: *u8 /* void* */, dc: *u8 /* void* */, attr: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawCellHighlight(_obj, dc, attr)
        }
    }
    fn DrawColLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxGrid_DrawColLabel(_obj, dc, col)
        }
    }
    fn DrawColLabels(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawColLabels(_obj, dc)
        }
    }
    fn DrawGridSpace(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawGridSpace(_obj, dc)
        }
    }
    fn DrawRowLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */, row: c_int /* int */) {
        unsafe {
            wxGrid_DrawRowLabel(_obj, dc, row)
        }
    }
    fn DrawRowLabels(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawRowLabels(_obj, dc)
        }
    }
    fn DrawTextRectangle(_obj: *u8 /* void* */, dc: *u8 /* void* */, txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, horizontalAlignment: c_int /* int */, verticalAlignment: c_int /* int */) {
        unsafe {
            wxGrid_DrawTextRectangle(_obj, dc, txt, arg0, arg1, arg2, arg3, horizontalAlignment, verticalAlignment)
        }
    }
    fn EnableCellEditControl(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableCellEditControl(_obj, enable)
        }
    }
    fn EnableDragColSize(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableDragColSize(_obj, enable)
        }
    }
    fn EnableDragGridSize(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableDragGridSize(_obj, enable)
        }
    }
    fn EnableDragRowSize(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableDragRowSize(_obj, enable)
        }
    }
    fn EnableEditing(_obj: *u8 /* void* */, edit: c_int /* int */) {
        unsafe {
            wxGrid_EnableEditing(_obj, edit)
        }
    }
    fn EnableGridLines(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableGridLines(_obj, enable)
        }
    }
    fn EndBatch(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_EndBatch(_obj)
        }
    }
    fn GetBatchCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetBatchCount(_obj)
        }
    }
    fn GetCellAlignment(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetCellAlignment(_obj, row, col, arg0, arg1)
        }
    }
    fn GetCellBackgroundColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_GetCellBackgroundColour(_obj, row, col, colour)
        }
    }
    fn GetCellEditor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetCellEditor(_obj, row, col)
        }
    }
    fn GetCellFont(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, font: *u8 /* void* */) {
        unsafe {
            wxGrid_GetCellFont(_obj, row, col, font)
        }
    }
    fn GetCellHighlightColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetCellHighlightColour(_obj, _ref)
        }
    }
    fn GetCellRenderer(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetCellRenderer(_obj, row, col)
        }
    }
    fn GetCellTextColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_GetCellTextColour(_obj, row, col, colour)
        }
    }
    fn GetCellValue(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetCellValue(_obj, row, col)
        }
    }
    fn GetColLabelAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetColLabelAlignment(_obj, arg0, arg1)
        }
    }
    fn GetColLabelSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetColLabelSize(_obj)
        }
    }
    fn GetColLabelValue(_obj: *u8 /* void* */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetColLabelValue(_obj, col)
        }
    }
    fn GetColSize(_obj: *u8 /* void* */, col: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetColSize(_obj, col)
        }
    }
    fn GetDefaultCellAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetDefaultCellAlignment(_obj, arg0, arg1)
        }
    }
    fn GetDefaultCellBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetDefaultCellBackgroundColour(_obj, _ref)
        }
    }
    fn GetDefaultCellFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetDefaultCellFont(_obj, _ref)
        }
    }
    fn GetDefaultCellTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetDefaultCellTextColour(_obj, _ref)
        }
    }
    fn GetDefaultColLabelSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetDefaultColLabelSize(_obj)
        }
    }
    fn GetDefaultColSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetDefaultColSize(_obj)
        }
    }
    fn GetDefaultEditor(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultEditor(_obj)
        }
    }
    fn GetDefaultEditorForCell(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultEditorForCell(_obj, row, col)
        }
    }
    fn GetDefaultEditorForType(_obj: *u8 /* void* */, typeName: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultEditorForType(_obj, typeName)
        }
    }
    fn GetDefaultRenderer(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultRenderer(_obj)
        }
    }
    fn GetDefaultRendererForCell(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultRendererForCell(_obj, row, col)
        }
    }
    fn GetDefaultRendererForType(_obj: *u8 /* void* */, typeName: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultRendererForType(_obj, typeName)
        }
    }
    fn GetDefaultRowLabelSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetDefaultRowLabelSize(_obj)
        }
    }
    fn GetDefaultRowSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetDefaultRowSize(_obj)
        }
    }
    fn GetGridCursorCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetGridCursorCol(_obj)
        }
    }
    fn GetGridCursorRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetGridCursorRow(_obj)
        }
    }
    fn GetGridLineColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetGridLineColour(_obj, _ref)
        }
    }
    fn GetLabelBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetLabelBackgroundColour(_obj, _ref)
        }
    }
    fn GetLabelFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetLabelFont(_obj, _ref)
        }
    }
    fn GetLabelTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetLabelTextColour(_obj, _ref)
        }
    }
    fn GetNumberCols(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetNumberCols(_obj)
        }
    }
    fn GetNumberRows(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetNumberRows(_obj)
        }
    }
    fn GetRowLabelAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetRowLabelAlignment(_obj, arg0, arg1)
        }
    }
    fn GetRowLabelSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetRowLabelSize(_obj)
        }
    }
    fn GetRowLabelValue(_obj: *u8 /* void* */, row: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetRowLabelValue(_obj, row)
        }
    }
    fn GetRowSize(_obj: *u8 /* void* */, row: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetRowSize(_obj, row)
        }
    }
    fn GetSelectionBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectionBackground(_obj, _ref)
        }
    }
    fn GetSelectionForeground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectionForeground(_obj, _ref)
        }
    }
    fn GetTable(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetTable(_obj)
        }
    }
    fn GetTextBoxSize(_obj: *u8 /* void* */, dc: *u8 /* void* */, arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxGrid_GetTextBoxSize(_obj, dc, arg0, arg1, arg2, arg3)
        }
    }
    fn GridLinesEnabled(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GridLinesEnabled(_obj)
        }
    }
    fn HideCellEditControl(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_HideCellEditControl(_obj)
        }
    }
    fn InsertCols(_obj: *u8 /* void* */, pos: c_int /* int */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_InsertCols(_obj, pos, numCols, updateLabels)
        }
    }
    fn InsertRows(_obj: *u8 /* void* */, pos: c_int /* int */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_InsertRows(_obj, pos, numRows, updateLabels)
        }
    }
    fn IsCellEditControlEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsCellEditControlEnabled(_obj)
        }
    }
    fn IsCellEditControlShown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsCellEditControlShown(_obj)
        }
    }
    fn IsCurrentCellReadOnly(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsCurrentCellReadOnly(_obj)
        }
    }
    fn IsEditable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsEditable(_obj)
        }
    }
    fn IsInSelection(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsInSelection(_obj, row, col)
        }
    }
    fn IsReadOnly(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsReadOnly(_obj, row, col)
        }
    }
    fn IsSelection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsSelection(_obj)
        }
    }
    fn IsVisible(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, wholeCellVisible: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsVisible(_obj, row, col, wholeCellVisible)
        }
    }
    fn MakeCellVisible(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) {
        unsafe {
            wxGrid_MakeCellVisible(_obj, row, col)
        }
    }
    fn MoveCursorDown(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorDown(_obj, expandSelection)
        }
    }
    fn MoveCursorDownBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorDownBlock(_obj, expandSelection)
        }
    }
    fn MoveCursorLeft(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorLeft(_obj, expandSelection)
        }
    }
    fn MoveCursorLeftBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorLeftBlock(_obj, expandSelection)
        }
    }
    fn MoveCursorRight(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorRight(_obj, expandSelection)
        }
    }
    fn MoveCursorRightBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorRightBlock(_obj, expandSelection)
        }
    }
    fn MoveCursorUp(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorUp(_obj, expandSelection)
        }
    }
    fn MoveCursorUpBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorUpBlock(_obj, expandSelection)
        }
    }
    fn MovePageDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_MovePageDown(_obj)
        }
    }
    fn MovePageUp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_MovePageUp(_obj)
        }
    }
    fn ProcessTableMessage(_obj: *u8 /* void* */, evt: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_ProcessTableMessage(_obj, evt)
        }
    }
    fn RegisterDataType(_obj: *u8 /* void* */, typeName: *u8 /* void* */, renderer: *u8 /* void* */, editor: *u8 /* void* */) {
        unsafe {
            wxGrid_RegisterDataType(_obj, typeName, renderer, editor)
        }
    }
    fn SaveEditControlValue(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_SaveEditControlValue(_obj)
        }
    }
    fn SelectAll(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_SelectAll(_obj)
        }
    }
    fn SelectBlock(_obj: *u8 /* void* */, topRow: c_int /* int */, leftCol: c_int /* int */, bottomRow: c_int /* int */, rightCol: c_int /* int */, addToSelected: c_int /* int */) {
        unsafe {
            wxGrid_SelectBlock(_obj, topRow, leftCol, bottomRow, rightCol, addToSelected)
        }
    }
    fn SelectCol(_obj: *u8 /* void* */, col: c_int /* int */, addToSelected: c_int /* int */) {
        unsafe {
            wxGrid_SelectCol(_obj, col, addToSelected)
        }
    }
    fn SelectRow(_obj: *u8 /* void* */, row: c_int /* int */, addToSelected: c_int /* int */) {
        unsafe {
            wxGrid_SelectRow(_obj, row, addToSelected)
        }
    }
    fn SetCellAlignment(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, horiz: c_int /* int */, vert: c_int /* int */) {
        unsafe {
            wxGrid_SetCellAlignment(_obj, row, col, horiz, vert)
        }
    }
    fn SetCellBackgroundColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellBackgroundColour(_obj, row, col, colour)
        }
    }
    fn SetCellEditor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, editor: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellEditor(_obj, row, col, editor)
        }
    }
    fn SetCellFont(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, font: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellFont(_obj, row, col, font)
        }
    }
    fn SetCellHighlightColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellHighlightColour(_obj, col)
        }
    }
    fn SetCellRenderer(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, renderer: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellRenderer(_obj, row, col, renderer)
        }
    }
    fn SetCellTextColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellTextColour(_obj, row, col, colour)
        }
    }
    fn SetCellValue(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, s: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellValue(_obj, row, col, s)
        }
    }
    fn SetColAttr(_obj: *u8 /* void* */, col: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxGrid_SetColAttr(_obj, col, attr)
        }
    }
    fn SetColFormatBool(_obj: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxGrid_SetColFormatBool(_obj, col)
        }
    }
    fn SetColFormatCustom(_obj: *u8 /* void* */, col: c_int /* int */, typeName: *u8 /* void* */) {
        unsafe {
            wxGrid_SetColFormatCustom(_obj, col, typeName)
        }
    }
    fn SetColFormatFloat(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */, precision: c_int /* int */) {
        unsafe {
            wxGrid_SetColFormatFloat(_obj, col, width, precision)
        }
    }
    fn SetColFormatNumber(_obj: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxGrid_SetColFormatNumber(_obj, col)
        }
    }
    fn SetColLabelAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */) {
        unsafe {
            wxGrid_SetColLabelAlignment(_obj, horiz, vert)
        }
    }
    fn SetColLabelSize(_obj: *u8 /* void* */, height: c_int /* int */) {
        unsafe {
            wxGrid_SetColLabelSize(_obj, height)
        }
    }
    fn SetColLabelValue(_obj: *u8 /* void* */, col: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxGrid_SetColLabelValue(_obj, col, label)
        }
    }
    fn SetColMinimalWidth(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */) {
        unsafe {
            wxGrid_SetColMinimalWidth(_obj, col, width)
        }
    }
    fn SetColSize(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */) {
        unsafe {
            wxGrid_SetColSize(_obj, col, width)
        }
    }
    fn SetDefaultCellAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */) {
        unsafe {
            wxGrid_SetDefaultCellAlignment(_obj, horiz, vert)
        }
    }
    fn SetDefaultCellBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultCellBackgroundColour(_obj, colour)
        }
    }
    fn SetDefaultCellFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultCellFont(_obj, font)
        }
    }
    fn SetDefaultCellTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultCellTextColour(_obj, colour)
        }
    }
    fn SetDefaultColSize(_obj: *u8 /* void* */, width: c_int /* int */, resizeExistingCols: c_int /* int */) {
        unsafe {
            wxGrid_SetDefaultColSize(_obj, width, resizeExistingCols)
        }
    }
    fn SetDefaultEditor(_obj: *u8 /* void* */, editor: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultEditor(_obj, editor)
        }
    }
    fn SetDefaultRenderer(_obj: *u8 /* void* */, renderer: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultRenderer(_obj, renderer)
        }
    }
    fn SetDefaultRowSize(_obj: *u8 /* void* */, height: c_int /* int */, resizeExistingRows: c_int /* int */) {
        unsafe {
            wxGrid_SetDefaultRowSize(_obj, height, resizeExistingRows)
        }
    }
    fn SetGridCursor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) {
        unsafe {
            wxGrid_SetGridCursor(_obj, row, col)
        }
    }
    fn SetGridLineColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxGrid_SetGridLineColour(_obj, col)
        }
    }
    fn SetLabelBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetLabelBackgroundColour(_obj, colour)
        }
    }
    fn SetLabelFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxGrid_SetLabelFont(_obj, font)
        }
    }
    fn SetLabelTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetLabelTextColour(_obj, colour)
        }
    }
    fn SetMargins(_obj: *u8 /* void* */, extraWidth: c_int /* int */, extraHeight: c_int /* int */) {
        unsafe {
            wxGrid_SetMargins(_obj, extraWidth, extraHeight)
        }
    }
    fn SetReadOnly(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, isReadOnly: bool /* bool */) {
        unsafe {
            wxGrid_SetReadOnly(_obj, row, col, isReadOnly)
        }
    }
    fn SetRowAttr(_obj: *u8 /* void* */, row: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxGrid_SetRowAttr(_obj, row, attr)
        }
    }
    fn SetRowLabelAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */) {
        unsafe {
            wxGrid_SetRowLabelAlignment(_obj, horiz, vert)
        }
    }
    fn SetRowLabelSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxGrid_SetRowLabelSize(_obj, width)
        }
    }
    fn SetRowLabelValue(_obj: *u8 /* void* */, row: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxGrid_SetRowLabelValue(_obj, row, label)
        }
    }
    fn SetRowMinimalHeight(_obj: *u8 /* void* */, row: c_int /* int */, width: c_int /* int */) {
        unsafe {
            wxGrid_SetRowMinimalHeight(_obj, row, width)
        }
    }
    fn SetRowSize(_obj: *u8 /* void* */, row: c_int /* int */, height: c_int /* int */) {
        unsafe {
            wxGrid_SetRowSize(_obj, row, height)
        }
    }
    fn SetSelectionBackground(_obj: *u8 /* void* */, c: *u8 /* void* */) {
        unsafe {
            wxGrid_SetSelectionBackground(_obj, c)
        }
    }
    fn SetSelectionForeground(_obj: *u8 /* void* */, c: *u8 /* void* */) {
        unsafe {
            wxGrid_SetSelectionForeground(_obj, c)
        }
    }
    fn SetSelectionMode(_obj: *u8 /* void* */, selmode: c_int /* int */) {
        unsafe {
            wxGrid_SetSelectionMode(_obj, selmode)
        }
    }
    fn SetTable(_obj: *u8 /* void* */, table: *u8 /* void* */, takeOwnership: bool /* bool */, selmode: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxGrid_SetTable(_obj, table, takeOwnership, selmode)
        }
    }
    fn ShowCellEditControl(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_ShowCellEditControl(_obj)
        }
    }
    fn StringToLines(_obj: *u8 /* void* */, value: *u8 /* void* */, lines: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_StringToLines(_obj, value, lines)
        }
    }
    fn XToCol(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_XToCol(_obj, x)
        }
    }
    fn XToEdgeOfCol(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_XToEdgeOfCol(_obj, x)
        }
    }
    fn XYToCell(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxGrid_XYToCell(_obj, arg0, arg1, arg2, arg3)
        }
    }
    fn YToEdgeOfRow(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_YToEdgeOfRow(_obj, y)
        }
    }
    fn YToRow(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_YToRow(_obj, y)
        }
    }
    fn GetSelectedCells(_obj: *u8 /* void* */, _arr: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectedCells(_obj, _arr)
        }
    }
    fn GetSelectionBlockTopLeft(_obj: *u8 /* void* */, _arr: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectionBlockTopLeft(_obj, _arr)
        }
    }
    fn GetSelectionBlockBottomRight(_obj: *u8 /* void* */, _arr: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectionBlockBottomRight(_obj, _arr)
        }
    }
    fn GetSelectedRows(_obj: *u8 /* void* */, _arr: *intptr_t /* intptr_t* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetSelectedRows(_obj, _arr)
        }
    }
    fn GetSelectedCols(_obj: *u8 /* void* */, _arr: *intptr_t /* intptr_t* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetSelectedCols(_obj, _arr)
        }
    }
    fn GetCellSize(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetCellSize(_obj, row, col, arg0, arg1)
        }
    }
    fn SetCellSize(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxGrid_SetCellSize(_obj, row, col, arg0, arg1)
        }
    }
}
trait wxHtmlFilter {
}
trait cbDimHandlerBase {
}
trait cbBarInfo {
    // missing: cbBarInfo_Create
    // missing: cbBarInfo_Delete
    // missing: cbBarInfo_IsExpanded
    // missing: cbBarInfo_IsFixed
}
trait wxPropertyGrid {
    fn Append(_obj: *u8 /* void* */, prop: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPropertyGrid_Append(_obj, prop)
        }
    }
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPropertyGrid_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn DisableProperty(_obj: *u8 /* void* */, propName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPropertyGrid_DisableProperty(_obj, propName)
        }
    }
}
trait wxGridCellBoolEditor {
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellBoolEditor_Ctor()
        }
    }
}
trait wxCalendarCtrl {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _dat: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCalendarCtrl_Create(_prt, _id, _dat, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn EnableHolidayDisplay(_obj: *u8 /* void* */, display: c_int /* int */) {
        unsafe {
            wxCalendarCtrl_EnableHolidayDisplay(_obj, display)
        }
    }
    fn EnableMonthChange(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxCalendarCtrl_EnableMonthChange(_obj, enable)
        }
    }
    fn GetAttr(_obj: *u8 /* void* */, day: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCalendarCtrl_GetAttr(_obj, day)
        }
    }
    fn GetDate(_obj: *u8 /* void* */, date: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetDate(_obj, date)
        }
    }
    fn GetHeaderColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHeaderColourBg(_obj, _ref)
        }
    }
    fn GetHeaderColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHeaderColourFg(_obj, _ref)
        }
    }
    fn GetHighlightColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHighlightColourBg(_obj, _ref)
        }
    }
    fn GetHighlightColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHighlightColourFg(_obj, _ref)
        }
    }
    fn GetHolidayColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHolidayColourBg(_obj, _ref)
        }
    }
    fn GetHolidayColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHolidayColourFg(_obj, _ref)
        }
    }
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, date: *u8 /* void* */, wd: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCalendarCtrl_HitTest(_obj, arg0, arg1, date, wd)
        }
    }
    fn ResetAttr(_obj: *u8 /* void* */, day: c_int /* int */) {
        unsafe {
            wxCalendarCtrl_ResetAttr(_obj, day)
        }
    }
    fn SetAttr(_obj: *u8 /* void* */, day: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetAttr(_obj, day, attr)
        }
    }
    fn SetDate(_obj: *u8 /* void* */, date: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetDate(_obj, date)
        }
    }
    fn SetHeaderColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetHeaderColours(_obj, colFg, colBg)
        }
    }
    fn SetHighlightColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetHighlightColours(_obj, colFg, colBg)
        }
    }
    fn SetHoliday(_obj: *u8 /* void* */, day: c_int /* int */) {
        unsafe {
            wxCalendarCtrl_SetHoliday(_obj, day)
        }
    }
    fn SetHolidayColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetHolidayColours(_obj, colFg, colBg)
        }
    }
}
trait wxSingleInstanceChecker {
    fn Create(_obj: *u8 /* void* */, name: *u8 /* void* */, path: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSingleInstanceChecker_Create(_obj, name, path)
        }
    }
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxSingleInstanceChecker_CreateDefault()
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxSingleInstanceChecker_Delete(_obj)
        }
    }
    fn IsAnotherRunning(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSingleInstanceChecker_IsAnotherRunning(_obj)
        }
    }
}
trait wxTreeEvent {
    fn GetCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeEvent_GetCode(_obj)
        }
    }
    fn GetItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxTreeEvent_GetItem(_obj, _ref)
        }
    }
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeEvent_GetLabel(_obj)
        }
    }
    fn GetOldItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxTreeEvent_GetOldItem(_obj, _ref)
        }
    }
    fn GetPoint(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeEvent_GetPoint(_obj)
        }
    }
}
trait wxClassInfo {
    fn CreateClassByName(_inf: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_CreateClassByName(_inf)
        }
    }
    fn GetClassName(_inf: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_GetClassName(_inf)
        }
    }
    fn IsKindOf(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClassInfo_IsKindOf(_obj, _name)
        }
    }
    fn FindClass(_txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_FindClass(_txt)
        }
    }
    fn GetBaseClassName1(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_GetBaseClassName1(_obj)
        }
    }
    fn GetBaseClassName2(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_GetBaseClassName2(_obj)
        }
    }
    fn GetClassNameEx(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_GetClassNameEx(_obj)
        }
    }
    fn GetSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxClassInfo_GetSize(_obj)
        }
    }
    fn IsKindOfEx(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClassInfo_IsKindOfEx(_obj, classInfo)
        }
    }
    fn wxNotebook_AssignImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxNotebook_AssignImageList(_obj, imageList)
        }
    }
}
trait wxFSFile {
}
trait wxFileSystemHandler {
}
trait wxStreamBuffer {
}
trait wxDragImage {
}
trait wxCheckBox {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCheckBox_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCheckBox_Delete(_obj)
        }
    }
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCheckBox_GetValue(_obj)
        }
    }
    fn SetValue(_obj: *u8 /* void* */, value: c_int /* int */) {
        unsafe {
            wxCheckBox_SetValue(_obj, value)
        }
    }
}
trait cbLayoutRowEvent {
    // missing: cbLayoutRowEvent_Row
}
trait wxTextEntryDialog {
}
trait wxBoxSizer {
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBoxSizer_CalcMin(_obj)
        }
    }
    fn Create(orient: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBoxSizer_Create(orient)
        }
    }
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBoxSizer_GetOrientation(_obj)
        }
    }
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxBoxSizer_RecalcSizes(_obj)
        }
    }
}
trait wxSetCursorEvent {
    fn GetCursor(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSetCursorEvent_GetCursor(_obj)
        }
    }
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSetCursorEvent_GetX(_obj)
        }
    }
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSetCursorEvent_GetY(_obj)
        }
    }
    fn HasCursor(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSetCursorEvent_HasCursor(_obj)
        }
    }
    fn SetCursor(_obj: *u8 /* void* */, cursor: *u8 /* void* */) {
        unsafe {
            wxSetCursorEvent_SetCursor(_obj, cursor)
        }
    }
}
trait cbDrawPaneDecorEvent {
    // missing: cbDrawPaneDecorEvent_Dc
}
trait wxFileName {
}
