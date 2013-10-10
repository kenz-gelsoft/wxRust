use std::libc::*;
use native::*;

// skipping globals...

trait wxMultiCellItemHandle {
    #[fixed_stack_segment]
    fn Create(row: c_int /* int */, column: c_int /* int */, height: c_int /* int */, width: c_int /* int */, sx: c_int /* int */, sy: c_int /* int */, style: c_int /* int */, wx: c_int /* int */, wy: c_int /* int */, align: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMultiCellItemHandle_Create(row, column, height, width, sx, sy, style, wx, wy, align)
        }
    }
    #[fixed_stack_segment]
    fn CreateWithSize(_obj: *u8 /* void* */, row: c_int /* int */, column: c_int /* int */, sx: c_int /* int */, sy: c_int /* int */, style: c_int /* int */, wx: c_int /* int */, wy: c_int /* int */, align: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMultiCellItemHandle_CreateWithSize(_obj, row, column, sx, sy, style, wx, wy, align)
        }
    }
    #[fixed_stack_segment]
    fn CreateWithStyle(_obj: *u8 /* void* */, row: c_int /* int */, column: c_int /* int */, style: c_int /* int */, wx: c_int /* int */, wy: c_int /* int */, align: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMultiCellItemHandle_CreateWithStyle(_obj, row, column, style, wx, wy, align)
        }
    }
    #[fixed_stack_segment]
    fn GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellItemHandle_GetAlignment(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetColumn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellItemHandle_GetColumn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellItemHandle_GetHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLocalSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxMultiCellItemHandle_GetLocalSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellItemHandle_GetRow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellItemHandle_GetStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWeight(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxMultiCellItemHandle_GetWeight(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellItemHandle_GetWidth(_obj)
        }
    }
}
trait wxKeyEvent {
    #[fixed_stack_segment]
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_AltDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_ControlDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxKeyEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetKeyCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxKeyEvent_GetKeyCode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxKeyEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxKeyEvent_GetX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxKeyEvent_GetY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetModifiers(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxKeyEvent_GetModifiers(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasModifiers(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_HasModifiers(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_MetaDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetKeyCode(_obj: *u8 /* void* */, code: c_int /* int */) {
        unsafe {
            wxKeyEvent_SetKeyCode(_obj, code)
        }
    }
    #[fixed_stack_segment]
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxKeyEvent_ShiftDown(_obj)
        }
    }
}
trait wxWindow {
    #[fixed_stack_segment]
    fn AddChild(_obj: *u8 /* void* */, child: *u8 /* void* */) {
        unsafe {
            wxWindow_AddChild(_obj, child)
        }
    }
    #[fixed_stack_segment]
    fn AddConstraintReference(_obj: *u8 /* void* */, otherWin: *u8 /* void* */) {
        unsafe {
            wxWindow_AddConstraintReference(_obj, otherWin)
        }
    }
    #[fixed_stack_segment]
    fn CaptureMouse(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_CaptureMouse(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Center(_obj: *u8 /* void* */, direction: c_int /* int */) {
        unsafe {
            wxWindow_Center(_obj, direction)
        }
    }
    #[fixed_stack_segment]
    fn CenterOnParent(_obj: *u8 /* void* */, dir: c_int /* int */) {
        unsafe {
            wxWindow_CenterOnParent(_obj, dir)
        }
    }
    #[fixed_stack_segment]
    fn ClearBackground(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_ClearBackground(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ClientToScreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ClientToScreen(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Close(_obj: *u8 /* void* */, _force: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxWindow_Close(_obj, _force)
        }
    }
    #[fixed_stack_segment]
    fn ConvertDialogToPixels(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ConvertDialogToPixels(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ConvertPixelsToDialog(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ConvertPixelsToDialog(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn DeleteRelatedConstraints(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_DeleteRelatedConstraints(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Destroy(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Destroy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DestroyChildren(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_DestroyChildren(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Disable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Disable(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DoPhase(_obj: *u8 /* void* */, phase: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_DoPhase(_obj, phase)
        }
    }
    #[fixed_stack_segment]
    fn Enable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Enable(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FindFocus(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_FindFocus(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FindWindow(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_FindWindow(_obj, name)
        }
    }
    #[fixed_stack_segment]
    fn Fit(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Fit(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FitInside(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_FitInside(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Freeze(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Freeze(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEffectiveMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetEffectiveMinSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetAutoLayout(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetAutoLayout(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxWindow_GetBackgroundColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetBestSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetBestSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCaret(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetCaret(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCharHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetCharHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCharWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetCharWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetChildren(_obj: *u8 /* void* */, _res: *u8 /* void* */, _cnt: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetChildren(_obj, _res, _cnt)
        }
    }
    #[fixed_stack_segment]
    fn GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetClientData(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetClientSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetClientSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetClientSizeConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxWindow_GetClientSizeConstraint(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetConstraints(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetConstraints(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetConstraintsInvolvedIn(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetConstraintsInvolvedIn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCursor(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetCursor(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDropTarget(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetDropTarget(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEventHandler(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetEventHandler(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxWindow_GetFont(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetForegroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxWindow_GetForegroundColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetHandle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetHandle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetLabel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLabelEmpty(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetLabelEmpty(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaxHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetMaxHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaxWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetMaxWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetMinHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetMinWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetName(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetParent(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetParent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPositionConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxWindow_GetPositionConstraint(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetRect(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetScrollPos(_obj, orient)
        }
    }
    #[fixed_stack_segment]
    fn GetScrollRange(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetScrollRange(_obj, orient)
        }
    }
    #[fixed_stack_segment]
    fn GetScrollThumb(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetScrollThumb(_obj, orient)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSizeConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxWindow_GetSizeConstraint(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetSizer(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetSizer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTextExtent(_obj: *u8 /* void* */, string: *u8 /* void* */, x: *c_int /* int* */, y: *c_int /* int* */, descent: *c_int /* int* */, externalLeading: *c_int /* int* */, theFont: *u8 /* void* */) {
        unsafe {
            wxWindow_GetTextExtent(_obj, string, x, y, descent, externalLeading, theFont)
        }
    }
    #[fixed_stack_segment]
    fn GetToolTip(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetToolTip(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetUpdateRegion(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetUpdateRegion(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValidator(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetValidator(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetVirtualSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_GetVirtualSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWindowStyleFlag(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_GetWindowStyleFlag(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasFlag(_obj: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxWindow_HasFlag(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn Hide(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Hide(_obj)
        }
    }
    #[fixed_stack_segment]
    fn InitDialog(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_InitDialog(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsBeingDeleted(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsBeingDeleted(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsEnabled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsExposed(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsExposed(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn IsShown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsShown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsTopLevel(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_IsTopLevel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Layout(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_Layout(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LayoutPhase1(_obj: *u8 /* void* */, noChanges: *c_int /* int* */) -> c_int /* int */ {
        unsafe {
            wxWindow_LayoutPhase1(_obj, noChanges)
        }
    }
    #[fixed_stack_segment]
    fn LayoutPhase2(_obj: *u8 /* void* */, noChanges: *c_int /* int* */) -> c_int /* int */ {
        unsafe {
            wxWindow_LayoutPhase2(_obj, noChanges)
        }
    }
    #[fixed_stack_segment]
    fn Lower(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Lower(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MakeModal(_obj: *u8 /* void* */, modal: bool /* bool */) {
        unsafe {
            wxWindow_MakeModal(_obj, modal)
        }
    }
    #[fixed_stack_segment]
    fn Move(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_Move(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn MoveConstraint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_MoveConstraint(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn PopEventHandler(_obj: *u8 /* void* */, deleteHandler: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_PopEventHandler(_obj, deleteHandler)
        }
    }
    #[fixed_stack_segment]
    fn PopupMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxWindow_PopupMenu(_obj, menu, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxWindow_PrepareDC(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn PushEventHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxWindow_PushEventHandler(_obj, handler)
        }
    }
    #[fixed_stack_segment]
    fn Raise(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Raise(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Refresh(_obj: *u8 /* void* */, eraseBackground: bool /* bool */) {
        unsafe {
            wxWindow_Refresh(_obj, eraseBackground)
        }
    }
    #[fixed_stack_segment]
    fn RefreshRect(_obj: *u8 /* void* */, eraseBackground: bool /* bool */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxWindow_RefreshRect(_obj, eraseBackground, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn ReleaseMouse(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_ReleaseMouse(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RemoveChild(_obj: *u8 /* void* */, child: *u8 /* void* */) {
        unsafe {
            wxWindow_RemoveChild(_obj, child)
        }
    }
    #[fixed_stack_segment]
    fn RemoveConstraintReference(_obj: *u8 /* void* */, otherWin: *u8 /* void* */) {
        unsafe {
            wxWindow_RemoveConstraintReference(_obj, otherWin)
        }
    }
    #[fixed_stack_segment]
    fn Reparent(_obj: *u8 /* void* */, _par: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_Reparent(_obj, _par)
        }
    }
    #[fixed_stack_segment]
    fn ResetConstraints(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_ResetConstraints(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ScreenToClient(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ScreenToClient(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn ScrollWindow(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_ScrollWindow(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn ScrollWindowRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: c_int /* int */) {
        unsafe {
            wxWindow_ScrollWindowRect(_obj, arg0, arg1, arg2, arg3, arg4, arg5)
        }
    }
    #[fixed_stack_segment]
    fn SetAcceleratorTable(_obj: *u8 /* void* */, accel: *u8 /* void* */) {
        unsafe {
            wxWindow_SetAcceleratorTable(_obj, accel)
        }
    }
    #[fixed_stack_segment]
    fn SetAutoLayout(_obj: *u8 /* void* */, autoLayout: bool /* bool */) {
        unsafe {
            wxWindow_SetAutoLayout(_obj, autoLayout)
        }
    }
    #[fixed_stack_segment]
    fn SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_SetBackgroundColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetCaret(_obj: *u8 /* void* */, caret: *u8 /* void* */) {
        unsafe {
            wxWindow_SetCaret(_obj, caret)
        }
    }
    #[fixed_stack_segment]
    fn SetClientData(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxWindow_SetClientData(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn SetClientObject(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxWindow_SetClientObject(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn SetClientSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_SetClientSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetConstraintSizes(_obj: *u8 /* void* */, recurse: c_int /* int */) {
        unsafe {
            wxWindow_SetConstraintSizes(_obj, recurse)
        }
    }
    #[fixed_stack_segment]
    fn SetConstraints(_obj: *u8 /* void* */, constraints: *u8 /* void* */) {
        unsafe {
            wxWindow_SetConstraints(_obj, constraints)
        }
    }
    #[fixed_stack_segment]
    fn SetCursor(_obj: *u8 /* void* */, cursor: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_SetCursor(_obj, cursor)
        }
    }
    #[fixed_stack_segment]
    fn SetDropTarget(_obj: *u8 /* void* */, dropTarget: *u8 /* void* */) {
        unsafe {
            wxWindow_SetDropTarget(_obj, dropTarget)
        }
    }
    #[fixed_stack_segment]
    fn SetExtraStyle(_obj: *u8 /* void* */, exStyle: c_long /* long */) {
        unsafe {
            wxWindow_SetExtraStyle(_obj, exStyle)
        }
    }
    #[fixed_stack_segment]
    fn SetFocus(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_SetFocus(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_SetFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn SetForegroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWindow_SetForegroundColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetId(_obj: *u8 /* void* */, _id: c_int /* int */) {
        unsafe {
            wxWindow_SetId(_obj, _id)
        }
    }
    #[fixed_stack_segment]
    fn SetLabel(_obj: *u8 /* void* */, _title: *u8 /* void* */) {
        unsafe {
            wxWindow_SetLabel(_obj, _title)
        }
    }
    #[fixed_stack_segment]
    fn SetName(_obj: *u8 /* void* */, _name: *u8 /* void* */) {
        unsafe {
            wxWindow_SetName(_obj, _name)
        }
    }
    #[fixed_stack_segment]
    fn SetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */, pos: c_int /* int */, refresh: bool /* bool */) {
        unsafe {
            wxWindow_SetScrollPos(_obj, orient, pos, refresh)
        }
    }
    #[fixed_stack_segment]
    fn SetScrollbar(_obj: *u8 /* void* */, orient: c_int /* int */, pos: c_int /* int */, thumbVisible: c_int /* int */, range: c_int /* int */, refresh: bool /* bool */) {
        unsafe {
            wxWindow_SetScrollbar(_obj, orient, pos, thumbVisible, range, refresh)
        }
    }
    #[fixed_stack_segment]
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, sizeFlags: c_int /* int */) {
        unsafe {
            wxWindow_SetSize(_obj, arg0, arg1, arg2, arg3, sizeFlags)
        }
    }
    #[fixed_stack_segment]
    fn SetSizeConstraint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxWindow_SetSizeConstraint(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn SetSizeHints(_obj: *u8 /* void* */, minW: c_int /* int */, minH: c_int /* int */, maxW: c_int /* int */, maxH: c_int /* int */, incW: c_int /* int */, incH: c_int /* int */) {
        unsafe {
            wxWindow_SetSizeHints(_obj, minW, minH, maxW, maxH, incW, incH)
        }
    }
    #[fixed_stack_segment]
    fn SetSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) {
        unsafe {
            wxWindow_SetSizer(_obj, sizer)
        }
    }
    #[fixed_stack_segment]
    fn SetToolTip(_obj: *u8 /* void* */, tip: *u8 /* void* */) {
        unsafe {
            wxWindow_SetToolTip(_obj, tip)
        }
    }
    #[fixed_stack_segment]
    fn SetValidator(_obj: *u8 /* void* */, validator: *u8 /* void* */) {
        unsafe {
            wxWindow_SetValidator(_obj, validator)
        }
    }
    #[fixed_stack_segment]
    fn SetWindowStyleFlag(_obj: *u8 /* void* */, style: c_long /* long */) {
        unsafe {
            wxWindow_SetWindowStyleFlag(_obj, style)
        }
    }
    #[fixed_stack_segment]
    fn Show(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Show(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Thaw(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_Thaw(_obj)
        }
    }
    #[fixed_stack_segment]
    fn TransferDataFromWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_TransferDataFromWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn TransferDataToWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_TransferDataToWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn UnsetConstraints(_obj: *u8 /* void* */, c: *u8 /* void* */) {
        unsafe {
            wxWindow_UnsetConstraints(_obj, c)
        }
    }
    #[fixed_stack_segment]
    fn UpdateWindowUI(_obj: *u8 /* void* */) {
        unsafe {
            wxWindow_UpdateWindowUI(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Validate(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxWindow_Validate(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetVirtualSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_SetVirtualSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn WarpPointer(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWindow_WarpPointer(_obj, arg0, arg1)
        }
    }
}
trait wxCustomDataObject {
}
trait wxSashWindow {
    #[fixed_stack_segment]
    fn Create(_par: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSashWindow_Create(_par, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultBorderSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetDefaultBorderSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEdgeMargin(_obj: *u8 /* void* */, edge: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetEdgeMargin(_obj, edge)
        }
    }
    #[fixed_stack_segment]
    fn GetExtraBorderSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetExtraBorderSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaximumSizeX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetMaximumSizeX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaximumSizeY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetMaximumSizeY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinimumSizeX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetMinimumSizeX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinimumSizeY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashWindow_GetMinimumSizeY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSashVisible(_obj: *u8 /* void* */, edge: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSashWindow_GetSashVisible(_obj, edge)
        }
    }
    #[fixed_stack_segment]
    fn HasBorder(_obj: *u8 /* void* */, edge: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSashWindow_HasBorder(_obj, edge)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultBorderSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxSashWindow_SetDefaultBorderSize(_obj, width)
        }
    }
    #[fixed_stack_segment]
    fn SetExtraBorderSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxSashWindow_SetExtraBorderSize(_obj, width)
        }
    }
    #[fixed_stack_segment]
    fn SetMaximumSizeX(_obj: *u8 /* void* */, max: c_int /* int */) {
        unsafe {
            wxSashWindow_SetMaximumSizeX(_obj, max)
        }
    }
    #[fixed_stack_segment]
    fn SetMaximumSizeY(_obj: *u8 /* void* */, max: c_int /* int */) {
        unsafe {
            wxSashWindow_SetMaximumSizeY(_obj, max)
        }
    }
    #[fixed_stack_segment]
    fn SetMinimumSizeX(_obj: *u8 /* void* */, min: c_int /* int */) {
        unsafe {
            wxSashWindow_SetMinimumSizeX(_obj, min)
        }
    }
    #[fixed_stack_segment]
    fn SetMinimumSizeY(_obj: *u8 /* void* */, min: c_int /* int */) {
        unsafe {
            wxSashWindow_SetMinimumSizeY(_obj, min)
        }
    }
    #[fixed_stack_segment]
    fn SetSashBorder(_obj: *u8 /* void* */, edge: c_int /* int */, border: bool /* bool */) {
        unsafe {
            wxSashWindow_SetSashBorder(_obj, edge, border)
        }
    }
    #[fixed_stack_segment]
    fn SetSashVisible(_obj: *u8 /* void* */, edge: c_int /* int */, sash: bool /* bool */) {
        unsafe {
            wxSashWindow_SetSashVisible(_obj, edge, sash)
        }
    }
}
trait wxRealPoint {
}
trait wxSystemSettings {
    #[fixed_stack_segment]
    fn GetColour(index: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxSystemSettings_GetColour(index, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetFont(index: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxSystemSettings_GetFont(index, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetMetric(index: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxSystemSettings_GetMetric(index)
        }
    }
    #[fixed_stack_segment]
    fn GetScreenType() -> c_int /* int */ {
        unsafe {
            wxSystemSettings_GetScreenType()
        }
    }
}
trait cbBarSpy {
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbBarSpy_Create(pPanel)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbBarSpy_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbBarSpy_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ProcessEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbBarSpy_ProcessEvent(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn SetBarWindow(_obj: *u8 /* void* */, pWnd: *u8 /* void* */) {
        unsafe {
            cbBarSpy_SetBarWindow(_obj, pWnd)
        }
    }
}
trait wxNotebookEvent {
}
trait wxJoystickEvent {
    #[fixed_stack_segment]
    fn ButtonDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_ButtonDown(_obj, but)
        }
    }
    #[fixed_stack_segment]
    fn ButtonIsDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_ButtonIsDown(_obj, but)
        }
    }
    #[fixed_stack_segment]
    fn ButtonUp(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_ButtonUp(_obj, but)
        }
    }
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxJoystickEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetButtonChange(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystickEvent_GetButtonChange(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetButtonState(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystickEvent_GetButtonState(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetJoystick(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystickEvent_GetJoystick(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxJoystickEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetZPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystickEvent_GetZPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsButton(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_IsButton(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsMove(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_IsMove(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsZMove(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystickEvent_IsZMove(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetButtonChange(_obj: *u8 /* void* */, change: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetButtonChange(_obj, change)
        }
    }
    #[fixed_stack_segment]
    fn SetButtonState(_obj: *u8 /* void* */, state: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetButtonState(_obj, state)
        }
    }
    #[fixed_stack_segment]
    fn SetJoystick(_obj: *u8 /* void* */, stick: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetJoystick(_obj, stick)
        }
    }
    #[fixed_stack_segment]
    fn SetPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetPosition(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetZPosition(_obj: *u8 /* void* */, zPos: c_int /* int */) {
        unsafe {
            wxJoystickEvent_SetZPosition(_obj, zPos)
        }
    }
}
trait wxMBConvUTF7 {
}
trait wxScrollWinEvent {
    #[fixed_stack_segment]
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollWinEvent_GetOrientation(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollWinEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */) {
        unsafe {
            wxScrollWinEvent_SetOrientation(_obj, orient)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Add(_obj: *u8 /* void* */, _dat: *u8 /* void* */, _preferred: c_int /* int */) {
        unsafe {
            wxDataObjectComposite_Add(_obj, _dat, _preferred)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxDataObjectComposite_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDataObjectComposite_Delete(_obj)
        }
    }
}
trait wxScreenDC {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxScreenDC_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxScreenDC_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EndDrawingOnTop(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxScreenDC_EndDrawingOnTop(_obj)
        }
    }
    #[fixed_stack_segment]
    fn StartDrawingOnTop(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxScreenDC_StartDrawingOnTop(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn StartDrawingOnTopOfWin(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxScreenDC_StartDrawingOnTopOfWin(_obj, win)
        }
    }
}
trait wxGraphicsPath {
}
trait wxHelpControllerBase {
}
trait wxTextDropTarget {
}
trait wxBoolProperty {
    #[fixed_stack_segment]
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxBoolProperty_Create(label, name, value)
        }
    }
}
trait wxSysColourChangedEvent {
}
trait wxDrawControl {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDrawControl_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait cbRowDragPlugin {
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            cbRowDragPlugin_Create(pPanel, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbRowDragPlugin_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbRowDragPlugin_Delete(_obj)
        }
    }
}
trait wxBufferedInputStream {
}
trait wxIconBundle {
    #[fixed_stack_segment]
    fn AddIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) {
        unsafe {
            wxIconBundle_AddIcon(_obj, icon)
        }
    }
    #[fixed_stack_segment]
    fn AddIconFromFile(_obj: *u8 /* void* */, file: *u8 /* void* */, type_: c_int /* int */) {
        unsafe {
            wxIconBundle_AddIconFromFile(_obj, file, type_)
        }
    }
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxIconBundle_Assign(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxIconBundle_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromFile(file: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxIconBundle_CreateFromFile(file, type_)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromIcon(icon: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxIconBundle_CreateFromIcon(icon)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxIconBundle_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetIcon(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxIconBundle_GetIcon(_obj, arg0, arg1, _ref)
        }
    }
}
trait wxGridCellFloatRenderer {
}
trait wxcPrintout {
}
trait wxTextValidator {
    #[fixed_stack_segment]
    fn Create(style: c_int /* int */, val: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextValidator_Create(style, val)
        }
    }
    #[fixed_stack_segment]
    fn GetExcludes(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxTextValidator_GetExcludes(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetIncludes(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxTextValidator_GetIncludes(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn SetExcludes(_obj: *u8 /* void* */, list: *wchar_t /* wchar_t* */, count: c_int /* int */) {
        unsafe {
            wxTextValidator_SetExcludes(_obj, list, count)
        }
    }
    #[fixed_stack_segment]
    fn SetIncludes(_obj: *u8 /* void* */, list: *wchar_t /* wchar_t* */, count: c_int /* int */) {
        unsafe {
            wxTextValidator_SetIncludes(_obj, list, count)
        }
    }
    #[fixed_stack_segment]
    fn Clone(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextValidator_Clone(_obj)
        }
    }
    #[fixed_stack_segment]
    fn TransferToWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextValidator_TransferToWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn TransferFromWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextValidator_TransferFromWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTextValidator_GetStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn OnChar(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxTextValidator_OnChar(_obj, event)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Area(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            cbStartDrawInAreaEvent_Area(_obj, arg0, arg1, arg2, arg3)
        }
    }
}
trait ELJCommand {
    #[fixed_stack_segment]
    fn CanUndo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJCommand_CanUndo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_und: c_int /* int */, _nme: *u8 /* void* */, _obj: *u8 /* void* */, _clb: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJCommand_Create(_und, _nme, _obj, _clb)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJCommand_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJCommand_GetName(_obj)
        }
    }
}
trait ELJDragDataObject {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _fmt: *u8 /* void* */, _func1: *u8 /* void* */, _func2: *u8 /* void* */, _func3: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJDragDataObject_Create(_obj, _fmt, _func1, _func2, _func3)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJDragDataObject_Delete(_obj)
        }
    }
}
trait wxStaticLine {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStaticLine_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStaticLine_GetDefaultSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsVertical(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStaticLine_IsVertical(_obj)
        }
    }
}
trait wxWindowDisabler {
}
trait wxTextInputStream {
    #[fixed_stack_segment]
    fn Create(inputStream: *u8 /* void* */, sep: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextInputStream_Create(inputStream, sep)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxTextInputStream_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn ReadLine(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextInputStream_ReadLine(self_)
        }
    }
}
trait wxListCtrl {
    #[fixed_stack_segment]
    fn Arrange(_obj: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_Arrange(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn ClearAll(_obj: *u8 /* void* */) {
        unsafe {
            wxListCtrl_ClearAll(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn DeleteAllColumns(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_DeleteAllColumns(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DeleteAllItems(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_DeleteAllItems(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DeleteColumn(_obj: *u8 /* void* */, col: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_DeleteColumn(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn DeleteItem(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_DeleteItem(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn EditLabel(_obj: *u8 /* void* */, item: c_int /* int */) {
        unsafe {
            wxListCtrl_EditLabel(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn EndEditLabel(_obj: *u8 /* void* */, cancel: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_EndEditLabel(_obj, cancel)
        }
    }
    #[fixed_stack_segment]
    fn EnsureVisible(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_EnsureVisible(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn FindItem(_obj: *u8 /* void* */, start: c_int /* int */, str: *u8 /* void* */, partial: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_FindItem(_obj, start, str, partial)
        }
    }
    #[fixed_stack_segment]
    fn FindItemByData(_obj: *u8 /* void* */, start: c_int /* int */, data: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_FindItemByData(_obj, start, data)
        }
    }
    #[fixed_stack_segment]
    fn FindItemByPosition(_obj: *u8 /* void* */, start: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, direction: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_FindItemByPosition(_obj, start, arg0, arg1, direction)
        }
    }
    #[fixed_stack_segment]
    fn GetColumn(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_GetColumn(_obj, col, item)
        }
    }
    #[fixed_stack_segment]
    fn GetColumnCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetColumnCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetColumnWidth(_obj: *u8 /* void* */, col: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetColumnWidth(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn GetCountPerPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetCountPerPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEditControl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetEditControl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetImageList(_obj: *u8 /* void* */, which: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetImageList(_obj, which)
        }
    }
    #[fixed_stack_segment]
    fn GetItem(_obj: *u8 /* void* */, info: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_GetItem(_obj, info)
        }
    }
    #[fixed_stack_segment]
    fn GetItemCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetItemCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetItemData(_obj: *u8 /* void* */, item: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetItemData(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn GetItemFont(_obj: *u8 /* void* */, item: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemFont(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn GetItemPosition(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemPosition(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn GetItemRect(_obj: *u8 /* void* */, item: c_int /* int */, code: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemRect(_obj, item, code)
        }
    }
    #[fixed_stack_segment]
    fn GetItemSpacing(_obj: *u8 /* void* */, isSmall: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemSpacing(_obj, isSmall)
        }
    }
    #[fixed_stack_segment]
    fn GetItemState(_obj: *u8 /* void* */, item: c_int /* int */, stateMask: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetItemState(_obj, item, stateMask)
        }
    }
    #[fixed_stack_segment]
    fn GetItemText(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemText(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn GetNextItem(_obj: *u8 /* void* */, item: c_int /* int */, geometry: c_int /* int */, state: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetNextItem(_obj, item, geometry, state)
        }
    }
    #[fixed_stack_segment]
    fn GetSelectedItemCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetSelectedItemCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListCtrl_GetTextColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetTopItem(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_GetTopItem(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_HitTest(_obj, arg0, arg1, flags)
        }
    }
    #[fixed_stack_segment]
    fn InsertColumn(_obj: *u8 /* void* */, col: c_int /* int */, heading: *u8 /* void* */, format: c_int /* int */, width: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertColumn(_obj, col, heading, format, width)
        }
    }
    #[fixed_stack_segment]
    fn InsertColumnFromInfo(_obj: *u8 /* void* */, col: c_int /* int */, info: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertColumnFromInfo(_obj, col, info)
        }
    }
    #[fixed_stack_segment]
    fn InsertItem(_obj: *u8 /* void* */, info: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertItem(_obj, info)
        }
    }
    #[fixed_stack_segment]
    fn InsertItemWithData(_obj: *u8 /* void* */, index: c_int /* int */, label: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertItemWithData(_obj, index, label)
        }
    }
    #[fixed_stack_segment]
    fn InsertItemWithImage(_obj: *u8 /* void* */, index: c_int /* int */, imageIndex: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertItemWithImage(_obj, index, imageIndex)
        }
    }
    #[fixed_stack_segment]
    fn InsertItemWithLabel(_obj: *u8 /* void* */, index: c_int /* int */, label: *u8 /* void* */, imageIndex: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_InsertItemWithLabel(_obj, index, label, imageIndex)
        }
    }
    #[fixed_stack_segment]
    fn IsVirtual(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_IsVirtual(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RefreshItem(_obj: *u8 /* void* */, item: c_long /* long */) {
        unsafe {
            wxListCtrl_RefreshItem(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn ScrollList(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_ScrollList(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetBackgroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxListCtrl_SetBackgroundColour(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetColumn(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetColumn(_obj, col, item)
        }
    }
    #[fixed_stack_segment]
    fn SetColumnWidth(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetColumnWidth(_obj, col, width)
        }
    }
    #[fixed_stack_segment]
    fn SetForegroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListCtrl_SetForegroundColour(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */, which: c_int /* int */) {
        unsafe {
            wxListCtrl_SetImageList(_obj, imageList, which)
        }
    }
    #[fixed_stack_segment]
    fn SetItem(_obj: *u8 /* void* */, index: c_int /* int */, col: c_int /* int */, label: *u8 /* void* */, imageId: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItem(_obj, index, col, label, imageId)
        }
    }
    #[fixed_stack_segment]
    fn SetItemData(_obj: *u8 /* void* */, item: c_int /* int */, data: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemData(_obj, item, data)
        }
    }
    #[fixed_stack_segment]
    fn SetItemFromInfo(_obj: *u8 /* void* */, info: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemFromInfo(_obj, info)
        }
    }
    #[fixed_stack_segment]
    fn SetItemImage(_obj: *u8 /* void* */, item: c_int /* int */, image: c_int /* int */, selImage: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemImage(_obj, item, image, selImage)
        }
    }
    #[fixed_stack_segment]
    fn SetItemPosition(_obj: *u8 /* void* */, item: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemPosition(_obj, item, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetItemState(_obj: *u8 /* void* */, item: c_int /* int */, state: c_int /* int */, stateMask: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SetItemState(_obj, item, state, stateMask)
        }
    }
    #[fixed_stack_segment]
    fn SetItemText(_obj: *u8 /* void* */, item: c_int /* int */, str: *u8 /* void* */) {
        unsafe {
            wxListCtrl_SetItemText(_obj, item, str)
        }
    }
    #[fixed_stack_segment]
    fn SetSingleStyle(_obj: *u8 /* void* */, style: c_int /* int */, add: bool /* bool */) {
        unsafe {
            wxListCtrl_SetSingleStyle(_obj, style, add)
        }
    }
    #[fixed_stack_segment]
    fn SetTextColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxListCtrl_SetTextColour(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetWindowStyleFlag(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxListCtrl_SetWindowStyleFlag(_obj, style)
        }
    }
    #[fixed_stack_segment]
    fn SortItems(_obj: *u8 /* void* */, fn_: *u8 /* void* */, eif_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SortItems(_obj, fn_, eif_obj)
        }
    }
    #[fixed_stack_segment]
    fn UpdateStyle(_obj: *u8 /* void* */) {
        unsafe {
            wxListCtrl_UpdateStyle(_obj)
        }
    }
}
trait wxThinSplitterWindow {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxThinSplitterWindow_Create(parent, id, arg0, arg1, arg2, arg3, style)
        }
    }
    #[fixed_stack_segment]
    fn DrawSash(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxThinSplitterWindow_DrawSash(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn SashHitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, tolerance: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxThinSplitterWindow_SashHitTest(_obj, arg0, arg1, tolerance)
        }
    }
    #[fixed_stack_segment]
    fn SizeWindows(_obj: *u8 /* void* */) {
        unsafe {
            wxThinSplitterWindow_SizeWindows(_obj)
        }
    }
}
trait wxGridCellWorker {
}
trait wxWindowCreateEvent {
    #[fixed_stack_segment]
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindowCreateEvent_GetWindow(_obj)
        }
    }
}
trait wxcHtmlEvent {
    #[fixed_stack_segment]
    fn GetMouseEvent(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetMouseEvent(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetHtmlCell(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetHtmlCell(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetHtmlCellId(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetHtmlCellId(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetHref(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetHref(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetTarget(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetTarget(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetLogicalPosition(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlEvent_GetLogicalPosition(self_)
        }
    }
}
trait wxAutomationObject {
}
trait wxPageSetupDialog {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialog_Create(parent, data)
        }
    }
    #[fixed_stack_segment]
    fn GetPageSetupData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialog_GetPageSetupData(_obj, _ref)
        }
    }
}
trait wxMultiCellSizer {
    #[fixed_stack_segment]
    fn CalcMin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxMultiCellSizer_CalcMin(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Create(rows: c_int /* int */, cols: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMultiCellSizer_Create(rows, cols)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMultiCellSizer_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EnableGridLines(_obj: *u8 /* void* */, win: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellSizer_EnableGridLines(_obj, win)
        }
    }
    #[fixed_stack_segment]
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxMultiCellSizer_RecalcSizes(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetColumnWidth(_obj: *u8 /* void* */, column: c_int /* int */, colSize: c_int /* int */, expandable: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxMultiCellSizer_SetColumnWidth(_obj, column, colSize, expandable)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultCellSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxMultiCellSizer_SetDefaultCellSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetGridPen(_obj: *u8 /* void* */, pen: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellSizer_SetGridPen(_obj, pen)
        }
    }
    #[fixed_stack_segment]
    fn SetRowHeight(_obj: *u8 /* void* */, row: c_int /* int */, rowSize: c_int /* int */, expandable: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxMultiCellSizer_SetRowHeight(_obj, row, rowSize, expandable)
        }
    }
}
trait wxPaintDC {
    #[fixed_stack_segment]
    fn Create(win: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPaintDC_Create(win)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
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
trait wxMediaCtrl {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, windowID: c_int /* int */, fileName: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */, w: c_int /* int */, h: c_int /* int */, style: c_long /* long */, szBackend: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMediaCtrl_Create(parent, windowID, fileName, x, y, w, h, style, szBackend, name)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxMediaCtrl_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetBestSize(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMediaCtrl_GetBestSize(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetPlaybackRate(self_: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxMediaCtrl_GetPlaybackRate(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetVolume(self_: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxMediaCtrl_GetVolume(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetState(self_: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMediaCtrl_GetState(self_)
        }
    }
    #[fixed_stack_segment]
    fn Length(self_: *u8 /* void* */) -> i64 /* i64 */ {
        unsafe {
            wxMediaCtrl_Length(self_)
        }
    }
    #[fixed_stack_segment]
    fn Load(self_: *u8 /* void* */, fileName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMediaCtrl_Load(self_, fileName)
        }
    }
    #[fixed_stack_segment]
    fn LoadURI(self_: *u8 /* void* */, uri: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMediaCtrl_LoadURI(self_, uri)
        }
    }
    #[fixed_stack_segment]
    fn LoadURIWithProxy(self_: *u8 /* void* */, uri: *u8 /* void* */, proxy: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMediaCtrl_LoadURIWithProxy(self_, uri, proxy)
        }
    }
    #[fixed_stack_segment]
    fn Pause(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMediaCtrl_Pause(self_)
        }
    }
    #[fixed_stack_segment]
    fn Play(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMediaCtrl_Play(self_)
        }
    }
    #[fixed_stack_segment]
    fn Seek(self_: *u8 /* void* */, offsetWhere: i64 /* i64 */, mode: c_int /* int */) -> i64 /* i64 */ {
        unsafe {
            wxMediaCtrl_Seek(self_, offsetWhere, mode)
        }
    }
    #[fixed_stack_segment]
    fn SetPlaybackRate(self_: *u8 /* void* */, dRate: c_double /* double */) -> bool /* bool */ {
        unsafe {
            wxMediaCtrl_SetPlaybackRate(self_, dRate)
        }
    }
    #[fixed_stack_segment]
    fn SetVolume(self_: *u8 /* void* */, dVolume: c_double /* double */) -> bool /* bool */ {
        unsafe {
            wxMediaCtrl_SetVolume(self_, dVolume)
        }
    }
    #[fixed_stack_segment]
    fn ShowPlayerControls(self_: *u8 /* void* */, flags: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMediaCtrl_ShowPlayerControls(self_, flags)
        }
    }
    #[fixed_stack_segment]
    fn Stop(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMediaCtrl_Stop(self_)
        }
    }
    #[fixed_stack_segment]
    fn Tell(self_: *u8 /* void* */) -> i64 /* i64 */ {
        unsafe {
            wxMediaCtrl_Tell(self_)
        }
    }
}
trait wxPaintEvent {
}
trait wxSocketServer {
}
trait wxString {
}
trait wxBitmapToggleButton {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmapToggleButton_Create(parent, id, _bmp, arg0, arg1, arg2, arg3, style)
        }
    }
    #[fixed_stack_segment]
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxBitmapToggleButton_Enable(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBitmapToggleButton_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, state: bool /* bool */) {
        unsafe {
            wxBitmapToggleButton_SetValue(_obj, state)
        }
    }
    #[fixed_stack_segment]
    fn SetBitmapLabel(_obj: *u8 /* void* */, _bmp: *u8 /* void* */) {
        unsafe {
            wxBitmapToggleButton_SetBitmapLabel(_obj, _bmp)
        }
    }
}
trait wxDbSqlTypeInfo {
}
trait wxFontDialog {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, fnt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFontDialog_Create(_prt, fnt)
        }
    }
    #[fixed_stack_segment]
    fn GetFontData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxFontDialog_GetFontData(_obj, _ref)
        }
    }
}
trait wxOutputStream {
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxOutputStream_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LastWrite(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxOutputStream_LastWrite(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PutC(_obj: *u8 /* void* */, c: wchar_t /* wchar_t */) {
        unsafe {
            wxOutputStream_PutC(_obj, c)
        }
    }
    #[fixed_stack_segment]
    fn Seek(_obj: *u8 /* void* */, pos: c_int /* int */, mode: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxOutputStream_Seek(_obj, pos, mode)
        }
    }
    #[fixed_stack_segment]
    fn Sync(_obj: *u8 /* void* */) {
        unsafe {
            wxOutputStream_Sync(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Tell(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxOutputStream_Tell(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxAcceleratorEntry_Create(flags, keyCode, cmd)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxAcceleratorEntry_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCommand(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxAcceleratorEntry_GetCommand(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxAcceleratorEntry_GetFlags(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetKeyCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxAcceleratorEntry_GetKeyCode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Set(_obj: *u8 /* void* */, flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */) {
        unsafe {
            wxAcceleratorEntry_Set(_obj, flags, keyCode, cmd)
        }
    }
}
trait wxDialUpManager {
    #[fixed_stack_segment]
    fn CancelDialing(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialUpManager_CancelDialing(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxDialUpManager_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDialUpManager_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Dial(_obj: *u8 /* void* */, nameOfISP: *u8 /* void* */, username: *u8 /* void* */, password: *u8 /* void* */, async: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxDialUpManager_Dial(_obj, nameOfISP, username, password, async)
        }
    }
    #[fixed_stack_segment]
    fn DisableAutoCheckOnlineStatus(_obj: *u8 /* void* */) {
        unsafe {
            wxDialUpManager_DisableAutoCheckOnlineStatus(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EnableAutoCheckOnlineStatus(_obj: *u8 /* void* */, nSeconds: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDialUpManager_EnableAutoCheckOnlineStatus(_obj, nSeconds)
        }
    }
    #[fixed_stack_segment]
    fn GetISPNames(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDialUpManager_GetISPNames(_obj, _lst)
        }
    }
    #[fixed_stack_segment]
    fn HangUp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialUpManager_HangUp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsAlwaysOnline(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialUpManager_IsAlwaysOnline(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsDialing(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialUpManager_IsDialing(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialUpManager_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOnline(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialUpManager_IsOnline(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetConnectCommand(_obj: *u8 /* void* */, commandDial: *u8 /* void* */, commandHangup: *u8 /* void* */) {
        unsafe {
            wxDialUpManager_SetConnectCommand(_obj, commandDial, commandHangup)
        }
    }
    #[fixed_stack_segment]
    fn SetOnlineStatus(_obj: *u8 /* void* */, isOnline: bool /* bool */) {
        unsafe {
            wxDialUpManager_SetOnlineStatus(_obj, isOnline)
        }
    }
    #[fixed_stack_segment]
    fn SetWellKnownHost(_obj: *u8 /* void* */, hostname: *u8 /* void* */, portno: c_int /* int */) {
        unsafe {
            wxDialUpManager_SetWellKnownHost(_obj, hostname, portno)
        }
    }
}
trait wxDynToolInfo {
    #[fixed_stack_segment]
    fn Index(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDynToolInfo_Index(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RealSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxDynToolInfo_RealSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn pToolWnd(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDynToolInfo_pToolWnd(_obj)
        }
    }
}
trait wxDocTemplate {
}
trait wxPreviewControlBar {
    #[fixed_stack_segment]
    fn expEVT_PRINT_BEGIN() -> c_int /* int */ {
        unsafe {
            expEVT_PRINT_BEGIN()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_PRINT_BEGIN_DOC() -> c_int /* int */ {
        unsafe {
            expEVT_PRINT_BEGIN_DOC()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_PRINT_END() -> c_int /* int */ {
        unsafe {
            expEVT_PRINT_END()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_PRINT_END_DOC() -> c_int /* int */ {
        unsafe {
            expEVT_PRINT_END_DOC()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_PRINT_PREPARE() -> c_int /* int */ {
        unsafe {
            expEVT_PRINT_PREPARE()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_PRINT_PAGE() -> c_int /* int */ {
        unsafe {
            expEVT_PRINT_PAGE()
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_GetDC(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintout_GetDC(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_GetPPIPrinter(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxPrintout_GetPPIPrinter(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_GetPPIScreen(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxPrintout_GetPPIScreen(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_GetPageSizeMM(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxPrintout_GetPageSizeMM(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_GetPageSizePixels(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxPrintout_GetPageSizePixels(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintout_GetTitle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_IsPreview(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintout_IsPreview(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_SetDC(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxPrintout_SetDC(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_SetPPIPrinter(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPrintout_SetPPIPrinter(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_SetPPIScreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPrintout_SetPPIScreen(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_SetPageSizeMM(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPrintout_SetPageSizeMM(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxPrintout_SetPageSizePixels(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPrintout_SetPageSizePixels(_obj, arg0, arg1)
        }
    }
}
trait ELJPreviewFrame {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _init: *u8 /* void* */, _create_canvas: *u8 /* void* */, _create_toolbar: *u8 /* void* */, preview: *u8 /* void* */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent, title, arg0, arg1, arg2, arg3, style)
        }
    }
    #[fixed_stack_segment]
    fn GetControlBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewFrame_GetControlBar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPreviewCanvas(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewFrame_GetPreviewCanvas(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintPreview(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewFrame_GetPrintPreview(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Initialize(_obj: *u8 /* void* */) {
        unsafe {
            ELJPreviewFrame_Initialize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetControlBar(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            ELJPreviewFrame_SetControlBar(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn SetPreviewCanvas(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            ELJPreviewFrame_SetPreviewCanvas(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn SetPrintPreview(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            ELJPreviewFrame_SetPrintPreview(_obj, obj)
        }
    }
}
trait wxBitmapHandler {
}
trait wxGraphicsContext {
}
trait wxGaugeMSW {
}
trait cbDimInfo {
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, other: *u8 /* void* */) {
        unsafe {
            cbDimInfo_Assign(_obj, other)
        }
    }
    #[fixed_stack_segment]
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */, isFixed: bool /* bool */, gap: c_int /* int */, pDimHandler: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDimInfo_Create(arg0, arg1, isFixed, gap, pDimHandler)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbDimInfo_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateWithHandler(pDimHandler: *u8 /* void* */, isFixed: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            cbDimInfo_CreateWithHandler(pDimHandler, isFixed)
        }
    }
    #[fixed_stack_segment]
    fn CreateWithInfo(dh_x: c_int /* int */, dh_y: c_int /* int */, dv_x: c_int /* int */, dv_y: c_int /* int */, f_x: c_int /* int */, f_y: c_int /* int */, isFixed: bool /* bool */, horizGap: c_int /* int */, vertGap: c_int /* int */, pDimHandler: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDimInfo_CreateWithInfo(dh_x, dh_y, dv_x, dv_y, f_x, f_y, isFixed, horizGap, vertGap, pDimHandler)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbDimInfo_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDimHandler(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDimInfo_GetDimHandler(_obj)
        }
    }
}
trait wxMDIClientWindow {
}
trait wxDataObjectSimple {
}
trait wxDialog {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDialog_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn EndModal(_obj: *u8 /* void* */, retCode: c_int /* int */) {
        unsafe {
            wxDialog_EndModal(_obj, retCode)
        }
    }
    #[fixed_stack_segment]
    fn GetReturnCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDialog_GetReturnCode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsModal(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialog_IsModal(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetReturnCode(_obj: *u8 /* void* */, returnCode: c_int /* int */) {
        unsafe {
            wxDialog_SetReturnCode(_obj, returnCode)
        }
    }
    #[fixed_stack_segment]
    fn ShowModal(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDialog_ShowModal(_obj)
        }
    }
}
trait wxImageHandler {
}
trait wxClosure {
    #[fixed_stack_segment]
    fn Create(_fun_CEvent: *u8 /* void* */, _data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClosure_Create(_fun_CEvent, _data)
        }
    }
    #[fixed_stack_segment]
    fn GetData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClosure_GetData(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxEvtHandler_GetClosure(_obj: *u8 /* void* */, id: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_GetClosure(_obj, id, type_)
        }
    }
    #[fixed_stack_segment]
    fn wxEvtHandler_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_GetClientClosure(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxEvtHandler_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_SetClientClosure(_obj, closure)
        }
    }
    #[fixed_stack_segment]
    fn wxObject_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxObject_GetClientClosure(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxObject_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxObject_SetClientClosure(_obj, closure)
        }
    }
}
trait wxIndividualLayoutConstraint {
    #[fixed_stack_segment]
    fn Above(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_Above(_obj, sibling, marg)
        }
    }
    #[fixed_stack_segment]
    fn Absolute(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_Absolute(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn AsIs(_obj: *u8 /* void* */) {
        unsafe {
            wxIndividualLayoutConstraint_AsIs(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Below(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_Below(_obj, sibling, marg)
        }
    }
    #[fixed_stack_segment]
    fn GetDone(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIndividualLayoutConstraint_GetDone(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEdge(_obj: *u8 /* void* */, which: c_int /* int */, thisWin: *u8 /* void* */, other: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetEdge(_obj, which, thisWin, other)
        }
    }
    #[fixed_stack_segment]
    fn GetMargin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetMargin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMyEdge(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetMyEdge(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOtherEdge(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetOtherEdge(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOtherWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxIndividualLayoutConstraint_GetOtherWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPercent(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetPercent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRelationship(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetRelationship(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIndividualLayoutConstraint_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LeftOf(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_LeftOf(_obj, sibling, marg)
        }
    }
    #[fixed_stack_segment]
    fn PercentOf(_obj: *u8 /* void* */, otherW: *u8 /* void* */, wh: c_int /* int */, per: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_PercentOf(_obj, otherW, wh, per)
        }
    }
    #[fixed_stack_segment]
    fn ResetIfWin(_obj: *u8 /* void* */, otherW: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIndividualLayoutConstraint_ResetIfWin(_obj, otherW)
        }
    }
    #[fixed_stack_segment]
    fn RightOf(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_RightOf(_obj, sibling, marg)
        }
    }
    #[fixed_stack_segment]
    fn SameAs(_obj: *u8 /* void* */, otherW: *u8 /* void* */, edge: c_int /* int */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SameAs(_obj, otherW, edge, marg)
        }
    }
    #[fixed_stack_segment]
    fn SatisfyConstraint(_obj: *u8 /* void* */, constraints: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIndividualLayoutConstraint_SatisfyConstraint(_obj, constraints, win)
        }
    }
    #[fixed_stack_segment]
    fn Set(_obj: *u8 /* void* */, rel: c_int /* int */, otherW: *u8 /* void* */, otherE: c_int /* int */, val: c_int /* int */, marg: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_Set(_obj, rel, otherW, otherE, val, marg)
        }
    }
    #[fixed_stack_segment]
    fn SetDone(_obj: *u8 /* void* */, d: bool /* bool */) {
        unsafe {
            wxIndividualLayoutConstraint_SetDone(_obj, d)
        }
    }
    #[fixed_stack_segment]
    fn SetEdge(_obj: *u8 /* void* */, which: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SetEdge(_obj, which)
        }
    }
    #[fixed_stack_segment]
    fn SetMargin(_obj: *u8 /* void* */, m: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SetMargin(_obj, m)
        }
    }
    #[fixed_stack_segment]
    fn SetRelationship(_obj: *u8 /* void* */, r: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SetRelationship(_obj, r)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxIndividualLayoutConstraint_SetValue(_obj, v)
        }
    }
    #[fixed_stack_segment]
    fn Unconstrained(_obj: *u8 /* void* */) {
        unsafe {
            wxIndividualLayoutConstraint_Unconstrained(_obj)
        }
    }
}
trait wxTempFile {
}
trait cbFloatedBarWindow {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbFloatedBarWindow_Create(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbFloatedBarWindow_GetBar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PositionFloatedWnd(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            cbFloatedBarWindow_PositionFloatedWnd(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn SetBar(_obj: *u8 /* void* */, _bar: *u8 /* void* */) {
        unsafe {
            cbFloatedBarWindow_SetBar(_obj, _bar)
        }
    }
    #[fixed_stack_segment]
    fn SetLayout(_obj: *u8 /* void* */, _layout: *u8 /* void* */) {
        unsafe {
            cbFloatedBarWindow_SetLayout(_obj, _layout)
        }
    }
}
trait cbLeftUpEvent {
    #[fixed_stack_segment]
    fn Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbLeftUpEvent_Pos(_obj, arg0, arg1)
        }
    }
}
trait wxHtmlTagsModule {
}
trait wxMirrorDC {
    #[fixed_stack_segment]
    fn Create(dc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMirrorDC_Create(dc)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMirrorDC_Delete(_obj)
        }
    }
}
trait wxLocale {
    #[fixed_stack_segment]
    fn AddCatalog(_obj: *u8 /* void* */, szDomain: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxLocale_AddCatalog(_obj, szDomain)
        }
    }
    #[fixed_stack_segment]
    fn AddCatalogLookupPathPrefix(_obj: *u8 /* void* */, prefix: *u8 /* void* */) {
        unsafe {
            wxLocale_AddCatalogLookupPathPrefix(_obj, prefix)
        }
    }
    #[fixed_stack_segment]
    fn Create(_name: c_int /* int */, _flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxLocale_Create(_name, _flags)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxLocale_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLocale(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLocale_GetLocale(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLocale_GetName(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetString(_obj: *u8 /* void* */, szOrigString: *u8 /* void* */, szDomain: *u8 /* void* */) -> *wchar_t /* wchar_t* */ {
        unsafe {
            wxLocale_GetString(_obj, szOrigString, szDomain)
        }
    }
    #[fixed_stack_segment]
    fn IsLoaded(_obj: *u8 /* void* */, szDomain: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLocale_IsLoaded(_obj, szDomain)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLocale_IsOk(_obj)
        }
    }
}
trait wxFindDialogEvent {
    #[fixed_stack_segment]
    fn GetFindString(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFindDialogEvent_GetFindString(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFindDialogEvent_GetFlags(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Blit(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, source: *u8 /* void* */, arg4: c_int /* int */, arg5: c_int /* int */, rop: c_int /* int */, useMask: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxDC_Blit(_obj, arg0, arg1, arg2, arg3, source, arg4, arg5, rop, useMask)
        }
    }
    #[fixed_stack_segment]
    fn CalcBoundingBox(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_CalcBoundingBox(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn CanDrawBitmap(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_CanDrawBitmap(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CanGetTextExtent(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_CanGetTextExtent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_Clear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ComputeScaleAndOrigin(obj: *u8 /* void* */) {
        unsafe {
            wxDC_ComputeScaleAndOrigin(obj)
        }
    }
    #[fixed_stack_segment]
    fn CrossHair(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_CrossHair(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DestroyClippingRegion(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_DestroyClippingRegion(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DeviceToLogicalX(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_DeviceToLogicalX(_obj, x)
        }
    }
    #[fixed_stack_segment]
    fn DeviceToLogicalXRel(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_DeviceToLogicalXRel(_obj, x)
        }
    }
    #[fixed_stack_segment]
    fn DeviceToLogicalY(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_DeviceToLogicalY(_obj, y)
        }
    }
    #[fixed_stack_segment]
    fn DeviceToLogicalYRel(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_DeviceToLogicalYRel(_obj, y)
        }
    }
    #[fixed_stack_segment]
    fn DrawArc(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: c_int /* int */) {
        unsafe {
            wxDC_DrawArc(_obj, arg0, arg1, arg2, arg3, arg4, arg5)
        }
    }
    #[fixed_stack_segment]
    fn DrawBitmap(_obj: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, useMask: bool /* bool */) {
        unsafe {
            wxDC_DrawBitmap(_obj, bmp, arg0, arg1, useMask)
        }
    }
    #[fixed_stack_segment]
    fn DrawCheckMark(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_DrawCheckMark(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn DrawCircle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, radius: c_int /* int */) {
        unsafe {
            wxDC_DrawCircle(_obj, arg0, arg1, radius)
        }
    }
    #[fixed_stack_segment]
    fn DrawEllipse(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_DrawEllipse(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn DrawEllipticArc(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, sa: c_double /* double */, ea: c_double /* double */) {
        unsafe {
            wxDC_DrawEllipticArc(_obj, arg0, arg1, arg2, arg3, sa, ea)
        }
    }
    #[fixed_stack_segment]
    fn DrawIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_DrawIcon(_obj, icon, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn DrawLabel(_obj: *u8 /* void* */, str: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, align: c_int /* int */, indexAccel: c_int /* int */) {
        unsafe {
            wxDC_DrawLabel(_obj, str, arg0, arg1, arg2, arg3, align, indexAccel)
        }
    }
    #[fixed_stack_segment]
    fn DrawLabelBitmap(_obj: *u8 /* void* */, str: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, align: c_int /* int */, indexAccel: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDC_DrawLabelBitmap(_obj, str, bmp, arg0, arg1, arg2, arg3, align, indexAccel)
        }
    }
    #[fixed_stack_segment]
    fn DrawLine(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_DrawLine(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn DrawLines(_obj: *u8 /* void* */, n: c_int /* int */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_DrawLines(_obj, n, x, y, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn DrawPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_DrawPoint(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn DrawPolygon(_obj: *u8 /* void* */, n: c_int /* int */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, fillStyle: c_int /* int */) {
        unsafe {
            wxDC_DrawPolygon(_obj, n, x, y, arg0, arg1, fillStyle)
        }
    }
    #[fixed_stack_segment]
    fn DrawPolyPolygon(_obj: *u8 /* void* */, n: c_int /* int */, count: *u8 /* void* */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, fillStyle: c_int /* int */) {
        unsafe {
            wxDC_DrawPolyPolygon(_obj, n, count, x, y, arg0, arg1, fillStyle)
        }
    }
    #[fixed_stack_segment]
    fn DrawRectangle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_DrawRectangle(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn DrawRotatedText(_obj: *u8 /* void* */, text: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, angle: c_double /* double */) {
        unsafe {
            wxDC_DrawRotatedText(_obj, text, arg0, arg1, angle)
        }
    }
    #[fixed_stack_segment]
    fn DrawRoundedRectangle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, radius: c_double /* double */) {
        unsafe {
            wxDC_DrawRoundedRectangle(_obj, arg0, arg1, arg2, arg3, radius)
        }
    }
    #[fixed_stack_segment]
    fn DrawText(_obj: *u8 /* void* */, text: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_DrawText(_obj, text, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn EndDoc(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_EndDoc(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EndPage(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_EndPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FloodFill(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxDC_FloodFill(_obj, arg0, arg1, col, style)
        }
    }
    #[fixed_stack_segment]
    fn GetBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetBackground(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetBackgroundMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetBackgroundMode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBrush(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetBrush(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetCharHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetCharHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCharWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetCharWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetClippingBox(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxDC_GetClippingBox(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn GetDepth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetDepth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDeviceOrigin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxDC_GetDeviceOrigin(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetFont(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetLogicalFunction(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetLogicalFunction(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLogicalOrigin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxDC_GetLogicalOrigin(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetLogicalScale(_obj: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */) {
        unsafe {
            wxDC_GetLogicalScale(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetMapMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_GetMapMode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPPI(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDC_GetPPI(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPen(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetPen(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetPixel(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_GetPixel(_obj, arg0, arg1, col)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDC_GetSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSizeMM(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDC_GetSizeMM(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTextBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetTextBackground(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetTextExtent(self_: *u8 /* void* */, string: *u8 /* void* */, w: *u8 /* void* */, h: *u8 /* void* */, descent: *u8 /* void* */, externalLeading: *u8 /* void* */, theFont: *u8 /* void* */) {
        unsafe {
            wxDC_GetTextExtent(self_, string, w, h, descent, externalLeading, theFont)
        }
    }
    #[fixed_stack_segment]
    fn GetMultiLineTextExtent(self_: *u8 /* void* */, string: *u8 /* void* */, w: *u8 /* void* */, h: *u8 /* void* */, heightLine: *u8 /* void* */, theFont: *u8 /* void* */) {
        unsafe {
            wxDC_GetMultiLineTextExtent(self_, string, w, h, heightLine, theFont)
        }
    }
    #[fixed_stack_segment]
    fn GetTextForeground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDC_GetTextForeground(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetUserScale(_obj: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */) {
        unsafe {
            wxDC_GetUserScale(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn LogicalToDeviceX(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_LogicalToDeviceX(_obj, x)
        }
    }
    #[fixed_stack_segment]
    fn LogicalToDeviceXRel(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_LogicalToDeviceXRel(_obj, x)
        }
    }
    #[fixed_stack_segment]
    fn LogicalToDeviceY(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_LogicalToDeviceY(_obj, y)
        }
    }
    #[fixed_stack_segment]
    fn LogicalToDeviceYRel(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDC_LogicalToDeviceYRel(_obj, y)
        }
    }
    #[fixed_stack_segment]
    fn MaxX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_MaxX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MaxY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_MaxY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MinX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_MinX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MinY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDC_MinY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ResetBoundingBox(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_ResetBoundingBox(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetAxisOrientation(_obj: *u8 /* void* */, xLeftRight: bool /* bool */, yBottomUp: bool /* bool */) {
        unsafe {
            wxDC_SetAxisOrientation(_obj, xLeftRight, yBottomUp)
        }
    }
    #[fixed_stack_segment]
    fn SetBackground(_obj: *u8 /* void* */, brush: *u8 /* void* */) {
        unsafe {
            wxDC_SetBackground(_obj, brush)
        }
    }
    #[fixed_stack_segment]
    fn SetBackgroundMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxDC_SetBackgroundMode(_obj, mode)
        }
    }
    #[fixed_stack_segment]
    fn SetBrush(_obj: *u8 /* void* */, brush: *u8 /* void* */) {
        unsafe {
            wxDC_SetBrush(_obj, brush)
        }
    }
    #[fixed_stack_segment]
    fn SetClippingRegion(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxDC_SetClippingRegion(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn SetClippingRegionFromRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) {
        unsafe {
            wxDC_SetClippingRegionFromRegion(_obj, region)
        }
    }
    #[fixed_stack_segment]
    fn SetDeviceOrigin(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_SetDeviceOrigin(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxDC_SetFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn SetLogicalFunction(_obj: *u8 /* void* */, function: c_int /* int */) {
        unsafe {
            wxDC_SetLogicalFunction(_obj, function)
        }
    }
    #[fixed_stack_segment]
    fn SetLogicalOrigin(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDC_SetLogicalOrigin(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetLogicalScale(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */) {
        unsafe {
            wxDC_SetLogicalScale(_obj, x, y)
        }
    }
    #[fixed_stack_segment]
    fn SetMapMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxDC_SetMapMode(_obj, mode)
        }
    }
    #[fixed_stack_segment]
    fn SetPalette(_obj: *u8 /* void* */, palette: *u8 /* void* */) {
        unsafe {
            wxDC_SetPalette(_obj, palette)
        }
    }
    #[fixed_stack_segment]
    fn SetPen(_obj: *u8 /* void* */, pen: *u8 /* void* */) {
        unsafe {
            wxDC_SetPen(_obj, pen)
        }
    }
    #[fixed_stack_segment]
    fn SetTextBackground(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxDC_SetTextBackground(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetTextForeground(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxDC_SetTextForeground(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetUserScale(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */) {
        unsafe {
            wxDC_SetUserScale(_obj, x, y)
        }
    }
    #[fixed_stack_segment]
    fn StartDoc(_obj: *u8 /* void* */, msg: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDC_StartDoc(_obj, msg)
        }
    }
    #[fixed_stack_segment]
    fn StartPage(_obj: *u8 /* void* */) {
        unsafe {
            wxDC_StartPage(_obj)
        }
    }
}
trait ELJTextValidator {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */, _txt: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJTextValidator_Create(_obj, _fnc, _txt, _stl)
        }
    }
}
trait wxHelpProvider {
    #[fixed_stack_segment]
    fn AddHelp(_obj: *u8 /* void* */, window: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxHelpProvider_AddHelp(_obj, window, text)
        }
    }
    #[fixed_stack_segment]
    fn AddHelpById(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */) {
        unsafe {
            wxHelpProvider_AddHelpById(_obj, id, text)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxHelpProvider_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Get() -> *u8 /* void* */ {
        unsafe {
            wxHelpProvider_Get()
        }
    }
    #[fixed_stack_segment]
    fn GetHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpProvider_GetHelp(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn RemoveHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxHelpProvider_RemoveHelp(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn Set(helpProvider: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpProvider_Set(helpProvider)
        }
    }
    #[fixed_stack_segment]
    fn ShowHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHelpProvider_ShowHelp(_obj, window)
        }
    }
}
trait wxHashMap {
}
trait wxGLContext {
    #[fixed_stack_segment]
    fn wxGLCanvas_Create(parent: *u8 /* void* */, windowID: c_int /* int */, attributes: *c_int /* int* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */, title: *u8 /* void* */, palette: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGLCanvas_Create(parent, windowID, attributes, arg0, arg1, arg2, arg3, style, title, palette)
        }
    }
    #[fixed_stack_segment]
    fn wxGLCanvas_SetColour(self_: *u8 /* void* */, colour: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGLCanvas_SetColour(self_, colour)
        }
    }
    #[fixed_stack_segment]
    fn wxGLCanvas_SetCurrent(self_: *u8 /* void* */, ctxt: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGLCanvas_SetCurrent(self_, ctxt)
        }
    }
    #[fixed_stack_segment]
    fn wxGLCanvas_SwapBuffers(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGLCanvas_SwapBuffers(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGLCanvas_IsDisplaySupported(attributes: *c_int /* int* */) -> bool /* bool */ {
        unsafe {
            wxGLCanvas_IsDisplaySupported(attributes)
        }
    }
    #[fixed_stack_segment]
    fn wxGLCanvas_IsExtensionSupported(extension: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGLCanvas_IsExtensionSupported(extension)
        }
    }
    #[fixed_stack_segment]
    fn Create(win: *u8 /* void* */, other: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGLContext_Create(win, other)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromNull(win: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGLContext_CreateFromNull(win)
        }
    }
    #[fixed_stack_segment]
    fn SetCurrent(self_: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGLContext_SetCurrent(self_, win)
        }
    }
}
trait wxGraphicsRenderer {
    #[fixed_stack_segment]
    fn wxGraphicsBrush_Create() -> *u8 /* void* */ {
        unsafe {
            wxGraphicsBrush_Create()
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsBrush_Delete(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsBrush_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_Create(dc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsContext_Create(dc)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_CreateFromWindow(window: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsContext_CreateFromWindow(window)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_Delete(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_CreateFromNative(context: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsContext_CreateFromNative(context)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_CreateFromNativeWindow(window: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsContext_CreateFromNativeWindow(window)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_Clip(self_: *u8 /* void* */, region: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_Clip(self_, region)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_ClipByRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */) {
        unsafe {
            wxGraphicsContext_ClipByRectangle(self_, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_ResetClip(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_ResetClip(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_DrawBitmap(self_: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */) {
        unsafe {
            wxGraphicsContext_DrawBitmap(self_, bmp, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_DrawEllipse(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */) {
        unsafe {
            wxGraphicsContext_DrawEllipse(self_, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_DrawIcon(self_: *u8 /* void* */, icon: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */) {
        unsafe {
            wxGraphicsContext_DrawIcon(self_, icon, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_DrawLines(self_: *u8 /* void* */, n: size_t /* size_t */, x: *u8 /* void* */, y: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxGraphicsContext_DrawLines(self_, n, x, y, style)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_DrawPath(self_: *u8 /* void* */, path: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxGraphicsContext_DrawPath(self_, path, style)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_DrawRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */) {
        unsafe {
            wxGraphicsContext_DrawRectangle(self_, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_DrawRoundedRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */, radius: c_double /* double */) {
        unsafe {
            wxGraphicsContext_DrawRoundedRectangle(self_, arg0, arg1, arg2, arg3, radius)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_DrawText(self_: *u8 /* void* */, text: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */) {
        unsafe {
            wxGraphicsContext_DrawText(self_, text, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_DrawTextWithAngle(self_: *u8 /* void* */, text: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, radius: c_double /* double */) {
        unsafe {
            wxGraphicsContext_DrawTextWithAngle(self_, text, arg0, arg1, radius)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_FillPath(self_: *u8 /* void* */, path: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxGraphicsContext_FillPath(self_, path, style)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_StrokePath(self_: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_StrokePath(self_, path)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_GetNativeContext(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsContext_GetNativeContext(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_GetTextExtent(self_: *u8 /* void* */, text: *u8 /* void* */, width: *c_double /* double* */, height: *c_double /* double* */, descent: *c_double /* double* */, externalLeading: *c_double /* double* */) {
        unsafe {
            wxGraphicsContext_GetTextExtent(self_, text, width, height, descent, externalLeading)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_Rotate(self_: *u8 /* void* */, angle: c_double /* double */) {
        unsafe {
            wxGraphicsContext_Rotate(self_, angle)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_Scale(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */) {
        unsafe {
            wxGraphicsContext_Scale(self_, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_Translate(self_: *u8 /* void* */, dx: c_double /* double */, dy: c_double /* double */) {
        unsafe {
            wxGraphicsContext_Translate(self_, dx, dy)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_SetTransform(self_: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_SetTransform(self_, path)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_ConcatTransform(self_: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_ConcatTransform(self_, path)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_SetBrush(self_: *u8 /* void* */, brush: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_SetBrush(self_, brush)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_SetGraphicsBrush(self_: *u8 /* void* */, brush: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_SetGraphicsBrush(self_, brush)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_SetFont(self_: *u8 /* void* */, font: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_SetFont(self_, font, colour)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_SetGraphicsFont(self_: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_SetGraphicsFont(self_, font)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_SetPen(self_: *u8 /* void* */, pen: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_SetPen(self_, pen)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_SetGraphicsPen(self_: *u8 /* void* */, pen: *u8 /* void* */) {
        unsafe {
            wxGraphicsContext_SetGraphicsPen(self_, pen)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_StrokeLine(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */) {
        unsafe {
            wxGraphicsContext_StrokeLine(self_, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsContext_StrokeLines(self_: *u8 /* void* */, n: size_t /* size_t */, x: *u8 /* void* */, y: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxGraphicsContext_StrokeLines(self_, n, x, y, style)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsFont_Create() -> *u8 /* void* */ {
        unsafe {
            wxGraphicsFont_Create()
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsFont_Delete(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsFont_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_Create() -> *u8 /* void* */ {
        unsafe {
            wxGraphicsMatrix_Create()
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_Delete(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsMatrix_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_Concat(self_: *u8 /* void* */, t: *u8 /* void* */) {
        unsafe {
            wxGraphicsMatrix_Concat(self_, t)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_Get(self_: *u8 /* void* */, a: *c_double /* double* */, b: *c_double /* double* */, c: *c_double /* double* */, d: *c_double /* double* */, tx: *c_double /* double* */, ty: *c_double /* double* */) {
        unsafe {
            wxGraphicsMatrix_Get(self_, a, b, c, d, tx, ty)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_GetNativeMatrix(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsMatrix_GetNativeMatrix(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_Invert(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsMatrix_Invert(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_IsEqual(self_: *u8 /* void* */, t: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGraphicsMatrix_IsEqual(self_, t)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_IsIdentity(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGraphicsMatrix_IsIdentity(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_Rotate(self_: *u8 /* void* */, angle: c_double /* double */) {
        unsafe {
            wxGraphicsMatrix_Rotate(self_, angle)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_Scale(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */) {
        unsafe {
            wxGraphicsMatrix_Scale(self_, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_Set(self_: *u8 /* void* */, a: c_double /* double */, b: c_double /* double */, c: c_double /* double */, d: c_double /* double */, tx: c_double /* double */, ty: c_double /* double */) {
        unsafe {
            wxGraphicsMatrix_Set(self_, a, b, c, d, tx, ty)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_Translate(self_: *u8 /* void* */, dx: c_double /* double */, dy: c_double /* double */) {
        unsafe {
            wxGraphicsMatrix_Translate(self_, dx, dy)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_TransformPoint(self_: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */) {
        unsafe {
            wxGraphicsMatrix_TransformPoint(self_, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsMatrix_TransformDistance(self_: *u8 /* void* */, dx: *c_double /* double* */, dy: *c_double /* double* */) {
        unsafe {
            wxGraphicsMatrix_TransformDistance(self_, dx, dy)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsObject_GetRenderer() -> *u8 /* void* */ {
        unsafe {
            wxGraphicsObject_GetRenderer()
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsObject_IsNull(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGraphicsObject_IsNull(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_Create() -> *u8 /* void* */ {
        unsafe {
            wxGraphicsPath_Create()
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_Delete(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsPath_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_MoveToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */) {
        unsafe {
            wxGraphicsPath_MoveToPoint(self_, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddArc(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, r: c_double /* double */, startAngle: c_double /* double */, endAngle: c_double /* double */, clockwise: bool /* bool */) {
        unsafe {
            wxGraphicsPath_AddArc(self_, arg0, arg1, r, startAngle, endAngle, clockwise)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddArcToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */, r: c_double /* double */) {
        unsafe {
            wxGraphicsPath_AddArcToPoint(self_, arg0, arg1, arg2, arg3, r)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddCircle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, r: c_double /* double */) {
        unsafe {
            wxGraphicsPath_AddCircle(self_, arg0, arg1, r)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddCurveToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */, arg4: c_double /* double */, arg5: c_double /* double */) {
        unsafe {
            wxGraphicsPath_AddCurveToPoint(self_, arg0, arg1, arg2, arg3, arg4, arg5)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddEllipse(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */) {
        unsafe {
            wxGraphicsPath_AddEllipse(self_, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddLineToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */) {
        unsafe {
            wxGraphicsPath_AddLineToPoint(self_, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddPath(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, path: *u8 /* void* */) {
        unsafe {
            wxGraphicsPath_AddPath(self_, arg0, arg1, path)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddQuadCurveToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */) {
        unsafe {
            wxGraphicsPath_AddQuadCurveToPoint(self_, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */) {
        unsafe {
            wxGraphicsPath_AddRectangle(self_, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_AddRoundedRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */, radius: c_double /* double */) {
        unsafe {
            wxGraphicsPath_AddRoundedRectangle(self_, arg0, arg1, arg2, arg3, radius)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_CloseSubpath(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsPath_CloseSubpath(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_Contains(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, style: c_int /* int */) {
        unsafe {
            wxGraphicsPath_Contains(self_, arg0, arg1, style)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_GetBox(self_: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */, arg2: *c_double /* double* */, arg3: *c_double /* double* */) {
        unsafe {
            wxGraphicsPath_GetBox(self_, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_GetCurrentPoint(self_: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */) {
        unsafe {
            wxGraphicsPath_GetCurrentPoint(self_, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_Transform(self_: *u8 /* void* */, matrix: *u8 /* void* */) {
        unsafe {
            wxGraphicsPath_Transform(self_, matrix)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_GetNativePath(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsPath_GetNativePath(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPath_UnGetNativePath(p: *u8 /* void* */) {
        unsafe {
            wxGraphicsPath_UnGetNativePath(p)
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPen_Create() -> *u8 /* void* */ {
        unsafe {
            wxGraphicsPen_Create()
        }
    }
    #[fixed_stack_segment]
    fn wxGraphicsPen_Delete(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsPen_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxGraphicsRenderer_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultRenderer(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsRenderer_GetDefaultRenderer(self_)
        }
    }
    #[fixed_stack_segment]
    fn CreateContext(dc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsRenderer_CreateContext(dc)
        }
    }
    #[fixed_stack_segment]
    fn CreateContextFromWindow(window: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsRenderer_CreateContextFromWindow(window)
        }
    }
    #[fixed_stack_segment]
    fn CreateContextFromNativeContext(context: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsRenderer_CreateContextFromNativeContext(context)
        }
    }
    #[fixed_stack_segment]
    fn CreateContextFromNativeWindow(window: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGraphicsRenderer_CreateContextFromNativeWindow(window)
        }
    }
}
trait wxCondition {
    #[fixed_stack_segment]
    fn Broadcast(_obj: *u8 /* void* */) {
        unsafe {
            wxCondition_Broadcast(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_mut: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCondition_Create(_mut)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCondition_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Signal(_obj: *u8 /* void* */) {
        unsafe {
            wxCondition_Signal(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Wait(_obj: *u8 /* void* */) {
        unsafe {
            wxCondition_Wait(_obj)
        }
    }
    #[fixed_stack_segment]
    fn WaitFor(_obj: *u8 /* void* */, sec: c_int /* int */, nsec: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxCondition_WaitFor(_obj, sec, nsec)
        }
    }
}
trait wxGridTableBase {
}
trait wxApp {
}
trait wxCommand {
}
trait wxStreamBase {
    #[fixed_stack_segment]
    fn GetLastError(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStreamBase_GetLastError(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStreamBase_GetSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStreamBase_IsOk(_obj)
        }
    }
}
trait wxMDIParentFrame {
    #[fixed_stack_segment]
    fn ActivateNext(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_ActivateNext(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ActivatePrevious(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_ActivatePrevious(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ArrangeIcons(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_ArrangeIcons(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Cascade(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_Cascade(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetActiveChild(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_GetActiveChild(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetClientWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_GetClientWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWindowMenu(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_GetWindowMenu(_obj)
        }
    }
    #[fixed_stack_segment]
    fn OnCreateClient(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMDIParentFrame_OnCreateClient(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetWindowMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_SetWindowMenu(_obj, menu)
        }
    }
    #[fixed_stack_segment]
    fn Tile(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIParentFrame_Tile(_obj)
        }
    }
}
trait wxFilterOutputStream {
}
trait wxClientDC {
    #[fixed_stack_segment]
    fn Create(win: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClientDC_Create(win)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxClientDC_Delete(_obj)
        }
    }
}
trait wxWizardPage {
}
trait wxTextCtrl {
    #[fixed_stack_segment]
    fn AppendText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_AppendText(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn CanCopy(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanCopy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CanCut(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanCut(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CanPaste(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanPaste(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CanRedo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanRedo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CanUndo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_CanUndo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ChangeValue(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_ChangeValue(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Clear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Copy(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Copy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Cut(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Cut(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DiscardEdits(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_DiscardEdits(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetInsertionPoint(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxTextCtrl_GetInsertionPoint(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLastPosition(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxTextCtrl_GetLastPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLineLength(_obj: *u8 /* void* */, lineNo: c_long /* long */) -> c_int /* int */ {
        unsafe {
            wxTextCtrl_GetLineLength(_obj, lineNo)
        }
    }
    #[fixed_stack_segment]
    fn GetLineText(_obj: *u8 /* void* */, lineNo: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetLineText(_obj, lineNo)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberOfLines(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTextCtrl_GetNumberOfLines(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelection(_obj: *u8 /* void* */, from: *u8 /* void* */, to: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_GetSelection(_obj, from, to)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsEditable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_IsEditable(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsModified(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_IsModified(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LoadFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_LoadFile(_obj, file)
        }
    }
    #[fixed_stack_segment]
    fn Paste(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Paste(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PositionToXY(_obj: *u8 /* void* */, pos: c_long /* long */, x: *c_long /* long* */, y: *c_long /* long* */) -> c_int /* int */ {
        unsafe {
            wxTextCtrl_PositionToXY(_obj, pos, x, y)
        }
    }
    #[fixed_stack_segment]
    fn Redo(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Redo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Remove(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */) {
        unsafe {
            wxTextCtrl_Remove(_obj, from, to)
        }
    }
    #[fixed_stack_segment]
    fn Replace(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */, value: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Replace(_obj, from, to, value)
        }
    }
    #[fixed_stack_segment]
    fn SaveFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_SaveFile(_obj, file)
        }
    }
    #[fixed_stack_segment]
    fn SetEditable(_obj: *u8 /* void* */, editable: bool /* bool */) {
        unsafe {
            wxTextCtrl_SetEditable(_obj, editable)
        }
    }
    #[fixed_stack_segment]
    fn SetInsertionPoint(_obj: *u8 /* void* */, pos: c_long /* long */) {
        unsafe {
            wxTextCtrl_SetInsertionPoint(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn SetInsertionPointEnd(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_SetInsertionPointEnd(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetSelection(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */) {
        unsafe {
            wxTextCtrl_SetSelection(_obj, from, to)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, value: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_SetValue(_obj, value)
        }
    }
    #[fixed_stack_segment]
    fn ShowPosition(_obj: *u8 /* void* */, pos: c_long /* long */) {
        unsafe {
            wxTextCtrl_ShowPosition(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn Undo(_obj: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_Undo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn WriteText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxTextCtrl_WriteText(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn XYToPosition(_obj: *u8 /* void* */, arg0: c_long /* long */, arg1: c_long /* long */) -> c_long /* long */ {
        unsafe {
            wxTextCtrl_XYToPosition(_obj, arg0, arg1)
        }
    }
}
trait wxConnection {
}
trait wxTimerEx {
    #[fixed_stack_segment]
    fn Connect(_obj: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxTimerEx_Connect(_obj, closure)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxTimerEx_Create()
        }
    }
    #[fixed_stack_segment]
    fn GetClosure(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTimerEx_GetClosure(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxMenu_AppendRadioItem(self_: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */) {
        unsafe {
            wxMenu_AppendRadioItem(self_, id, text, help)
        }
    }
    #[fixed_stack_segment]
    fn wxMenuItem_CreateSeparator() -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_CreateSeparator()
        }
    }
    #[fixed_stack_segment]
    fn wxMenuItem_CreateEx(id: c_int /* int */, label: *u8 /* void* */, help: *u8 /* void* */, itemkind: c_int /* int */, submenu: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_CreateEx(id, label, help, itemkind, submenu)
        }
    }
    #[fixed_stack_segment]
    fn wxToolBar_AddTool2(_obj: *u8 /* void* */, toolId: c_int /* int */, label: *u8 /* void* */, bmp: *u8 /* void* */, bmpDisabled: *u8 /* void* */, itemKind: c_int /* int */, shortHelp: *u8 /* void* */, longHelp: *u8 /* void* */) {
        unsafe {
            wxToolBar_AddTool2(_obj, toolId, label, bmp, bmpDisabled, itemKind, shortHelp, longHelp)
        }
    }
    #[fixed_stack_segment]
    fn wxProgressDialog_Create(title: *u8 /* void* */, message: *u8 /* void* */, max: c_int /* int */, parent: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxProgressDialog_Create(title, message, max, parent, style)
        }
    }
    #[fixed_stack_segment]
    fn wxProgressDialog_Update(obj: *u8 /* void* */, value: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxProgressDialog_Update(obj, value)
        }
    }
    #[fixed_stack_segment]
    fn wxProgressDialog_UpdateWithMessage(obj: *u8 /* void* */, value: c_int /* int */, message: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProgressDialog_UpdateWithMessage(obj, value, message)
        }
    }
    #[fixed_stack_segment]
    fn wxProgressDialog_Resume(obj: *u8 /* void* */) {
        unsafe {
            wxProgressDialog_Resume(obj)
        }
    }
    #[fixed_stack_segment]
    fn wxVersionNumber() -> c_int /* int */ {
        unsafe {
            wxVersionNumber()
        }
    }
    #[fixed_stack_segment]
    fn wxIsDefined(s: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxIsDefined(s)
        }
    }
}
trait wxFrame {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn CreateStatusBar(_obj: *u8 /* void* */, number: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_CreateStatusBar(_obj, number, style)
        }
    }
    #[fixed_stack_segment]
    fn CreateToolBar(_obj: *u8 /* void* */, style: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_CreateToolBar(_obj, style)
        }
    }
    #[fixed_stack_segment]
    fn GetClientAreaOrigin_left(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFrame_GetClientAreaOrigin_left(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetClientAreaOrigin_top(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFrame_GetClientAreaOrigin_top(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMenuBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_GetMenuBar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStatusBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_GetStatusBar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetToolBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_GetToolBar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Restore(_obj: *u8 /* void* */) {
        unsafe {
            wxFrame_Restore(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetMenuBar(_obj: *u8 /* void* */, menubar: *u8 /* void* */) {
        unsafe {
            wxFrame_SetMenuBar(_obj, menubar)
        }
    }
    #[fixed_stack_segment]
    fn SetStatusBar(_obj: *u8 /* void* */, statBar: *u8 /* void* */) {
        unsafe {
            wxFrame_SetStatusBar(_obj, statBar)
        }
    }
    #[fixed_stack_segment]
    fn SetStatusText(_obj: *u8 /* void* */, _txt: *u8 /* void* */, _number: c_int /* int */) {
        unsafe {
            wxFrame_SetStatusText(_obj, _txt, _number)
        }
    }
    #[fixed_stack_segment]
    fn SetStatusWidths(_obj: *u8 /* void* */, _n: c_int /* int */, _widths_field: *u8 /* void* */) {
        unsafe {
            wxFrame_SetStatusWidths(_obj, _n, _widths_field)
        }
    }
    #[fixed_stack_segment]
    fn SetToolBar(_obj: *u8 /* void* */, _toolbar: *u8 /* void* */) {
        unsafe {
            wxFrame_SetToolBar(_obj, _toolbar)
        }
    }
}
trait wxDateTime {
    #[fixed_stack_segment]
    fn AddDate(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_AddDate(_obj, diff, _ref)
        }
    }
    #[fixed_stack_segment]
    fn AddDateValues(_obj: *u8 /* void* */, _yrs: c_int /* int */, _mnt: c_int /* int */, _wek: c_int /* int */, _day: c_int /* int */) {
        unsafe {
            wxDateTime_AddDateValues(_obj, _yrs, _mnt, _wek, _day)
        }
    }
    #[fixed_stack_segment]
    fn AddTime(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_AddTime(_obj, diff, _ref)
        }
    }
    #[fixed_stack_segment]
    fn AddTimeValues(_obj: *u8 /* void* */, _hrs: c_int /* int */, _min: c_int /* int */, _sec: c_int /* int */, _mls: c_int /* int */) {
        unsafe {
            wxDateTime_AddTimeValues(_obj, _hrs, _min, _sec, _mls)
        }
    }
    #[fixed_stack_segment]
    fn ConvertYearToBC(year: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_ConvertYearToBC(year)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxDateTime_Create()
        }
    }
    #[fixed_stack_segment]
    fn Format(_obj: *u8 /* void* */, format: *u8 /* void* */, tz: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_Format(_obj, format, tz)
        }
    }
    #[fixed_stack_segment]
    fn FormatDate(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_FormatDate(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FormatISODate(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_FormatISODate(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FormatISOTime(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_FormatISOTime(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FormatTime(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_FormatTime(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetAmString() -> *u8 /* void* */ {
        unsafe {
            wxDateTime_GetAmString()
        }
    }
    #[fixed_stack_segment]
    fn GetBeginDST(year: c_int /* int */, country: c_int /* int */, dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetBeginDST(year, country, dt)
        }
    }
    #[fixed_stack_segment]
    fn GetCentury(year: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetCentury(year)
        }
    }
    #[fixed_stack_segment]
    fn GetCountry() -> c_int /* int */ {
        unsafe {
            wxDateTime_GetCountry()
        }
    }
    #[fixed_stack_segment]
    fn GetCurrentMonth(cal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetCurrentMonth(cal)
        }
    }
    #[fixed_stack_segment]
    fn GetCurrentYear(cal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetCurrentYear(cal)
        }
    }
    #[fixed_stack_segment]
    fn GetDay(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetDay(_obj, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetDayOfYear(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetDayOfYear(_obj, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetEndDST(year: c_int /* int */, country: c_int /* int */, dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetEndDST(year, country, dt)
        }
    }
    #[fixed_stack_segment]
    fn GetHour(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetHour(_obj, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetLastMonthDay(_obj: *u8 /* void* */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetLastMonthDay(_obj, month, year, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetLastWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetLastWeekDay(_obj, weekday, month, year, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetMillisecond(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetMillisecond(_obj, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetMinute(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetMinute(_obj, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetMonth(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetMonth(_obj, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetMonthName(month: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_GetMonthName(month, flags)
        }
    }
    #[fixed_stack_segment]
    fn GetNextWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetNextWeekDay(_obj, weekday, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberOfDays(year: c_int /* int */, cal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetNumberOfDays(year, cal)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberOfDaysMonth(month: c_int /* int */, year: c_int /* int */, cal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetNumberOfDaysMonth(month, year, cal)
        }
    }
    #[fixed_stack_segment]
    fn GetPmString() -> *u8 /* void* */ {
        unsafe {
            wxDateTime_GetPmString()
        }
    }
    #[fixed_stack_segment]
    fn GetPrevWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetPrevWeekDay(_obj, weekday, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetSecond(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetSecond(_obj, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetTicks(_obj: *u8 /* void* */) -> time_t /* time_t */ {
        unsafe {
            wxDateTime_GetTicks(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTimeNow() -> c_int /* int */ {
        unsafe {
            wxDateTime_GetTimeNow()
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */, hi_long: *u8 /* void* */, lo_long: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetValue(_obj, hi_long, lo_long)
        }
    }
    #[fixed_stack_segment]
    fn GetWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, n: c_int /* int */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetWeekDay(_obj, weekday, n, month, year, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetWeekDayInSameWeek(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_GetWeekDayInSameWeek(_obj, weekday, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetWeekDayName(weekday: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_GetWeekDayName(weekday, flags)
        }
    }
    #[fixed_stack_segment]
    fn GetWeekDayTZ(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetWeekDayTZ(_obj, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetWeekOfMonth(_obj: *u8 /* void* */, flags: c_int /* int */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetWeekOfMonth(_obj, flags, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetWeekOfYear(_obj: *u8 /* void* */, flags: c_int /* int */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetWeekOfYear(_obj, flags, tz)
        }
    }
    #[fixed_stack_segment]
    fn GetYear(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDateTime_GetYear(_obj, tz)
        }
    }
    #[fixed_stack_segment]
    fn IsBetween(_obj: *u8 /* void* */, t1: *u8 /* void* */, t2: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsBetween(_obj, t1, t2)
        }
    }
    #[fixed_stack_segment]
    fn IsDST(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsDST(_obj, country)
        }
    }
    #[fixed_stack_segment]
    fn IsDSTApplicable(year: c_int /* int */, country: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsDSTApplicable(year, country)
        }
    }
    #[fixed_stack_segment]
    fn IsEarlierThan(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsEarlierThan(_obj, datetime)
        }
    }
    #[fixed_stack_segment]
    fn IsEqualTo(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsEqualTo(_obj, datetime)
        }
    }
    #[fixed_stack_segment]
    fn IsEqualUpTo(_obj: *u8 /* void* */, dt: *u8 /* void* */, ts: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsEqualUpTo(_obj, dt, ts)
        }
    }
    #[fixed_stack_segment]
    fn IsGregorianDate(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsGregorianDate(_obj, country)
        }
    }
    #[fixed_stack_segment]
    fn IsLaterThan(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsLaterThan(_obj, datetime)
        }
    }
    #[fixed_stack_segment]
    fn IsLeapYear(year: c_int /* int */, cal: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsLeapYear(year, cal)
        }
    }
    #[fixed_stack_segment]
    fn IsSameDate(_obj: *u8 /* void* */, dt: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsSameDate(_obj, dt)
        }
    }
    #[fixed_stack_segment]
    fn IsSameTime(_obj: *u8 /* void* */, dt: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsSameTime(_obj, dt)
        }
    }
    #[fixed_stack_segment]
    fn IsStrictlyBetween(_obj: *u8 /* void* */, t1: *u8 /* void* */, t2: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsStrictlyBetween(_obj, t1, t2)
        }
    }
    #[fixed_stack_segment]
    fn IsValid(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsValid(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsWestEuropeanCountry(country: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsWestEuropeanCountry(country)
        }
    }
    #[fixed_stack_segment]
    fn IsWorkDay(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_IsWorkDay(_obj, country)
        }
    }
    #[fixed_stack_segment]
    fn MakeGMT(_obj: *u8 /* void* */, noDST: c_int /* int */) {
        unsafe {
            wxDateTime_MakeGMT(_obj, noDST)
        }
    }
    #[fixed_stack_segment]
    fn MakeTimezone(_obj: *u8 /* void* */, tz: c_int /* int */, noDST: c_int /* int */) {
        unsafe {
            wxDateTime_MakeTimezone(_obj, tz, noDST)
        }
    }
    #[fixed_stack_segment]
    fn Now(dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_Now(dt)
        }
    }
    #[fixed_stack_segment]
    fn ParseDate(_obj: *u8 /* void* */, date: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseDate(_obj, date)
        }
    }
    #[fixed_stack_segment]
    fn ParseDateTime(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseDateTime(_obj, datetime)
        }
    }
    #[fixed_stack_segment]
    fn ParseFormat(_obj: *u8 /* void* */, date: *u8 /* void* */, format: *u8 /* void* */, dateDef: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseFormat(_obj, date, format, dateDef)
        }
    }
    #[fixed_stack_segment]
    fn ParseRfc822Date(_obj: *u8 /* void* */, date: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseRfc822Date(_obj, date)
        }
    }
    #[fixed_stack_segment]
    fn ParseTime(_obj: *u8 /* void* */, time: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_ParseTime(_obj, time)
        }
    }
    #[fixed_stack_segment]
    fn ResetTime(_obj: *u8 /* void* */) {
        unsafe {
            wxDateTime_ResetTime(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Set(_obj: *u8 /* void* */, day: c_int /* int */, month: c_int /* int */, year: c_int /* int */, hour: c_int /* int */, minute: c_int /* int */, second: c_int /* int */, millisec: c_int /* int */) {
        unsafe {
            wxDateTime_Set(_obj, day, month, year, hour, minute, second, millisec)
        }
    }
    #[fixed_stack_segment]
    fn SetCountry(country: c_int /* int */) {
        unsafe {
            wxDateTime_SetCountry(country)
        }
    }
    #[fixed_stack_segment]
    fn SetDay(_obj: *u8 /* void* */, day: c_int /* int */) {
        unsafe {
            wxDateTime_SetDay(_obj, day)
        }
    }
    #[fixed_stack_segment]
    fn SetHour(_obj: *u8 /* void* */, hour: c_int /* int */) {
        unsafe {
            wxDateTime_SetHour(_obj, hour)
        }
    }
    #[fixed_stack_segment]
    fn SetMillisecond(_obj: *u8 /* void* */, millisecond: c_int /* int */) {
        unsafe {
            wxDateTime_SetMillisecond(_obj, millisecond)
        }
    }
    #[fixed_stack_segment]
    fn SetMinute(_obj: *u8 /* void* */, minute: c_int /* int */) {
        unsafe {
            wxDateTime_SetMinute(_obj, minute)
        }
    }
    #[fixed_stack_segment]
    fn SetMonth(_obj: *u8 /* void* */, month: c_int /* int */) {
        unsafe {
            wxDateTime_SetMonth(_obj, month)
        }
    }
    #[fixed_stack_segment]
    fn SetSecond(_obj: *u8 /* void* */, second: c_int /* int */) {
        unsafe {
            wxDateTime_SetSecond(_obj, second)
        }
    }
    #[fixed_stack_segment]
    fn SetTime(_obj: *u8 /* void* */, hour: c_int /* int */, minute: c_int /* int */, second: c_int /* int */, millisec: c_int /* int */) {
        unsafe {
            wxDateTime_SetTime(_obj, hour, minute, second, millisec)
        }
    }
    #[fixed_stack_segment]
    fn SetToCurrent(_obj: *u8 /* void* */) {
        unsafe {
            wxDateTime_SetToCurrent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetToLastMonthDay(_obj: *u8 /* void* */, month: c_int /* int */, year: c_int /* int */) {
        unsafe {
            wxDateTime_SetToLastMonthDay(_obj, month, year)
        }
    }
    #[fixed_stack_segment]
    fn SetToLastWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, month: c_int /* int */, year: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_SetToLastWeekDay(_obj, weekday, month, year)
        }
    }
    #[fixed_stack_segment]
    fn SetToNextWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */) {
        unsafe {
            wxDateTime_SetToNextWeekDay(_obj, weekday)
        }
    }
    #[fixed_stack_segment]
    fn SetToPrevWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */) {
        unsafe {
            wxDateTime_SetToPrevWeekDay(_obj, weekday)
        }
    }
    #[fixed_stack_segment]
    fn SetToWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, n: c_int /* int */, month: c_int /* int */, year: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDateTime_SetToWeekDay(_obj, weekday, n, month, year)
        }
    }
    #[fixed_stack_segment]
    fn SetToWeekDayInSameWeek(_obj: *u8 /* void* */, weekday: c_int /* int */) {
        unsafe {
            wxDateTime_SetToWeekDayInSameWeek(_obj, weekday)
        }
    }
    #[fixed_stack_segment]
    fn SetYear(_obj: *u8 /* void* */, year: c_int /* int */) {
        unsafe {
            wxDateTime_SetYear(_obj, year)
        }
    }
    #[fixed_stack_segment]
    fn SubtractDate(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_SubtractDate(_obj, diff, _ref)
        }
    }
    #[fixed_stack_segment]
    fn SubtractTime(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxDateTime_SubtractTime(_obj, diff, _ref)
        }
    }
    #[fixed_stack_segment]
    fn ToGMT(_obj: *u8 /* void* */, noDST: c_int /* int */) {
        unsafe {
            wxDateTime_ToGMT(_obj, noDST)
        }
    }
    #[fixed_stack_segment]
    fn ToTimezone(_obj: *u8 /* void* */, tz: c_int /* int */, noDST: c_int /* int */) {
        unsafe {
            wxDateTime_ToTimezone(_obj, tz, noDST)
        }
    }
    #[fixed_stack_segment]
    fn Today(dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_Today(dt)
        }
    }
    #[fixed_stack_segment]
    fn UNow(dt: *u8 /* void* */) {
        unsafe {
            wxDateTime_UNow(dt)
        }
    }
    #[fixed_stack_segment]
    fn wxDateTime(hi_long: c_int /* int */, lo_long: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDateTime_wxDateTime(hi_long, lo_long)
        }
    }
}
trait ELJGridTable {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _EifGetNumberRows: *u8 /* void* */, _EifGetNumberCols: *u8 /* void* */, _EifGetValue: *u8 /* void* */, _EifSetValue: *u8 /* void* */, _EifIsEmptyCell: *u8 /* void* */, _EifClear: *u8 /* void* */, _EifInsertRows: *u8 /* void* */, _EifAppendRows: *u8 /* void* */, _EifDeleteRows: *u8 /* void* */, _EifInsertCols: *u8 /* void* */, _EifAppendCols: *u8 /* void* */, _EifDeleteCols: *u8 /* void* */, _EifSetRowLabelValue: *u8 /* void* */, _EifSetColLabelValue: *u8 /* void* */, _EifGetRowLabelValue: *u8 /* void* */, _EifGetColLabelValue: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJGridTable_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetView(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJGridTable_GetView(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SendTableMessage(_obj: *u8 /* void* */, id: c_int /* int */, val1: c_int /* int */, val2: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJGridTable_SendTableMessage(_obj, id, val1, val2)
        }
    }
}
trait wxTabCtrl {
}
trait wxQueryNewPaletteEvent {
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxQueryNewPaletteEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPaletteRealized(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxQueryNewPaletteEvent_GetPaletteRealized(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetPaletteRealized(_obj: *u8 /* void* */, realized: bool /* bool */) {
        unsafe {
            wxQueryNewPaletteEvent_SetPaletteRealized(_obj, realized)
        }
    }
}
trait wxBitmapButton {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmapButton_Create(_prt, _id, _bmp, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetBitmapDisabled(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_GetBitmapDisabled(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetBitmapFocus(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_GetBitmapFocus(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetBitmapLabel(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_GetBitmapLabel(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetBitmapSelected(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_GetBitmapSelected(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetMarginX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmapButton_GetMarginX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMarginY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmapButton_GetMarginY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetBitmapDisabled(_obj: *u8 /* void* */, disabled: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_SetBitmapDisabled(_obj, disabled)
        }
    }
    #[fixed_stack_segment]
    fn SetBitmapFocus(_obj: *u8 /* void* */, focus: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_SetBitmapFocus(_obj, focus)
        }
    }
    #[fixed_stack_segment]
    fn SetBitmapLabel(_obj: *u8 /* void* */, bitmap: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_SetBitmapLabel(_obj, bitmap)
        }
    }
    #[fixed_stack_segment]
    fn SetBitmapSelected(_obj: *u8 /* void* */, sel: *u8 /* void* */) {
        unsafe {
            wxBitmapButton_SetBitmapSelected(_obj, sel)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Dc(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawRowBkGroundEvent_Dc(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Row(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawRowBkGroundEvent_Row(_obj)
        }
    }
}
trait wxHtmlTagHandler {
}
trait wxSizerItem {
    #[fixed_stack_segment]
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_CalcMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_Create(arg0, arg1, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn CreateInSizer(sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_CreateInSizer(sizer, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn CreateInWindow(window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_CreateInWindow(window, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn GetBorder(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSizerItem_GetBorder(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFlag(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSizerItem_GetFlag(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetMinSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRatio(_obj: *u8 /* void* */) -> c_float /* float */ {
        unsafe {
            wxSizerItem_GetRatio(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSizer(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetSizer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetUserData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetUserData(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsSizer(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizerItem_IsSizer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsSpacer(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizerItem_IsSpacer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizerItem_IsWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetBorder(_obj: *u8 /* void* */, border: c_int /* int */) {
        unsafe {
            wxSizerItem_SetBorder(_obj, border)
        }
    }
    #[fixed_stack_segment]
    fn SetDimension(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxSizerItem_SetDimension(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn SetFlag(_obj: *u8 /* void* */, flag: c_int /* int */) {
        unsafe {
            wxSizerItem_SetFlag(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn SetFloatRatio(_obj: *u8 /* void* */, ratio: c_float /* float */) {
        unsafe {
            wxSizerItem_SetFloatRatio(_obj, ratio)
        }
    }
    #[fixed_stack_segment]
    fn SetInitSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizerItem_SetInitSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetRatio(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizerItem_SetRatio(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) {
        unsafe {
            wxSizerItem_SetSizer(_obj, sizer)
        }
    }
    #[fixed_stack_segment]
    fn SetWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizerItem_SetWindow(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxSizerItem_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DeleteWindows(_obj: *u8 /* void* */) {
        unsafe {
            wxSizerItem_DeleteWindows(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DetachSizer(_obj: *u8 /* void* */) {
        unsafe {
            wxSizerItem_DetachSizer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetProportion(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSizerItem_GetProportion(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetRect(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSpacer(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizerItem_GetSpacer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsShown(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSizerItem_IsShown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetProportion(_obj: *u8 /* void* */, proportion: c_int /* int */) {
        unsafe {
            wxSizerItem_SetProportion(_obj, proportion)
        }
    }
    #[fixed_stack_segment]
    fn SetSpacer(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizerItem_SetSpacer(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Show(_obj: *u8 /* void* */, show: c_int /* int */) {
        unsafe {
            wxSizerItem_Show(_obj, show)
        }
    }
}
trait wxMenuBar {
    #[fixed_stack_segment]
    fn Append(_obj: *u8 /* void* */, menu: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_Append(_obj, menu, title)
        }
    }
    #[fixed_stack_segment]
    fn Check(_obj: *u8 /* void* */, id: c_int /* int */, check: bool /* bool */) {
        unsafe {
            wxMenuBar_Check(_obj, id, check)
        }
    }
    #[fixed_stack_segment]
    fn Create(_style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_Create(_style)
        }
    }
    #[fixed_stack_segment]
    fn DeletePointer(_obj: *u8 /* void* */) {
        unsafe {
            wxMenuBar_DeletePointer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_Enable(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn EnableItem(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxMenuBar_EnableItem(_obj, id, enable)
        }
    }
    #[fixed_stack_segment]
    fn EnableTop(_obj: *u8 /* void* */, pos: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxMenuBar_EnableTop(_obj, pos, enable)
        }
    }
    #[fixed_stack_segment]
    fn FindItem(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_FindItem(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn FindMenu(_obj: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_FindMenu(_obj, title)
        }
    }
    #[fixed_stack_segment]
    fn FindMenuItem(_obj: *u8 /* void* */, menuString: *u8 /* void* */, itemString: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_FindMenuItem(_obj, menuString, itemString)
        }
    }
    #[fixed_stack_segment]
    fn GetHelpString(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetHelpString(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn GetLabel(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetLabel(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn GetLabelTop(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetLabelTop(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn GetMenu(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetMenu(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn GetMenuCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_GetMenuCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Insert(_obj: *u8 /* void* */, pos: c_int /* int */, menu: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuBar_Insert(_obj, pos, menu, title)
        }
    }
    #[fixed_stack_segment]
    fn IsChecked(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMenuBar_IsChecked(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn IsEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMenuBar_IsEnabled(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn Remove(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_Remove(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn Replace(_obj: *u8 /* void* */, pos: c_int /* int */, menu: *u8 /* void* */, title: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_Replace(_obj, pos, menu, title)
        }
    }
    #[fixed_stack_segment]
    fn SetHelpString(_obj: *u8 /* void* */, id: c_int /* int */, helpString: *u8 /* void* */) {
        unsafe {
            wxMenuBar_SetHelpString(_obj, id, helpString)
        }
    }
    #[fixed_stack_segment]
    fn SetItemLabel(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxMenuBar_SetItemLabel(_obj, id, label)
        }
    }
    #[fixed_stack_segment]
    fn SetLabel(_obj: *u8 /* void* */, s: *u8 /* void* */) {
        unsafe {
            wxMenuBar_SetLabel(_obj, s)
        }
    }
    #[fixed_stack_segment]
    fn SetLabelTop(_obj: *u8 /* void* */, pos: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxMenuBar_SetLabelTop(_obj, pos, label)
        }
    }
}
trait cbCloseBox {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            cbCloseBox_Create()
        }
    }
}
trait wxMetafileDC {
    #[fixed_stack_segment]
    fn Close(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMetafileDC_Close(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_file: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMetafileDC_Create(_file)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMetafileDC_Delete(_obj)
        }
    }
}
trait wxDataOutputStream {
}
trait wxGridCellEditor {
    #[fixed_stack_segment]
    fn BeginEdit(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, grid: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_BeginEdit(_obj, row, col, grid)
        }
    }
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, parent: *u8 /* void* */, id: c_int /* int */, evtHandler: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_Create(_obj, parent, id, evtHandler)
        }
    }
    #[fixed_stack_segment]
    fn Destroy(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_Destroy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EndEdit(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, grid: *u8 /* void* */, oldStr: *u8 /* void* */, newStr: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridCellEditor_EndEdit(_obj, row, col, grid, oldStr, newStr)
        }
    }
    #[fixed_stack_segment]
    fn GetControl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellEditor_GetControl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HandleReturn(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_HandleReturn(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn IsAcceptedKey(_obj: *u8 /* void* */, event: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellEditor_IsAcceptedKey(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn IsCreated(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellEditor_IsCreated(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PaintBackground(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_PaintBackground(_obj, arg0, arg1, arg2, arg3, attr)
        }
    }
    #[fixed_stack_segment]
    fn Reset(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_Reset(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetControl(_obj: *u8 /* void* */, control: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_SetControl(_obj, control)
        }
    }
    #[fixed_stack_segment]
    fn SetParameters(_obj: *u8 /* void* */, params: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_SetParameters(_obj, params)
        }
    }
    #[fixed_stack_segment]
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxGridCellEditor_SetSize(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn Show(_obj: *u8 /* void* */, show: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_Show(_obj, show, attr)
        }
    }
    #[fixed_stack_segment]
    fn StartingClick(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_StartingClick(_obj)
        }
    }
    #[fixed_stack_segment]
    fn StartingKey(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxGridCellEditor_StartingKey(_obj, event)
        }
    }
}
trait wxBufferedDC {
    #[fixed_stack_segment]
    fn CreateByDCAndSize(dc: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBufferedDC_CreateByDCAndSize(dc, arg0, arg1, style)
        }
    }
    #[fixed_stack_segment]
    fn CreateByDCAndBitmap(dc: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBufferedDC_CreateByDCAndBitmap(dc, bitmap, style)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxBufferedDC_Delete(self_)
        }
    }
}
trait wxLogPassThrough {
}
trait wxTimer {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTimer_Create(_prt, _id)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxTimer_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetInterval(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTimer_GetInterval(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOneShot(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTimer_IsOneShot(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsRuning(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTimer_IsRuning(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Start(_obj: *u8 /* void* */, _int: c_int /* int */, _one: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxTimer_Start(_obj, _int, _one)
        }
    }
    #[fixed_stack_segment]
    fn Stop(_obj: *u8 /* void* */) {
        unsafe {
            wxTimer_Stop(_obj)
        }
    }
}
trait ELJArtProv {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _clb: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJArtProv_Create(_obj, _clb)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn GetDirection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWizardEvent_GetDirection(_obj)
        }
    }
}
trait wxMediaEvent {
}
trait wxSplitterScrolledWindow {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSplitterScrolledWindow_Create(parent, id, arg0, arg1, arg2, arg3, style)
        }
    }
}
trait wxDocManager {
}
trait wxDatabase {
}
trait wxPGProperty {
    #[fixed_stack_segment]
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPGProperty_GetLabel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPGProperty_GetName(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValueAsString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPGProperty_GetValueAsString(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValueType(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPGProperty_GetValueType(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetHelpString(_obj: *u8 /* void* */, helpString: *u8 /* void* */) {
        unsafe {
            wxPGProperty_SetHelpString(_obj, helpString)
        }
    }
}
trait wxList {
}
trait wxDynamicToolBar {
    #[fixed_stack_segment]
    fn AddSeparator(_obj: *u8 /* void* */, pSepartorWnd: *u8 /* void* */) {
        unsafe {
            wxDynamicToolBar_AddSeparator(_obj, pSepartorWnd)
        }
    }
    #[fixed_stack_segment]
    fn AddTool(_obj: *u8 /* void* */, toolIndex: c_int /* int */, pToolWindow: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxDynamicToolBar_AddTool(_obj, toolIndex, pToolWindow, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn AddToolBitmap(_obj: *u8 /* void* */, toolIndex: c_int /* int */, bitmap: *u8 /* void* */, pushedBitmap: *u8 /* void* */, toggle: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, clientData: *u8 /* void* */, helpString1: *u8 /* void* */, helpString2: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicToolBar_AddToolBitmap(_obj, toolIndex, bitmap, pushedBitmap, toggle, arg0, arg1, clientData, helpString1, helpString2)
        }
    }
    #[fixed_stack_segment]
    fn AddToolImage(_obj: *u8 /* void* */, toolIndex: c_int /* int */, imageFileName: *u8 /* void* */, imageFileType: c_int /* int */, labelText: *u8 /* void* */, alignTextRight: c_int /* int */, isFlat: bool /* bool */) {
        unsafe {
            wxDynamicToolBar_AddToolImage(_obj, toolIndex, imageFileName, imageFileType, labelText, alignTextRight, isFlat)
        }
    }
    #[fixed_stack_segment]
    fn AddToolLabel(_obj: *u8 /* void* */, toolIndex: c_int /* int */, labelBmp: *u8 /* void* */, labelText: *u8 /* void* */, alignTextRight: c_int /* int */, isFlat: bool /* bool */) {
        unsafe {
            wxDynamicToolBar_AddToolLabel(_obj, toolIndex, labelBmp, labelText, alignTextRight, isFlat)
        }
    }
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */, orientation: c_int /* int */, RowsOrColumns: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicToolBar_Create(parent, id, arg0, arg1, arg2, arg3, style, orientation, RowsOrColumns)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxDynamicToolBar_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateDefaultLayout(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicToolBar_CreateDefaultLayout(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateParams(_obj: *u8 /* void* */, parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */, orientation: c_int /* int */, RowsOrColumns: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxDynamicToolBar_CreateParams(_obj, parent, id, arg0, arg1, arg2, arg3, style, orientation, RowsOrColumns)
        }
    }
    #[fixed_stack_segment]
    fn CreateTool(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */, bmpNormal: *u8 /* void* */, bmpDisabled: *u8 /* void* */, kind: c_int /* int */, clientData: *u8 /* void* */, shortHelp: *u8 /* void* */, longHelp: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicToolBar_CreateTool(_obj, id, label, bmpNormal, bmpDisabled, kind, clientData, shortHelp, longHelp)
        }
    }
    #[fixed_stack_segment]
    fn CreateToolControl(_obj: *u8 /* void* */, control: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicToolBar_CreateToolControl(_obj, control)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDynamicToolBar_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DoDeleteTool(_obj: *u8 /* void* */, pos: c_int /* int */, tool: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDynamicToolBar_DoDeleteTool(_obj, pos, tool)
        }
    }
    #[fixed_stack_segment]
    fn DoEnableTool(_obj: *u8 /* void* */, tool: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxDynamicToolBar_DoEnableTool(_obj, tool, enable)
        }
    }
    #[fixed_stack_segment]
    fn DoInsertTool(_obj: *u8 /* void* */, pos: c_int /* int */, tool: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDynamicToolBar_DoInsertTool(_obj, pos, tool)
        }
    }
    #[fixed_stack_segment]
    fn DoSetToggle(_obj: *u8 /* void* */, tool: *u8 /* void* */, toggle: c_int /* int */) {
        unsafe {
            wxDynamicToolBar_DoSetToggle(_obj, tool, toggle)
        }
    }
    #[fixed_stack_segment]
    fn DoToggleTool(_obj: *u8 /* void* */, tool: *u8 /* void* */, toggle: c_int /* int */) {
        unsafe {
            wxDynamicToolBar_DoToggleTool(_obj, tool, toggle)
        }
    }
    #[fixed_stack_segment]
    fn DrawSeparator(_obj: *u8 /* void* */, info: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxDynamicToolBar_DrawSeparator(_obj, info, dc)
        }
    }
    #[fixed_stack_segment]
    fn EnableTool(_obj: *u8 /* void* */, toolIndex: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxDynamicToolBar_EnableTool(_obj, toolIndex, enable)
        }
    }
    #[fixed_stack_segment]
    fn FindToolForPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicToolBar_FindToolForPosition(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetPreferredDim(_obj: *u8 /* void* */, gw: c_int /* int */, gh: c_int /* int */, pw: *u8 /* void* */, ph: *u8 /* void* */) {
        unsafe {
            wxDynamicToolBar_GetPreferredDim(_obj, gw, gh, pw, ph)
        }
    }
    #[fixed_stack_segment]
    fn GetToolInfo(_obj: *u8 /* void* */, toolIndex: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicToolBar_GetToolInfo(_obj, toolIndex)
        }
    }
    #[fixed_stack_segment]
    fn Layout(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDynamicToolBar_Layout(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RemoveTool(_obj: *u8 /* void* */, toolIndex: c_int /* int */) {
        unsafe {
            wxDynamicToolBar_RemoveTool(_obj, toolIndex)
        }
    }
    #[fixed_stack_segment]
    fn SetLayout(_obj: *u8 /* void* */, pLayout: *u8 /* void* */) {
        unsafe {
            wxDynamicToolBar_SetLayout(_obj, pLayout)
        }
    }
}
trait wxDirDialog {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _dir: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDirDialog_Create(_prt, _msg, _dir, arg0, arg1, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetMessage(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDirDialog_GetMessage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDirDialog_GetPath(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDirDialog_GetStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetMessage(_obj: *u8 /* void* */, msg: *u8 /* void* */) {
        unsafe {
            wxDirDialog_SetMessage(_obj, msg)
        }
    }
    #[fixed_stack_segment]
    fn SetPath(_obj: *u8 /* void* */, pth: *u8 /* void* */) {
        unsafe {
            wxDirDialog_SetPath(_obj, pth)
        }
    }
    #[fixed_stack_segment]
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxDirDialog_SetStyle(_obj, style)
        }
    }
}
trait cbCustomizeBarEvent {
    #[fixed_stack_segment]
    fn Bar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbCustomizeBarEvent_Bar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ClickPos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbCustomizeBarEvent_ClickPos(_obj, arg0, arg1)
        }
    }
}
trait wxArray {
}
trait wxDialUpEvent {
    #[fixed_stack_segment]
    fn IsConnectedEvent(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialUpEvent_IsConnectedEvent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOwnEvent(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDialUpEvent_IsOwnEvent(_obj)
        }
    }
}
trait cbCommonPaneProperties {
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, _other: *u8 /* void* */) {
        unsafe {
            cbCommonPaneProperties_Assign(_obj, _other)
        }
    }
    #[fixed_stack_segment]
    fn BarCollapseIconsOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_BarCollapseIconsOn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn BarDragHintsOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_BarDragHintsOn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn BarFloatingOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_BarFloatingOn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ColProportionsOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_ColProportionsOn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbCommonPaneProperties_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbCommonPaneProperties_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ExactDockPredictionOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_ExactDockPredictionOn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MinCBarDim(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbCommonPaneProperties_MinCBarDim(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn NonDestructFrictionOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_NonDestructFrictionOn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn OutOfPaneDragOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_OutOfPaneDragOn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RealTimeUpdatesOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_RealTimeUpdatesOn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ResizeHandleSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_ResizeHandleSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RowProportionsOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_RowProportionsOn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetBarCollapseIconsOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetBarCollapseIconsOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetBarDragHintsOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetBarDragHintsOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetBarFloatingOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetBarFloatingOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetColProportionsOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetColProportionsOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetExactDockPredictionOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetExactDockPredictionOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetMinCBarDim(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetMinCBarDim(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetNonDestructFrictionOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetNonDestructFrictionOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetOutOfPaneDragOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetOutOfPaneDragOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetRealTimeUpdatesOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetRealTimeUpdatesOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetResizeHandleSize(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetResizeHandleSize(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetRowProportionsOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetRowProportionsOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn SetShow3DPaneBorderOn(_obj: *u8 /* void* */, _val: c_int /* int */) {
        unsafe {
            cbCommonPaneProperties_SetShow3DPaneBorderOn(_obj, _val)
        }
    }
    #[fixed_stack_segment]
    fn Show3DPaneBorderOn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbCommonPaneProperties_Show3DPaneBorderOn(_obj)
        }
    }
}
trait wxDirTraverser {
}
trait wxColourData {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxColourData_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxColourData_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetChooseFull(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxColourData_GetChooseFull(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxColourData_GetColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxColourData_GetCustomColour(_obj, i, _ref)
        }
    }
    #[fixed_stack_segment]
    fn SetChooseFull(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxColourData_SetChooseFull(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn SetColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxColourData_SetColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxColourData_SetCustomColour(_obj, i, colour)
        }
    }
}
trait wxTaskBarIcon {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxTaskBarIcon_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxTaskBarIcon_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsIconInstalled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_IsIconInstalled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PopupMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_PopupMenu(_obj, menu)
        }
    }
    #[fixed_stack_segment]
    fn RemoveIcon(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_RemoveIcon(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */, text: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTaskBarIcon_SetIcon(_obj, icon, text)
        }
    }
}
trait wxStaticBoxSizer {
    #[fixed_stack_segment]
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBoxSizer_CalcMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(box: *u8 /* void* */, orient: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBoxSizer_Create(box, orient)
        }
    }
    #[fixed_stack_segment]
    fn GetStaticBox(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBoxSizer_GetStaticBox(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxStaticBoxSizer_RecalcSizes(_obj)
        }
    }
}
trait wxTime {
}
trait wxMouseEvent {
    #[fixed_stack_segment]
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_AltDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Button(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Button(_obj, but)
        }
    }
    #[fixed_stack_segment]
    fn ButtonDClick(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ButtonDClick(_obj, but)
        }
    }
    #[fixed_stack_segment]
    fn ButtonDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ButtonDown(_obj, but)
        }
    }
    #[fixed_stack_segment]
    fn ButtonIsDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ButtonIsDown(_obj, but)
        }
    }
    #[fixed_stack_segment]
    fn ButtonUp(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ButtonUp(_obj, but)
        }
    }
    #[fixed_stack_segment]
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ControlDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxMouseEvent_CopyObject(_obj, object_dest)
        }
    }
    #[fixed_stack_segment]
    fn Dragging(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Dragging(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Entering(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Entering(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLogicalPosition(_obj: *u8 /* void* */, dc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMouseEvent_GetLogicalPosition(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMouseEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsButton(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_IsButton(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Leaving(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Leaving(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LeftDClick(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_LeftDClick(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LeftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_LeftDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LeftIsDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_LeftIsDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LeftUp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_LeftUp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MetaDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MiddleDClick(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MiddleDClick(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MiddleDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MiddleDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MiddleIsDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MiddleIsDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MiddleUp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_MiddleUp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Moving(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_Moving(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RightDClick(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_RightDClick(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RightDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_RightDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RightIsDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_RightIsDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RightUp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_RightUp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMouseEvent_ShiftDown(_obj)
        }
    }
}
trait ELJApp {
    #[fixed_stack_segment]
    fn Bell() {
        unsafe {
            ELJApp_Bell()
        }
    }
    #[fixed_stack_segment]
    fn CreateLogTarget() -> *u8 /* void* */ {
        unsafe {
            ELJApp_CreateLogTarget()
        }
    }
    #[fixed_stack_segment]
    fn Dispatch() {
        unsafe {
            ELJApp_Dispatch()
        }
    }
    #[fixed_stack_segment]
    fn DisplaySize() -> *u8 /* void* */ {
        unsafe {
            ELJApp_DisplaySize()
        }
    }
    #[fixed_stack_segment]
    fn EnableTooltips(_enable: bool /* bool */) {
        unsafe {
            ELJApp_EnableTooltips(_enable)
        }
    }
    #[fixed_stack_segment]
    fn EnableTopLevelWindows(_enb: c_int /* int */) {
        unsafe {
            ELJApp_EnableTopLevelWindows(_enb)
        }
    }
    #[fixed_stack_segment]
    fn ExecuteProcess(_cmd: *u8 /* void* */, _snc: c_int /* int */, _prc: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJApp_ExecuteProcess(_cmd, _snc, _prc)
        }
    }
    #[fixed_stack_segment]
    fn Exit() {
        unsafe {
            ELJApp_Exit()
        }
    }
    #[fixed_stack_segment]
    fn ExitMainLoop() {
        unsafe {
            ELJApp_ExitMainLoop()
        }
    }
    #[fixed_stack_segment]
    fn FindWindowById(_id: c_int /* int */, _prt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJApp_FindWindowById(_id, _prt)
        }
    }
    #[fixed_stack_segment]
    fn FindWindowByLabel(_lbl: *u8 /* void* */, _prt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJApp_FindWindowByLabel(_lbl, _prt)
        }
    }
    #[fixed_stack_segment]
    fn FindWindowByName(_lbl: *u8 /* void* */, _prt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJApp_FindWindowByName(_lbl, _prt)
        }
    }
    #[fixed_stack_segment]
    fn GetApp() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetApp()
        }
    }
    #[fixed_stack_segment]
    fn GetAppName() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetAppName()
        }
    }
    #[fixed_stack_segment]
    fn GetClassName() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetClassName()
        }
    }
    #[fixed_stack_segment]
    fn GetExitOnFrameDelete() -> c_int /* int */ {
        unsafe {
            ELJApp_GetExitOnFrameDelete()
        }
    }
    #[fixed_stack_segment]
    fn GetOsDescription() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetOsDescription()
        }
    }
    #[fixed_stack_segment]
    fn GetOsVersion(_maj: *u8 /* void* */, _min: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJApp_GetOsVersion(_maj, _min)
        }
    }
    #[fixed_stack_segment]
    fn GetTopWindow() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetTopWindow()
        }
    }
    #[fixed_stack_segment]
    fn GetUseBestVisual() -> c_int /* int */ {
        unsafe {
            ELJApp_GetUseBestVisual()
        }
    }
    #[fixed_stack_segment]
    fn GetUserHome(_usr: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetUserHome(_usr)
        }
    }
    #[fixed_stack_segment]
    fn GetUserId() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetUserId()
        }
    }
    #[fixed_stack_segment]
    fn GetUserName() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetUserName()
        }
    }
    #[fixed_stack_segment]
    fn GetVendorName() -> *u8 /* void* */ {
        unsafe {
            ELJApp_GetVendorName()
        }
    }
    #[fixed_stack_segment]
    fn InitAllImageHandlers() {
        unsafe {
            ELJApp_InitAllImageHandlers()
        }
    }
    #[fixed_stack_segment]
    fn Initialized() -> bool /* bool */ {
        unsafe {
            ELJApp_Initialized()
        }
    }
    #[fixed_stack_segment]
    fn MainLoop() -> c_int /* int */ {
        unsafe {
            ELJApp_MainLoop()
        }
    }
    #[fixed_stack_segment]
    fn MousePosition() -> *u8 /* void* */ {
        unsafe {
            ELJApp_MousePosition()
        }
    }
    #[fixed_stack_segment]
    fn Pending() -> c_int /* int */ {
        unsafe {
            ELJApp_Pending()
        }
    }
    #[fixed_stack_segment]
    fn SafeYield(_win: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJApp_SafeYield(_win)
        }
    }
    #[fixed_stack_segment]
    fn SetAppName(name: *u8 /* void* */) {
        unsafe {
            ELJApp_SetAppName(name)
        }
    }
    #[fixed_stack_segment]
    fn SetClassName(name: *u8 /* void* */) {
        unsafe {
            ELJApp_SetClassName(name)
        }
    }
    #[fixed_stack_segment]
    fn SetExitOnFrameDelete(flag: c_int /* int */) {
        unsafe {
            ELJApp_SetExitOnFrameDelete(flag)
        }
    }
    #[fixed_stack_segment]
    fn SetPrintMode(mode: c_int /* int */) {
        unsafe {
            ELJApp_SetPrintMode(mode)
        }
    }
    #[fixed_stack_segment]
    fn SetTooltipDelay(_ms: c_int /* int */) {
        unsafe {
            ELJApp_SetTooltipDelay(_ms)
        }
    }
    #[fixed_stack_segment]
    fn SetTopWindow(_wnd: *u8 /* void* */) {
        unsafe {
            ELJApp_SetTopWindow(_wnd)
        }
    }
    #[fixed_stack_segment]
    fn SetUseBestVisual(flag: c_int /* int */) {
        unsafe {
            ELJApp_SetUseBestVisual(flag)
        }
    }
    #[fixed_stack_segment]
    fn SetVendorName(name: *u8 /* void* */) {
        unsafe {
            ELJApp_SetVendorName(name)
        }
    }
    #[fixed_stack_segment]
    fn Sleep(_scs: c_int /* int */) {
        unsafe {
            ELJApp_Sleep(_scs)
        }
    }
    #[fixed_stack_segment]
    fn MilliSleep(_mscs: c_int /* int */) {
        unsafe {
            ELJApp_MilliSleep(_mscs)
        }
    }
    #[fixed_stack_segment]
    fn Yield() -> c_int /* int */ {
        unsafe {
            ELJApp_Yield()
        }
    }
    #[fixed_stack_segment]
    fn IsTerminating() -> c_int /* int */ {
        unsafe {
            ELJApp_IsTerminating()
        }
    }
}
trait wxTabEvent {
}
trait cbLeftDClickEvent {
    #[fixed_stack_segment]
    fn Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbLeftDClickEvent_Pos(_obj, arg0, arg1)
        }
    }
}
trait wxMBConvFile {
}
trait wxCaret {
    #[fixed_stack_segment]
    fn Create(_wnd: *u8 /* void* */, _wth: c_int /* int */, _hgt: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCaret_Create(_wnd, _wth, _hgt)
        }
    }
    #[fixed_stack_segment]
    fn GetBlinkTime() -> c_int /* int */ {
        unsafe {
            wxCaret_GetBlinkTime()
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCaret_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCaret_GetSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCaret_GetWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Hide(_obj: *u8 /* void* */) {
        unsafe {
            wxCaret_Hide(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCaret_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsVisible(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCaret_IsVisible(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Move(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxCaret_Move(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetBlinkTime(milliseconds: c_int /* int */) {
        unsafe {
            wxCaret_SetBlinkTime(milliseconds)
        }
    }
    #[fixed_stack_segment]
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxCaret_SetSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Show(_obj: *u8 /* void* */) {
        unsafe {
            wxCaret_Show(_obj)
        }
    }
}
trait wxHelpControllerHelpProvider {
    #[fixed_stack_segment]
    fn Create(ctr: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpControllerHelpProvider_Create(ctr)
        }
    }
    #[fixed_stack_segment]
    fn GetHelpController(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpControllerHelpProvider_GetHelpController(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetHelpController(_obj: *u8 /* void* */, hc: *u8 /* void* */) {
        unsafe {
            wxHelpControllerHelpProvider_SetHelpController(_obj, hc)
        }
    }
}
trait wxPlotWindow {
    #[fixed_stack_segment]
    fn Add(_obj: *u8 /* void* */, curve: *u8 /* void* */) {
        unsafe {
            wxPlotWindow_Add(_obj, curve)
        }
    }
    #[fixed_stack_segment]
    fn AddOnOff(_obj: *u8 /* void* */, curve: *u8 /* void* */) {
        unsafe {
            wxPlotWindow_AddOnOff(_obj, curve)
        }
    }
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPlotWindow_Create(parent, id, arg0, arg1, arg2, arg3, flags)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */, curve: *u8 /* void* */) {
        unsafe {
            wxPlotWindow_Delete(_obj, curve)
        }
    }
    #[fixed_stack_segment]
    fn DeleteOnOff(_obj: *u8 /* void* */, curve: *u8 /* void* */) {
        unsafe {
            wxPlotWindow_DeleteOnOff(_obj, curve)
        }
    }
    #[fixed_stack_segment]
    fn Enlarge(_obj: *u8 /* void* */, curve: *u8 /* void* */, factor: c_double /* double */) {
        unsafe {
            wxPlotWindow_Enlarge(_obj, curve, factor)
        }
    }
    #[fixed_stack_segment]
    fn GetAt(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPlotWindow_GetAt(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPlotWindow_GetCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCurrent(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPlotWindow_GetCurrent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnlargeAroundWindowCentre(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPlotWindow_GetEnlargeAroundWindowCentre(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOnOffCurveAt(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPlotWindow_GetOnOffCurveAt(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn GetOnOffCurveCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPlotWindow_GetOnOffCurveCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetScrollOnThumbRelease(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPlotWindow_GetScrollOnThumbRelease(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetUnitsPerValue(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxPlotWindow_GetUnitsPerValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetZoom(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxPlotWindow_GetZoom(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Move(_obj: *u8 /* void* */, curve: *u8 /* void* */, pixels_up: c_int /* int */) {
        unsafe {
            wxPlotWindow_Move(_obj, curve, pixels_up)
        }
    }
    #[fixed_stack_segment]
    fn RedrawEverything(_obj: *u8 /* void* */) {
        unsafe {
            wxPlotWindow_RedrawEverything(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RedrawXAxis(_obj: *u8 /* void* */) {
        unsafe {
            wxPlotWindow_RedrawXAxis(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RedrawYAxis(_obj: *u8 /* void* */) {
        unsafe {
            wxPlotWindow_RedrawYAxis(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ResetScrollbar(_obj: *u8 /* void* */) {
        unsafe {
            wxPlotWindow_ResetScrollbar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCurrent(_obj: *u8 /* void* */, current: *u8 /* void* */) {
        unsafe {
            wxPlotWindow_SetCurrent(_obj, current)
        }
    }
    #[fixed_stack_segment]
    fn SetEnlargeAroundWindowCentre(_obj: *u8 /* void* */, enlargeAroundWindowCentre: c_int /* int */) {
        unsafe {
            wxPlotWindow_SetEnlargeAroundWindowCentre(_obj, enlargeAroundWindowCentre)
        }
    }
    #[fixed_stack_segment]
    fn SetScrollOnThumbRelease(_obj: *u8 /* void* */, scrollOnThumbRelease: c_int /* int */) {
        unsafe {
            wxPlotWindow_SetScrollOnThumbRelease(_obj, scrollOnThumbRelease)
        }
    }
    #[fixed_stack_segment]
    fn SetUnitsPerValue(_obj: *u8 /* void* */, upv: c_double /* double */) {
        unsafe {
            wxPlotWindow_SetUnitsPerValue(_obj, upv)
        }
    }
    #[fixed_stack_segment]
    fn SetZoom(_obj: *u8 /* void* */, zoom: c_double /* double */) {
        unsafe {
            wxPlotWindow_SetZoom(_obj, zoom)
        }
    }
}
trait wxSocketClient {
}
trait wxPlotEvent {
    #[fixed_stack_segment]
    fn GetCurve(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPlotEvent_GetCurve(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPlotEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetZoom(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxPlotEvent_GetZoom(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxPlotEvent_SetPosition(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn SetZoom(_obj: *u8 /* void* */, zoom: c_double /* double */) {
        unsafe {
            wxPlotEvent_SetZoom(_obj, zoom)
        }
    }
}
trait wxCommandEvent {
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_CopyObject(_obj, object_dest)
        }
    }
    #[fixed_stack_segment]
    fn Create(_typ: c_int /* int */, _id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCommandEvent_Create(_typ, _id)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCommandEvent_GetClientData(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetClientObject(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCommandEvent_GetClientObject(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetExtraLong(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxCommandEvent_GetExtraLong(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetInt(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxCommandEvent_GetInt(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCommandEvent_GetSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCommandEvent_GetString(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsChecked(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCommandEvent_IsChecked(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsSelection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCommandEvent_IsSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetClientData(_obj: *u8 /* void* */, clientData: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_SetClientData(_obj, clientData)
        }
    }
    #[fixed_stack_segment]
    fn SetClientObject(_obj: *u8 /* void* */, clientObject: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_SetClientObject(_obj, clientObject)
        }
    }
    #[fixed_stack_segment]
    fn SetExtraLong(_obj: *u8 /* void* */, extraLong: c_long /* long */) {
        unsafe {
            wxCommandEvent_SetExtraLong(_obj, extraLong)
        }
    }
    #[fixed_stack_segment]
    fn SetInt(_obj: *u8 /* void* */, i: c_int /* int */) {
        unsafe {
            wxCommandEvent_SetInt(_obj, i)
        }
    }
    #[fixed_stack_segment]
    fn SetString(_obj: *u8 /* void* */, s: *u8 /* void* */) {
        unsafe {
            wxCommandEvent_SetString(_obj, s)
        }
    }
}
trait cbRemoveBarEvent {
    #[fixed_stack_segment]
    fn Bar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbRemoveBarEvent_Bar(_obj)
        }
    }
}
trait wxZlibOutputStream {
}
trait wxLogWindow {
}
trait wxPaletteChangedEvent {
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxPaletteChangedEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetChangedWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPaletteChangedEvent_GetChangedWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetChangedWindow(_obj: *u8 /* void* */, win: *u8 /* void* */) {
        unsafe {
            wxPaletteChangedEvent_SetChangedWindow(_obj, win)
        }
    }
}
trait ELJClient {
    #[fixed_stack_segment]
    fn Create(_eobj: *u8 /* void* */, _cnct: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJClient_Create(_eobj, _cnct)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJClient_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MakeConnection(_obj: *u8 /* void* */, host: *u8 /* void* */, server: *u8 /* void* */, topic: *u8 /* void* */) {
        unsafe {
            ELJClient_MakeConnection(_obj, host, server, topic)
        }
    }
}
trait wxGenericValidator {
}
trait wxStyledTextEvent {
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetKey(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetKey(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetModifiers(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetModifiers(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetModificationType(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetModificationType(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLength(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetLength(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLinesAdded(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetLinesAdded(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLine(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetLine(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFoldLevelNow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetFoldLevelNow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFoldLevelPrev(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetFoldLevelPrev(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMargin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetMargin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMessage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetMessage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWParam(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetWParam(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLParam(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetLParam(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetListType(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetListType(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDragText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextEvent_GetDragText(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDragAllowMove(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextEvent_GetDragAllowMove(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDragResult(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextEvent_GetDragResult(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetShift(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextEvent_GetShift(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetControl(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextEvent_GetControl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetAlt(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextEvent_GetAlt(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextEvent_GetText(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Clone(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextEvent_Clone(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetPosition(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn SetKey(_obj: *u8 /* void* */, k: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetKey(_obj, k)
        }
    }
    #[fixed_stack_segment]
    fn SetModifiers(_obj: *u8 /* void* */, m: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetModifiers(_obj, m)
        }
    }
    #[fixed_stack_segment]
    fn SetModificationType(_obj: *u8 /* void* */, t: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetModificationType(_obj, t)
        }
    }
    #[fixed_stack_segment]
    fn SetText(_obj: *u8 /* void* */, t: *u8 /* void* */) {
        unsafe {
            wxStyledTextEvent_SetText(_obj, t)
        }
    }
    #[fixed_stack_segment]
    fn SetLength(_obj: *u8 /* void* */, len: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetLength(_obj, len)
        }
    }
    #[fixed_stack_segment]
    fn SetLinesAdded(_obj: *u8 /* void* */, num: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetLinesAdded(_obj, num)
        }
    }
    #[fixed_stack_segment]
    fn SetLine(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetLine(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetFoldLevelNow(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetFoldLevelNow(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetFoldLevelPrev(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetFoldLevelPrev(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetMargin(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetMargin(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetMessage(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetMessage(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetWParam(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetWParam(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetLParam(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetLParam(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetListType(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetListType(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetX(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetX(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetY(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetY(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetDragText(_obj: *u8 /* void* */, val: *u8 /* void* */) {
        unsafe {
            wxStyledTextEvent_SetDragText(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetDragAllowMove(_obj: *u8 /* void* */, val: bool /* bool */) {
        unsafe {
            wxStyledTextEvent_SetDragAllowMove(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn SetDragResult(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxStyledTextEvent_SetDragResult(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_CHANGE() -> c_int /* int */ {
        unsafe {
            expEVT_STC_CHANGE()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_STYLENEEDED() -> c_int /* int */ {
        unsafe {
            expEVT_STC_STYLENEEDED()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_CHARADDED() -> c_int /* int */ {
        unsafe {
            expEVT_STC_CHARADDED()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_SAVEPOINTREACHED() -> c_int /* int */ {
        unsafe {
            expEVT_STC_SAVEPOINTREACHED()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_SAVEPOINTLEFT() -> c_int /* int */ {
        unsafe {
            expEVT_STC_SAVEPOINTLEFT()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_ROMODIFYATTEMPT() -> c_int /* int */ {
        unsafe {
            expEVT_STC_ROMODIFYATTEMPT()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_KEY() -> c_int /* int */ {
        unsafe {
            expEVT_STC_KEY()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_DOUBLECLICK() -> c_int /* int */ {
        unsafe {
            expEVT_STC_DOUBLECLICK()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_UPDATEUI() -> c_int /* int */ {
        unsafe {
            expEVT_STC_UPDATEUI()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_MODIFIED() -> c_int /* int */ {
        unsafe {
            expEVT_STC_MODIFIED()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_MACRORECORD() -> c_int /* int */ {
        unsafe {
            expEVT_STC_MACRORECORD()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_MARGINCLICK() -> c_int /* int */ {
        unsafe {
            expEVT_STC_MARGINCLICK()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_NEEDSHOWN() -> c_int /* int */ {
        unsafe {
            expEVT_STC_NEEDSHOWN()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_PAINTED() -> c_int /* int */ {
        unsafe {
            expEVT_STC_PAINTED()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_USERLISTSELECTION() -> c_int /* int */ {
        unsafe {
            expEVT_STC_USERLISTSELECTION()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_URIDROPPED() -> c_int /* int */ {
        unsafe {
            expEVT_STC_URIDROPPED()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_DWELLSTART() -> c_int /* int */ {
        unsafe {
            expEVT_STC_DWELLSTART()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_DWELLEND() -> c_int /* int */ {
        unsafe {
            expEVT_STC_DWELLEND()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_START_DRAG() -> c_int /* int */ {
        unsafe {
            expEVT_STC_START_DRAG()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_DRAG_OVER() -> c_int /* int */ {
        unsafe {
            expEVT_STC_DRAG_OVER()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_DO_DROP() -> c_int /* int */ {
        unsafe {
            expEVT_STC_DO_DROP()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_ZOOM() -> c_int /* int */ {
        unsafe {
            expEVT_STC_ZOOM()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_HOTSPOT_CLICK() -> c_int /* int */ {
        unsafe {
            expEVT_STC_HOTSPOT_CLICK()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_HOTSPOT_DCLICK() -> c_int /* int */ {
        unsafe {
            expEVT_STC_HOTSPOT_DCLICK()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_CALLTIP_CLICK() -> c_int /* int */ {
        unsafe {
            expEVT_STC_CALLTIP_CLICK()
        }
    }
    #[fixed_stack_segment]
    fn expEVT_STC_AUTOCOMP_SELECTION() -> c_int /* int */ {
        unsafe {
            expEVT_STC_AUTOCOMP_SELECTION()
        }
    }
}
trait wxLogChain {
    #[fixed_stack_segment]
    fn Create(logger: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLogChain_Create(logger)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxLogChain_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOldLog(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLogChain_GetOldLog(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsPassingMessages(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLogChain_IsPassingMessages(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PassMessages(_obj: *u8 /* void* */, bDoPass: bool /* bool */) {
        unsafe {
            wxLogChain_PassMessages(_obj, bDoPass)
        }
    }
    #[fixed_stack_segment]
    fn SetLog(_obj: *u8 /* void* */, logger: *u8 /* void* */) {
        unsafe {
            wxLogChain_SetLog(_obj, logger)
        }
    }
}
trait wxPreviewCanvas {
    #[fixed_stack_segment]
    fn Create(preview: *u8 /* void* */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPreviewCanvas_Create(preview, parent, arg0, arg1, arg2, arg3, style)
        }
    }
}
trait wxActivateEvent {
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxActivateEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetActive(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxActivateEvent_GetActive(_obj)
        }
    }
}
trait cbDrawHintRectEvent {
    #[fixed_stack_segment]
    fn EraseRect(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbDrawHintRectEvent_EraseRect(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsInClient(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            cbDrawHintRectEvent_IsInClient(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LastTime(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbDrawHintRectEvent_LastTime(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Rect(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            cbDrawHintRectEvent_Rect(_obj, arg0, arg1, arg2, arg3)
        }
    }
}
trait cbInsertBarEvent {
    #[fixed_stack_segment]
    fn Bar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbInsertBarEvent_Bar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Row(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbInsertBarEvent_Row(_obj)
        }
    }
}
trait wxHtmlColourCell {
}
trait cbDrawBarDecorEvent {
    #[fixed_stack_segment]
    fn Bar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawBarDecorEvent_Bar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn BoundsInParent(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            cbDrawBarDecorEvent_BoundsInParent(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn Dc(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawBarDecorEvent_Dc(_obj)
        }
    }
}
trait wxGridCellTextEditor {
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            cbBarHintsPlugin_Create(pPanel, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbBarHintsPlugin_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbBarHintsPlugin_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetGrooveCount(_obj: *u8 /* void* */, nGrooves: c_int /* int */) {
        unsafe {
            cbBarHintsPlugin_SetGrooveCount(_obj, nGrooves)
        }
    }
}
trait cbSizeBarWndEvent {
    #[fixed_stack_segment]
    fn Bar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbSizeBarWndEvent_Bar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn BoundsInParent(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            cbSizeBarWndEvent_BoundsInParent(_obj, arg0, arg1, arg2, arg3)
        }
    }
}
trait wxPropertyCategory {
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxSizeEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizeEvent_GetSize(_obj)
        }
    }
}
trait wxWizardPageSimple {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizardPageSimple_Create(_prt)
        }
    }
    #[fixed_stack_segment]
    fn GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxWizardPageSimple_GetBitmap(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetNext(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizardPageSimple_GetNext(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrev(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizardPageSimple_GetPrev(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetNext(_obj: *u8 /* void* */, next: *u8 /* void* */) {
        unsafe {
            wxWizardPageSimple_SetNext(_obj, next)
        }
    }
    #[fixed_stack_segment]
    fn SetPrev(_obj: *u8 /* void* */, prev: *u8 /* void* */) {
        unsafe {
            wxWizardPageSimple_SetPrev(_obj, prev)
        }
    }
}
trait wxFilterInputStream {
}
trait cbBarDragPlugin {
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            cbBarDragPlugin_Create(pPanel, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbBarDragPlugin_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbBarDragPlugin_Delete(_obj)
        }
    }
}
trait wxBusyInfo {
    #[fixed_stack_segment]
    fn Create(_txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBusyInfo_Create(_txt)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxBusyInfo_Delete(_obj)
        }
    }
}
trait wxInputSink {
    #[fixed_stack_segment]
    fn Create(input: *u8 /* void* */, evtHandler: *u8 /* void* */, bufferLen: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxInputSink_Create(input, evtHandler, bufferLen)
        }
    }
    #[fixed_stack_segment]
    fn GetId(obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputSink_GetId(obj)
        }
    }
    #[fixed_stack_segment]
    fn Start(obj: *u8 /* void* */) {
        unsafe {
            wxInputSink_Start(obj)
        }
    }
}
trait wxTimerEvent {
    #[fixed_stack_segment]
    fn GetInterval(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTimerEvent_GetInterval(_obj)
        }
    }
}
trait wxListEvent {
    #[fixed_stack_segment]
    fn Cancelled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListEvent_Cancelled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetCode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetColumn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetColumn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetData(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetData(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetImage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetImage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetIndex(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetIndex(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListEvent_GetItem(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListEvent_GetLabel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetMask(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPoint(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListEvent_GetPoint(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListEvent_GetText(_obj)
        }
    }
}
trait wxColour {
    #[fixed_stack_segment]
    fn Alpha(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */ {
        unsafe {
            wxColour_Alpha(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, other: *u8 /* void* */) {
        unsafe {
            wxColour_Assign(_obj, other)
        }
    }
    #[fixed_stack_segment]
    fn Blue(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */ {
        unsafe {
            wxColour_Blue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Copy(_obj: *u8 /* void* */, _other: *u8 /* void* */) {
        unsafe {
            wxColour_Copy(_obj, _other)
        }
    }
    #[fixed_stack_segment]
    fn CreateByName(_name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateByName(_name)
        }
    }
    #[fixed_stack_segment]
    fn CreateEmpty() -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateEmpty()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateFromStock(id)
        }
    }
    #[fixed_stack_segment]
    fn CreateRGB(_red: uint8_t /* uint8_t */, _green: uint8_t /* uint8_t */, _blue: uint8_t /* uint8_t */, _alpha: uint8_t /* uint8_t */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateRGB(_red, _green, _blue, _alpha)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxColour_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Green(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */ {
        unsafe {
            wxColour_Green(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxColour_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Red(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */ {
        unsafe {
            wxColour_Red(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Set(_obj: *u8 /* void* */, _red: uint8_t /* uint8_t */, _green: uint8_t /* uint8_t */, _blue: uint8_t /* uint8_t */, _alpha: uint8_t /* uint8_t */) {
        unsafe {
            wxColour_Set(_obj, _red, _green, _blue, _alpha)
        }
    }
    #[fixed_stack_segment]
    fn SetByName(_obj: *u8 /* void* */, _name: *u8 /* void* */) {
        unsafe {
            wxColour_SetByName(_obj, _name)
        }
    }
    #[fixed_stack_segment]
    fn ValidName(_name: *wchar_t /* wchar_t* */) -> bool /* bool */ {
        unsafe {
            wxColour_ValidName(_name)
        }
    }
}
trait wxGridCellStringRenderer {
}
trait wxTreeCompanionWindow {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCompanionWindow_Create(parent, id, arg0, arg1, arg2, arg3, style)
        }
    }
    #[fixed_stack_segment]
    fn DrawItem(_obj: *u8 /* void* */, dc: *u8 /* void* */, id: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxTreeCompanionWindow_DrawItem(_obj, dc, id, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn GetTreeCtrl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCompanionWindow_GetTreeCtrl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetTreeCtrl(_obj: *u8 /* void* */, treeCtrl: *u8 /* void* */) {
        unsafe {
            wxTreeCompanionWindow_SetTreeCtrl(_obj, treeCtrl)
        }
    }
}
trait wxRegionIterator {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxRegionIterator_Create()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromRegion(region: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxRegionIterator_CreateFromRegion(region)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxRegionIterator_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRegionIterator_GetHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRegionIterator_GetWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRegionIterator_GetX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRegionIterator_GetY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HaveRects(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegionIterator_HaveRects(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Next(_obj: *u8 /* void* */) {
        unsafe {
            wxRegionIterator_Next(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Reset(_obj: *u8 /* void* */) {
        unsafe {
            wxRegionIterator_Reset(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxLEDNumberCtrl_Create(parent, id, arg0, arg1, arg2, arg3, style)
        }
    }
    #[fixed_stack_segment]
    fn GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxLEDNumberCtrl_GetAlignment(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDrawFaded(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxLEDNumberCtrl_GetDrawFaded(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxLEDNumberCtrl_GetValue(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn SetAlignment(_obj: *u8 /* void* */, Alignment: c_int /* int */, Redraw: c_int /* int */) {
        unsafe {
            wxLEDNumberCtrl_SetAlignment(_obj, Alignment, Redraw)
        }
    }
    #[fixed_stack_segment]
    fn SetDrawFaded(_obj: *u8 /* void* */, DrawFaded: c_int /* int */, Redraw: c_int /* int */) {
        unsafe {
            wxLEDNumberCtrl_SetDrawFaded(_obj, DrawFaded, Redraw)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, Value: *u8 /* void* */, Redraw: c_int /* int */) {
        unsafe {
            wxLEDNumberCtrl_SetValue(_obj, Value, Redraw)
        }
    }
}
trait wxMimeTypesManager {
    #[fixed_stack_segment]
    fn AddFallbacks(_obj: *u8 /* void* */, _types: *u8 /* void* */) {
        unsafe {
            wxMimeTypesManager_AddFallbacks(_obj, _types)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxMimeTypesManager_Create()
        }
    }
    #[fixed_stack_segment]
    fn EnumAllFileTypes(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMimeTypesManager_EnumAllFileTypes(_obj, _lst)
        }
    }
    #[fixed_stack_segment]
    fn GetFileTypeFromExtension(_obj: *u8 /* void* */, _ext: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMimeTypesManager_GetFileTypeFromExtension(_obj, _ext)
        }
    }
    #[fixed_stack_segment]
    fn GetFileTypeFromMimeType(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMimeTypesManager_GetFileTypeFromMimeType(_obj, _name)
        }
    }
    #[fixed_stack_segment]
    fn IsOfType(_obj: *u8 /* void* */, _type: *u8 /* void* */, _wildcard: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMimeTypesManager_IsOfType(_obj, _type, _wildcard)
        }
    }
}
trait wxCursor {
    #[fixed_stack_segment]
    fn Cursor_CreateFromStock(_id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            Cursor_CreateFromStock(_id)
        }
    }
    #[fixed_stack_segment]
    fn Cursor_CreateFromImage(image: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            Cursor_CreateFromImage(image)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxStopWatch_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxStopWatch_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Start(_obj: *u8 /* void* */, msec: c_int /* int */) {
        unsafe {
            wxStopWatch_Start(_obj, msec)
        }
    }
    #[fixed_stack_segment]
    fn Pause(_obj: *u8 /* void* */) {
        unsafe {
            wxStopWatch_Pause(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Resume(_obj: *u8 /* void* */) {
        unsafe {
            wxStopWatch_Resume(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Time(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStopWatch_Time(_obj)
        }
    }
}
trait wxNotebook {
    #[fixed_stack_segment]
    fn AddPage(_obj: *u8 /* void* */, pPage: *u8 /* void* */, strText: *u8 /* void* */, bSelect: bool /* bool */, imageId: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_AddPage(_obj, pPage, strText, bSelect, imageId)
        }
    }
    #[fixed_stack_segment]
    fn AdvanceSelection(_obj: *u8 /* void* */, bForward: bool /* bool */) {
        unsafe {
            wxNotebook_AdvanceSelection(_obj, bForward)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxNotebook_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn DeleteAllPages(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNotebook_DeleteAllPages(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DeletePage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_DeletePage(_obj, nPage)
        }
    }
    #[fixed_stack_segment]
    fn GetImageList(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxNotebook_GetImageList(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxNotebook_GetPage(_obj, nPage)
        }
    }
    #[fixed_stack_segment]
    fn GetPageCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxNotebook_GetPageCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPageImage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxNotebook_GetPageImage(_obj, nPage)
        }
    }
    #[fixed_stack_segment]
    fn GetPageText(_obj: *u8 /* void* */, nPage: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxNotebook_GetPageText(_obj, nPage)
        }
    }
    #[fixed_stack_segment]
    fn GetRowCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxNotebook_GetRowCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxNotebook_GetSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *c_long /* long* */) -> c_int /* int */ {
        unsafe {
            wxNotebook_HitTest(_obj, arg0, arg1, flags)
        }
    }
    #[fixed_stack_segment]
    fn InsertPage(_obj: *u8 /* void* */, nPage: c_int /* int */, pPage: *u8 /* void* */, strText: *u8 /* void* */, bSelect: bool /* bool */, imageId: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_InsertPage(_obj, nPage, pPage, strText, bSelect, imageId)
        }
    }
    #[fixed_stack_segment]
    fn RemovePage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_RemovePage(_obj, nPage)
        }
    }
    #[fixed_stack_segment]
    fn SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxNotebook_SetImageList(_obj, imageList)
        }
    }
    #[fixed_stack_segment]
    fn SetPadding(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxNotebook_SetPadding(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetPageImage(_obj: *u8 /* void* */, nPage: c_int /* int */, nImage: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxNotebook_SetPageImage(_obj, nPage, nImage)
        }
    }
    #[fixed_stack_segment]
    fn SetPageSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxNotebook_SetPageSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetPageText(_obj: *u8 /* void* */, nPage: c_int /* int */, strText: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNotebook_SetPageText(_obj, nPage, strText)
        }
    }
    #[fixed_stack_segment]
    fn SetSelection(_obj: *u8 /* void* */, nPage: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxNotebook_SetSelection(_obj, nPage)
        }
    }
    #[fixed_stack_segment]
    fn expNB_TOP() -> c_int /* int */ {
        unsafe {
            expNB_TOP()
        }
    }
    #[fixed_stack_segment]
    fn expNB_BOTTOM() -> c_int /* int */ {
        unsafe {
            expNB_BOTTOM()
        }
    }
    #[fixed_stack_segment]
    fn expNB_LEFT() -> c_int /* int */ {
        unsafe {
            expNB_LEFT()
        }
    }
    #[fixed_stack_segment]
    fn expNB_RIGHT() -> c_int /* int */ {
        unsafe {
            expNB_RIGHT()
        }
    }
    #[fixed_stack_segment]
    fn expBK_HITTEST_NOWHERE() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_NOWHERE()
        }
    }
    #[fixed_stack_segment]
    fn expBK_HITTEST_ONICON() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_ONICON()
        }
    }
    #[fixed_stack_segment]
    fn expBK_HITTEST_ONLABEL() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_ONLABEL()
        }
    }
    #[fixed_stack_segment]
    fn expBK_HITTEST_ONITEM() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_ONITEM()
        }
    }
    #[fixed_stack_segment]
    fn expBK_HITTEST_ONPAGE() -> c_int /* int */ {
        unsafe {
            expBK_HITTEST_ONPAGE()
        }
    }
}
trait wxDCClipper {
}
trait wxBitmapDataObject {
    #[fixed_stack_segment]
    fn BitmapDataObject_Create(_bmp: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            BitmapDataObject_Create(_bmp)
        }
    }
    #[fixed_stack_segment]
    fn BitmapDataObject_CreateEmpty() -> *u8 /* void* */ {
        unsafe {
            BitmapDataObject_CreateEmpty()
        }
    }
    #[fixed_stack_segment]
    fn BitmapDataObject_Delete(_obj: *u8 /* void* */) {
        unsafe {
            BitmapDataObject_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn BitmapDataObject_GetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */) {
        unsafe {
            BitmapDataObject_GetBitmap(_obj, _bmp)
        }
    }
    #[fixed_stack_segment]
    fn BitmapDataObject_SetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */) {
        unsafe {
            BitmapDataObject_SetBitmap(_obj, _bmp)
        }
    }
}
trait cbCustomizeLayoutEvent {
    #[fixed_stack_segment]
    fn ClickPos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbCustomizeLayoutEvent_ClickPos(_obj, arg0, arg1)
        }
    }
}
trait wxPostScriptPrintNativeData {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxPostScriptPrintNativeData_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPostScriptPrintNativeData_Delete(_obj)
        }
    }
}
trait ELJPreviewControlBar {
    #[fixed_stack_segment]
    fn Create(preview: *u8 /* void* */, buttons: c_int /* int */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJPreviewControlBar_Create(preview, buttons, parent, title, arg0, arg1, arg2, arg3, style)
        }
    }
}
trait wxFFile {
}
trait wxPageSetupDialogData {
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_Assign(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn AssignData(_obj: *u8 /* void* */, printData: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_AssignData(_obj, printData)
        }
    }
    #[fixed_stack_segment]
    fn CalculateIdFromPaperSize(_obj: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_CalculateIdFromPaperSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CalculatePaperSizeFromId(_obj: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_CalculatePaperSizeFromId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_Create()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromData(printData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_CreateFromData(printData)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EnableHelp(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnableHelp(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn EnableMargins(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnableMargins(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn EnableOrientation(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnableOrientation(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn EnablePaper(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnablePaper(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn EnablePrinter(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_EnablePrinter(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultInfo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetDefaultInfo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultMinMargins(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetDefaultMinMargins(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnableHelp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnableHelp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnableMargins(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnableMargins(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnableOrientation(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnableOrientation(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnablePaper(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnablePaper(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnablePrinter(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPageSetupDialogData_GetEnablePrinter(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMarginBottomRight(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetMarginBottomRight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMarginTopLeft(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetMarginTopLeft(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinMarginBottomRight(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetMinMarginBottomRight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinMarginTopLeft(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetMinMarginTopLeft(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPaperId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPageSetupDialogData_GetPaperId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPaperSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPageSetupDialogData_GetPaperSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_GetPrintData(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultInfo(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPageSetupDialogData_SetDefaultInfo(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultMinMargins(_obj: *u8 /* void* */, flag: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetDefaultMinMargins(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn SetMarginBottomRight(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetMarginBottomRight(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetMarginTopLeft(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetMarginTopLeft(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetMinMarginBottomRight(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetMinMarginBottomRight(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetMinMarginTopLeft(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetMinMarginTopLeft(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetPaperId(_obj: *u8 /* void* */, id: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_SetPaperId(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn SetPaperSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetPaperSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetPaperSizeId(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxPageSetupDialogData_SetPaperSizeId(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn SetPrintData(_obj: *u8 /* void* */, printData: *u8 /* void* */) {
        unsafe {
            wxPageSetupDialogData_SetPrintData(_obj, printData)
        }
    }
}
trait wxCommandLineParser {
}
trait wxLayoutConstraints {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_Create()
        }
    }
    #[fixed_stack_segment]
    fn bottom(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_bottom(_obj)
        }
    }
    #[fixed_stack_segment]
    fn centreX(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_centreX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn centreY(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_centreY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn height(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_height(_obj)
        }
    }
    #[fixed_stack_segment]
    fn left(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_left(_obj)
        }
    }
    #[fixed_stack_segment]
    fn right(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_right(_obj)
        }
    }
    #[fixed_stack_segment]
    fn top(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_top(_obj)
        }
    }
    #[fixed_stack_segment]
    fn width(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLayoutConstraints_width(_obj)
        }
    }
}
trait wxDrawWindow {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDrawWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait wxDataFormat {
    #[fixed_stack_segment]
    fn CreateFromId(name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDataFormat_CreateFromId(name)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromType(typ: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDataFormat_CreateFromType(typ)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDataFormat_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetId(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDataFormat_GetId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetType(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxDataFormat_GetType(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsEqual(_obj: *u8 /* void* */, other: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDataFormat_IsEqual(_obj, other)
        }
    }
    #[fixed_stack_segment]
    fn SetId(_obj: *u8 /* void* */, id: *u8 /* void* */) {
        unsafe {
            wxDataFormat_SetId(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn SetType(_obj: *u8 /* void* */, typ: c_int /* int */) {
        unsafe {
            wxDataFormat_SetType(_obj, typ)
        }
    }
}
trait cbDockBox {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            cbDockBox_Create()
        }
    }
}
trait wxBitmap {
    #[fixed_stack_segment]
    fn AddHandler(handler: *u8 /* void* */) {
        unsafe {
            wxBitmap_AddHandler(handler)
        }
    }
    #[fixed_stack_segment]
    fn CleanUpHandlers() {
        unsafe {
            wxBitmap_CleanUpHandlers()
        }
    }
    #[fixed_stack_segment]
    fn Create(_data: *u8 /* void* */, _type: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, _depth: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_Create(_data, _type, arg0, arg1, _depth)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateEmpty(arg0: c_int /* int */, arg1: c_int /* int */, _depth: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateEmpty(arg0, arg1, _depth)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromXPM(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateFromXPM(data)
        }
    }
    #[fixed_stack_segment]
    fn CreateLoad(name: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateLoad(name, type_)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxBitmap_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FindHandlerByExtension(extension: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_FindHandlerByExtension(extension, type_)
        }
    }
    #[fixed_stack_segment]
    fn FindHandlerByName(name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_FindHandlerByName(name)
        }
    }
    #[fixed_stack_segment]
    fn FindHandlerByType(type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_FindHandlerByType(type_)
        }
    }
    #[fixed_stack_segment]
    fn GetDepth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmap_GetDepth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmap_GetHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMask(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_GetMask(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSubBitmap(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _ref: *u8 /* void* */) {
        unsafe {
            wxBitmap_GetSubBitmap(_obj, arg0, arg1, arg2, arg3, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmap_GetWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn InitStandardHandlers() {
        unsafe {
            wxBitmap_InitStandardHandlers()
        }
    }
    #[fixed_stack_segment]
    fn InsertHandler(handler: *u8 /* void* */) {
        unsafe {
            wxBitmap_InsertHandler(handler)
        }
    }
    #[fixed_stack_segment]
    fn LoadFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxBitmap_LoadFile(_obj, name, type_)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBitmap_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RemoveHandler(name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBitmap_RemoveHandler(name)
        }
    }
    #[fixed_stack_segment]
    fn SaveFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */, cmap: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBitmap_SaveFile(_obj, name, type_, cmap)
        }
    }
    #[fixed_stack_segment]
    fn SetDepth(_obj: *u8 /* void* */, d: c_int /* int */) {
        unsafe {
            wxBitmap_SetDepth(_obj, d)
        }
    }
    #[fixed_stack_segment]
    fn SetHeight(_obj: *u8 /* void* */, h: c_int /* int */) {
        unsafe {
            wxBitmap_SetHeight(_obj, h)
        }
    }
    #[fixed_stack_segment]
    fn SetMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) {
        unsafe {
            wxBitmap_SetMask(_obj, mask)
        }
    }
    #[fixed_stack_segment]
    fn SetWidth(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxBitmap_SetWidth(_obj, w)
        }
    }
}
trait wxBufferedOutputStream {
}
trait cbMotionEvent {
    #[fixed_stack_segment]
    fn Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbMotionEvent_Pos(_obj, arg0, arg1)
        }
    }
}
trait cbRowInfo {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            cbRowInfo_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbRowInfo_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFirstBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbRowInfo_GetFirstBar(_obj)
        }
    }
}
trait wxGridCellChoiceEditor {
    #[fixed_stack_segment]
    fn Ctor(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */, allowOthers: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellChoiceEditor_Ctor(arg0, arg1, allowOthers)
        }
    }
}
trait wxScopedPtr {
}
trait wxEraseEvent {
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxEraseEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDC(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEraseEvent_GetDC(_obj)
        }
    }
}
trait wxPanel {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPanel_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn InitDialog(_obj: *u8 /* void* */) {
        unsafe {
            wxPanel_InitDialog(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetFocus(_obj: *u8 /* void* */) {
        unsafe {
            wxPanel_SetFocus(_obj)
        }
    }
}
trait cbFinishDrawInAreaEvent {
    #[fixed_stack_segment]
    fn Area(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            cbFinishDrawInAreaEvent_Area(_obj, arg0, arg1, arg2, arg3)
        }
    }
}
trait wxPreviewFrame {
    #[fixed_stack_segment]
    fn Create(preview: *u8 /* void* */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPreviewFrame_Create(preview, parent, title, arg0, arg1, arg2, arg3, style, name)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxPreviewFrame_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn Initialize(self_: *u8 /* void* */) {
        unsafe {
            wxPreviewFrame_Initialize(self_)
        }
    }
}
trait wxFocusEvent {
}
trait cbCollapseBox {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            cbCollapseBox_Create()
        }
    }
}
trait wxLogStream {
}
trait wxScrolledWindow {
    #[fixed_stack_segment]
    fn AdjustScrollbars(_obj: *u8 /* void* */) {
        unsafe {
            wxScrolledWindow_AdjustScrollbars(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CalcScrolledPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_CalcScrolledPosition(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn CalcUnscrolledPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_CalcUnscrolledPosition(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxScrolledWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn EnableScrolling(_obj: *u8 /* void* */, x_scrolling: bool /* bool */, y_scrolling: bool /* bool */) {
        unsafe {
            wxScrolledWindow_EnableScrolling(_obj, x_scrolling, y_scrolling)
        }
    }
    #[fixed_stack_segment]
    fn GetScaleX(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxScrolledWindow_GetScaleX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetScaleY(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxScrolledWindow_GetScaleY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetScrollPageSize(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxScrolledWindow_GetScrollPageSize(_obj, orient)
        }
    }
    #[fixed_stack_segment]
    fn GetScrollPixelsPerUnit(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_GetScrollPixelsPerUnit(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetTargetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxScrolledWindow_GetTargetWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_GetViewStart(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetVirtualSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_GetVirtualSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn OnDraw(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxScrolledWindow_OnDraw(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxScrolledWindow_PrepareDC(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn Scroll(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxScrolledWindow_Scroll(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetScale(_obj: *u8 /* void* */, xs: c_double /* double */, ys: c_double /* double */) {
        unsafe {
            wxScrolledWindow_SetScale(_obj, xs, ys)
        }
    }
    #[fixed_stack_segment]
    fn SetScrollPageSize(_obj: *u8 /* void* */, orient: c_int /* int */, pageSize: c_int /* int */) {
        unsafe {
            wxScrolledWindow_SetScrollPageSize(_obj, orient, pageSize)
        }
    }
    #[fixed_stack_segment]
    fn SetScrollbars(_obj: *u8 /* void* */, pixelsPerUnitX: c_int /* int */, pixelsPerUnitY: c_int /* int */, noUnitsX: c_int /* int */, noUnitsY: c_int /* int */, xPos: c_int /* int */, yPos: c_int /* int */, noRefresh: bool /* bool */) {
        unsafe {
            wxScrolledWindow_SetScrollbars(_obj, pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh)
        }
    }
    #[fixed_stack_segment]
    fn ShowScrollbars(_obj: *u8 /* void* */, showh: c_int /* int */, showv: c_int /* int */) {
        unsafe {
            wxScrolledWindow_ShowScrollbars(_obj, showh, showv)
        }
    }
    #[fixed_stack_segment]
    fn SetTargetWindow(_obj: *u8 /* void* */, target: *u8 /* void* */) {
        unsafe {
            wxScrolledWindow_SetTargetWindow(_obj, target)
        }
    }
    #[fixed_stack_segment]
    fn ViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxScrolledWindow_ViewStart(_obj, arg0, arg1)
        }
    }
}
trait wxSockAddress {
}
trait wxMask {
    #[fixed_stack_segment]
    fn Create(bitmap: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMask_Create(bitmap)
        }
    }
    #[fixed_stack_segment]
    fn CreateColoured(bitmap: *u8 /* void* */, colour: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMask_CreateColoured(bitmap, colour)
        }
    }
}
trait wxImageList {
    #[fixed_stack_segment]
    fn AddBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */, mask: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImageList_AddBitmap(_obj, bitmap, mask)
        }
    }
    #[fixed_stack_segment]
    fn AddIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImageList_AddIcon(_obj, icon)
        }
    }
    #[fixed_stack_segment]
    fn AddMasked(_obj: *u8 /* void* */, bitmap: *u8 /* void* */, maskColour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImageList_AddMasked(_obj, bitmap, maskColour)
        }
    }
    #[fixed_stack_segment]
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */, mask: c_int /* int */, initialCount: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImageList_Create(arg0, arg1, mask, initialCount)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxImageList_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Draw(_obj: *u8 /* void* */, index: c_int /* int */, dc: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: c_int /* int */, solidBackground: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxImageList_Draw(_obj, index, dc, arg0, arg1, flags, solidBackground)
        }
    }
    #[fixed_stack_segment]
    fn GetImageCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImageList_GetImageCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */, index: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxImageList_GetSize(_obj, index, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Remove(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxImageList_Remove(_obj, index)
        }
    }
    #[fixed_stack_segment]
    fn RemoveAll(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImageList_RemoveAll(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Replace(_obj: *u8 /* void* */, index: c_int /* int */, bitmap: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImageList_Replace(_obj, index, bitmap, mask)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(outputStream: *u8 /* void* */, mode: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTextOutputStream_Create(outputStream, mode)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxTextOutputStream_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn WriteString(self_: *u8 /* void* */, txt: *u8 /* void* */) {
        unsafe {
            wxTextOutputStream_WriteString(self_, txt)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AddText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AddText(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AddStyledText(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AddStyledText(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_InsertText(_obj: *u8 /* void* */, pos: c_int /* int */, text: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_InsertText(_obj, pos, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ClearAll(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_ClearAll(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ClearDocumentStyle(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_ClearDocumentStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLength(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetLength(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetCharAt(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetCharAt(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetCurrentPos(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetCurrentPos(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetAnchor(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetAnchor(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetStyleAt(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetStyleAt(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_Redo(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_Redo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetUndoCollection(_obj: *u8 /* void* */, collectUndo: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetUndoCollection(_obj, collectUndo)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SelectAll(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SelectAll(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetSavePoint(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SetSavePoint(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CanRedo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_CanRedo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerLineFromHandle(_obj: *u8 /* void* */, handle: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_MarkerLineFromHandle(_obj, handle)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerDeleteHandle(_obj: *u8 /* void* */, handle: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_MarkerDeleteHandle(_obj, handle)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetUndoCollection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetUndoCollection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetViewWhiteSpace(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetViewWhiteSpace(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetViewWhiteSpace(_obj: *u8 /* void* */, viewWS: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetViewWhiteSpace(_obj, viewWS)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_PositionFromPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_PositionFromPoint(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_PositionFromPointClose(_obj: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_PositionFromPointClose(_obj, x, y)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GotoLine(_obj: *u8 /* void* */, line: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_GotoLine(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GotoPos(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_GotoPos(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetAnchor(_obj: *u8 /* void* */, posAnchor: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetAnchor(_obj, posAnchor)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetEndStyled(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetEndStyled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ConvertEOLs(_obj: *u8 /* void* */, eolMode: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_ConvertEOLs(_obj, eolMode)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetEOLMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetEOLMode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetEOLMode(_obj: *u8 /* void* */, eolMode: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetEOLMode(_obj, eolMode)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StartStyling(_obj: *u8 /* void* */, pos: c_int /* int */, mask: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_StartStyling(_obj, pos, mask)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetStyling(_obj: *u8 /* void* */, length: c_int /* int */, style: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetStyling(_obj, length, style)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetBufferedDraw(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetBufferedDraw(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetBufferedDraw(_obj: *u8 /* void* */, buffered: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetBufferedDraw(_obj, buffered)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetTabWidth(_obj: *u8 /* void* */, tabWidth: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetTabWidth(_obj, tabWidth)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetTabWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetTabWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetCodePage(_obj: *u8 /* void* */, codePage: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetCodePage(_obj, codePage)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerDefine(_obj: *u8 /* void* */, markerNumber: c_int /* int */, markerSymbol: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */, arg5: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_MarkerDefine(_obj, markerNumber, markerSymbol, arg0, arg1, arg2, arg3, arg4, arg5)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerSetForeground(_obj: *u8 /* void* */, markerNumber: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_MarkerSetForeground(_obj, markerNumber, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerSetBackground(_obj: *u8 /* void* */, markerNumber: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_MarkerSetBackground(_obj, markerNumber, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerAdd(_obj: *u8 /* void* */, line: c_int /* int */, markerNumber: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_MarkerAdd(_obj, line, markerNumber)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerDelete(_obj: *u8 /* void* */, line: c_int /* int */, markerNumber: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_MarkerDelete(_obj, line, markerNumber)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerDeleteAll(_obj: *u8 /* void* */, markerNumber: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_MarkerDeleteAll(_obj, markerNumber)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerGet(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_MarkerGet(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerNext(_obj: *u8 /* void* */, lineStart: c_int /* int */, markerMask: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_MarkerNext(_obj, lineStart, markerMask)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerPrevious(_obj: *u8 /* void* */, lineStart: c_int /* int */, markerMask: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_MarkerPrevious(_obj, lineStart, markerMask)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MarkerDefineBitmap(_obj: *u8 /* void* */, markerNumber: c_int /* int */, bmp: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_MarkerDefineBitmap(_obj, markerNumber, bmp)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetMarginType(_obj: *u8 /* void* */, margin: c_int /* int */, marginType: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetMarginType(_obj, margin, marginType)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetMarginType(_obj: *u8 /* void* */, margin: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetMarginType(_obj, margin)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetMarginWidth(_obj: *u8 /* void* */, margin: c_int /* int */, pixelWidth: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetMarginWidth(_obj, margin, pixelWidth)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetMarginWidth(_obj: *u8 /* void* */, margin: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetMarginWidth(_obj, margin)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetMarginMask(_obj: *u8 /* void* */, margin: c_int /* int */, mask: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetMarginMask(_obj, margin, mask)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetMarginMask(_obj: *u8 /* void* */, margin: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetMarginMask(_obj, margin)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetMarginSensitive(_obj: *u8 /* void* */, margin: c_int /* int */, sensitive: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetMarginSensitive(_obj, margin, sensitive)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetMarginSensitive(_obj: *u8 /* void* */, margin: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetMarginSensitive(_obj, margin)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleClearAll(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_StyleClearAll(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetForeground(_obj: *u8 /* void* */, style: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_StyleSetForeground(_obj, style, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetBackground(_obj: *u8 /* void* */, style: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_StyleSetBackground(_obj, style, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetBold(_obj: *u8 /* void* */, style: c_int /* int */, bold: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_StyleSetBold(_obj, style, bold)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetItalic(_obj: *u8 /* void* */, style: c_int /* int */, italic: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_StyleSetItalic(_obj, style, italic)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetSize(_obj: *u8 /* void* */, style: c_int /* int */, sizePoints: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_StyleSetSize(_obj, style, sizePoints)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetFaceName(_obj: *u8 /* void* */, style: c_int /* int */, fontName: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_StyleSetFaceName(_obj, style, fontName)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetEOLFilled(_obj: *u8 /* void* */, style: c_int /* int */, filled: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_StyleSetEOLFilled(_obj, style, filled)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleResetDefault(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_StyleResetDefault(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetUnderline(_obj: *u8 /* void* */, style: c_int /* int */, underline: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_StyleSetUnderline(_obj, style, underline)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetCase(_obj: *u8 /* void* */, style: c_int /* int */, caseForce: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_StyleSetCase(_obj, style, caseForce)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetCharacterSet(_obj: *u8 /* void* */, style: c_int /* int */, characterSet: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_StyleSetCharacterSet(_obj, style, characterSet)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetHotSpot(_obj: *u8 /* void* */, style: c_int /* int */, hotspot: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_StyleSetHotSpot(_obj, style, hotspot)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetSelForeground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetSelForeground(_obj, useSetting, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetSelBackground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetSelBackground(_obj, useSetting, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetCaretForeground(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetCaretForeground(_obj, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CmdKeyAssign(_obj: *u8 /* void* */, key: c_int /* int */, modifiers: c_int /* int */, cmd: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_CmdKeyAssign(_obj, key, modifiers, cmd)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CmdKeyClear(_obj: *u8 /* void* */, key: c_int /* int */, modifiers: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_CmdKeyClear(_obj, key, modifiers)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CmdKeyClearAll(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_CmdKeyClearAll(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetStyleBytes(_obj: *u8 /* void* */, length: c_int /* int */, styleBytes: *char /* char* */) {
        unsafe {
            wxStyledTextCtrl_SetStyleBytes(_obj, length, styleBytes)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetVisible(_obj: *u8 /* void* */, style: c_int /* int */, visible: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_StyleSetVisible(_obj, style, visible)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetCaretPeriod(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetCaretPeriod(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetCaretPeriod(_obj: *u8 /* void* */, periodMilliseconds: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetCaretPeriod(_obj, periodMilliseconds)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetWordChars(_obj: *u8 /* void* */, characters: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SetWordChars(_obj, characters)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_BeginUndoAction(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_BeginUndoAction(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_EndUndoAction(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_EndUndoAction(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_IndicatorSetStyle(_obj: *u8 /* void* */, indic: c_int /* int */, style: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_IndicatorSetStyle(_obj, indic, style)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_IndicatorGetStyle(_obj: *u8 /* void* */, indic: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_IndicatorGetStyle(_obj, indic)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_IndicatorSetForeground(_obj: *u8 /* void* */, indic: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_IndicatorSetForeground(_obj, indic, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetWhitespaceForeground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetWhitespaceForeground(_obj, useSetting, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetWhitespaceBackground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetWhitespaceBackground(_obj, useSetting, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetStyleBits(_obj: *u8 /* void* */, bits: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetStyleBits(_obj, bits)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetStyleBits(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetStyleBits(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetLineState(_obj: *u8 /* void* */, line: c_int /* int */, state: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetLineState(_obj, line, state)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLineState(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetLineState(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetMaxLineState(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetMaxLineState(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetCaretLineVisible(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetCaretLineVisible(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetCaretLineVisible(_obj: *u8 /* void* */, show: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetCaretLineVisible(_obj, show)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetChangeable(_obj: *u8 /* void* */, style: c_int /* int */, changeable: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_StyleSetChangeable(_obj, style, changeable)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompShow(_obj: *u8 /* void* */, lenEntered: c_int /* int */, itemList: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AutoCompShow(_obj, lenEntered, itemList)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompCancel(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AutoCompCancel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompActive(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_AutoCompActive(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompPosStart(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_AutoCompPosStart(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompComplete(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AutoCompComplete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompStops(_obj: *u8 /* void* */, characterSet: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AutoCompStops(_obj, characterSet)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompSetSeparator(_obj: *u8 /* void* */, separatorCharacter: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_AutoCompSetSeparator(_obj, separatorCharacter)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompGetSeparator(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_AutoCompGetSeparator(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompSelect(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AutoCompSelect(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompSetCancelAtStart(_obj: *u8 /* void* */, cancel: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_AutoCompSetCancelAtStart(_obj, cancel)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompGetCancelAtStart(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_AutoCompGetCancelAtStart(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompSetFillUps(_obj: *u8 /* void* */, characterSet: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AutoCompSetFillUps(_obj, characterSet)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompSetChooseSingle(_obj: *u8 /* void* */, chooseSingle: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_AutoCompSetChooseSingle(_obj, chooseSingle)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompGetChooseSingle(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_AutoCompGetChooseSingle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompSetIgnoreCase(_obj: *u8 /* void* */, ignoreCase: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_AutoCompSetIgnoreCase(_obj, ignoreCase)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompGetIgnoreCase(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_AutoCompGetIgnoreCase(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_UserListShow(_obj: *u8 /* void* */, listType: c_int /* int */, itemList: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_UserListShow(_obj, listType, itemList)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompSetAutoHide(_obj: *u8 /* void* */, autoHide: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_AutoCompSetAutoHide(_obj, autoHide)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompGetAutoHide(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_AutoCompGetAutoHide(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompSetDropRestOfWord(_obj: *u8 /* void* */, dropRestOfWord: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_AutoCompSetDropRestOfWord(_obj, dropRestOfWord)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompGetDropRestOfWord(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_AutoCompGetDropRestOfWord(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_RegisterImage(_obj: *u8 /* void* */, type_: c_int /* int */, bmp: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_RegisterImage(_obj, type_, bmp)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ClearRegisteredImages(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_ClearRegisteredImages(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompGetTypeSeparator(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_AutoCompGetTypeSeparator(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AutoCompSetTypeSeparator(_obj: *u8 /* void* */, separatorCharacter: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_AutoCompSetTypeSeparator(_obj, separatorCharacter)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetIndent(_obj: *u8 /* void* */, indentSize: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetIndent(_obj, indentSize)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetIndent(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetIndent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetUseTabs(_obj: *u8 /* void* */, useTabs: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetUseTabs(_obj, useTabs)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetUseTabs(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetUseTabs(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetLineIndentation(_obj: *u8 /* void* */, line: c_int /* int */, indentSize: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetLineIndentation(_obj, line, indentSize)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLineIndentation(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetLineIndentation(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLineIndentPosition(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetLineIndentPosition(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetColumn(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetColumn(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetUseHorizontalScrollBar(_obj: *u8 /* void* */, show: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetUseHorizontalScrollBar(_obj, show)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetUseHorizontalScrollBar(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetUseHorizontalScrollBar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetIndentationGuides(_obj: *u8 /* void* */, show: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetIndentationGuides(_obj, show)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetIndentationGuides(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetIndentationGuides(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetHighlightGuide(_obj: *u8 /* void* */, column: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetHighlightGuide(_obj, column)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetHighlightGuide(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetHighlightGuide(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLineEndPosition(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetLineEndPosition(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetCodePage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetCodePage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetReadOnly(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetReadOnly(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetCurrentPos(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetCurrentPos(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetSelectionStart(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetSelectionStart(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetSelectionStart(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetSelectionStart(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetSelectionEnd(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetSelectionEnd(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetSelectionEnd(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetSelectionEnd(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetPrintMagnification(_obj: *u8 /* void* */, magnification: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetPrintMagnification(_obj, magnification)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetPrintMagnification(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetPrintMagnification(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetPrintColourMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetPrintColourMode(_obj, mode)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetPrintColourMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetPrintColourMode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_FindText(_obj: *u8 /* void* */, minPos: c_int /* int */, maxPos: c_int /* int */, text: *u8 /* void* */, flags: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_FindText(_obj, minPos, maxPos, text, flags)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_FormatRange(_obj: *u8 /* void* */, doDraw: bool /* bool */, startPos: c_int /* int */, endPos: c_int /* int */, draw: *u8 /* void* */, target: *u8 /* void* */, renderRect: *u8 /* void* */, pageRect: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_FormatRange(_obj, doDraw, startPos, endPos, draw, target, renderRect, pageRect)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetFirstVisibleLine(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetFirstVisibleLine(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLineCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetLineCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetMarginLeft(_obj: *u8 /* void* */, pixelWidth: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetMarginLeft(_obj, pixelWidth)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetMarginLeft(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetMarginLeft(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetMarginRight(_obj: *u8 /* void* */, pixelWidth: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetMarginRight(_obj, pixelWidth)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetMarginRight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetMarginRight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetModify(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetModify(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetSelection(_obj: *u8 /* void* */, start: c_int /* int */, end: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetSelection(_obj, start, end)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_HideSelection(_obj: *u8 /* void* */, normal: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_HideSelection(_obj, normal)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LineFromPosition(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_LineFromPosition(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_PositionFromLine(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_PositionFromLine(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LineScroll(_obj: *u8 /* void* */, columns: c_int /* int */, lines: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_LineScroll(_obj, columns, lines)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_EnsureCaretVisible(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_EnsureCaretVisible(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ReplaceSelection(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_ReplaceSelection(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetReadOnly(_obj: *u8 /* void* */, readOnly: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetReadOnly(_obj, readOnly)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CanPaste(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_CanPaste(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CanUndo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_CanUndo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_EmptyUndoBuffer(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_EmptyUndoBuffer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_Undo(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_Undo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_Cut(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_Cut(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_Copy(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_Copy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_Paste(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_Paste(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_Clear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SetText(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetTextLength(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetTextLength(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetOvertype(_obj: *u8 /* void* */, overtype: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetOvertype(_obj, overtype)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetOvertype(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetOvertype(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetCaretWidth(_obj: *u8 /* void* */, pixelWidth: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetCaretWidth(_obj, pixelWidth)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetCaretWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetCaretWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetTargetStart(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetTargetStart(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetTargetStart(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetTargetStart(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetTargetEnd(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetTargetEnd(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetTargetEnd(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetTargetEnd(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ReplaceTarget(_obj: *u8 /* void* */, text: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_ReplaceTarget(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ReplaceTargetRE(_obj: *u8 /* void* */, text: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_ReplaceTargetRE(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SearchInTarget(_obj: *u8 /* void* */, text: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_SearchInTarget(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetSearchFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetSearchFlags(_obj, flags)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetSearchFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetSearchFlags(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CallTipShow(_obj: *u8 /* void* */, pos: c_int /* int */, definition: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_CallTipShow(_obj, pos, definition)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CallTipCancel(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_CallTipCancel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CallTipActive(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_CallTipActive(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CallTipPosAtStart(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_CallTipPosAtStart(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CallTipSetHighlight(_obj: *u8 /* void* */, start: c_int /* int */, end: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_CallTipSetHighlight(_obj, start, end)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CallTipSetBackground(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_CallTipSetBackground(_obj, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CallTipSetForeground(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_CallTipSetForeground(_obj, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CallTipSetForegroundHighlight(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_CallTipSetForegroundHighlight(_obj, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_VisibleFromDocLine(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_VisibleFromDocLine(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_DocLineFromVisible(_obj: *u8 /* void* */, lineDisplay: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_DocLineFromVisible(_obj, lineDisplay)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetFoldLevel(_obj: *u8 /* void* */, line: c_int /* int */, level: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetFoldLevel(_obj, line, level)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetFoldLevel(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetFoldLevel(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLastChild(_obj: *u8 /* void* */, line: c_int /* int */, level: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetLastChild(_obj, line, level)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetFoldParent(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetFoldParent(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ShowLines(_obj: *u8 /* void* */, lineStart: c_int /* int */, lineEnd: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_ShowLines(_obj, lineStart, lineEnd)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_HideLines(_obj: *u8 /* void* */, lineStart: c_int /* int */, lineEnd: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_HideLines(_obj, lineStart, lineEnd)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLineVisible(_obj: *u8 /* void* */, line: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetLineVisible(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetFoldExpanded(_obj: *u8 /* void* */, line: c_int /* int */, expanded: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetFoldExpanded(_obj, line, expanded)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetFoldExpanded(_obj: *u8 /* void* */, line: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetFoldExpanded(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ToggleFold(_obj: *u8 /* void* */, line: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_ToggleFold(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_EnsureVisible(_obj: *u8 /* void* */, line: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_EnsureVisible(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetFoldFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetFoldFlags(_obj, flags)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_EnsureVisibleEnforcePolicy(_obj: *u8 /* void* */, line: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_EnsureVisibleEnforcePolicy(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetTabIndents(_obj: *u8 /* void* */, tabIndents: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetTabIndents(_obj, tabIndents)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetTabIndents(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetTabIndents(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetBackSpaceUnIndents(_obj: *u8 /* void* */, bsUnIndents: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetBackSpaceUnIndents(_obj, bsUnIndents)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetBackSpaceUnIndents(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetBackSpaceUnIndents(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetMouseDwellTime(_obj: *u8 /* void* */, periodMilliseconds: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetMouseDwellTime(_obj, periodMilliseconds)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetMouseDwellTime(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetMouseDwellTime(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_WordStartPosition(_obj: *u8 /* void* */, pos: c_int /* int */, onlyWordCharacters: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_WordStartPosition(_obj, pos, onlyWordCharacters)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_WordEndPosition(_obj: *u8 /* void* */, pos: c_int /* int */, onlyWordCharacters: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_WordEndPosition(_obj, pos, onlyWordCharacters)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetWrapMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetWrapMode(_obj, mode)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetWrapMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetWrapMode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetLayoutCache(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetLayoutCache(_obj, mode)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLayoutCache(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetLayoutCache(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetScrollWidth(_obj: *u8 /* void* */, pixelWidth: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetScrollWidth(_obj, pixelWidth)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetScrollWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetScrollWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_TextWidth(_obj: *u8 /* void* */, style: c_int /* int */, text: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_TextWidth(_obj, style, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetEndAtLastLine(_obj: *u8 /* void* */, endAtLastLine: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetEndAtLastLine(_obj, endAtLastLine)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetEndAtLastLine(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetEndAtLastLine(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_TextHeight(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_TextHeight(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetUseVerticalScrollBar(_obj: *u8 /* void* */, show: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetUseVerticalScrollBar(_obj, show)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetUseVerticalScrollBar(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetUseVerticalScrollBar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AppendText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AppendText(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetTwoPhaseDraw(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetTwoPhaseDraw(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetTwoPhaseDraw(_obj: *u8 /* void* */, twoPhase: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetTwoPhaseDraw(_obj, twoPhase)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_TargetFromSelection(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_TargetFromSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LinesJoin(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_LinesJoin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LinesSplit(_obj: *u8 /* void* */, pixelWidth: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_LinesSplit(_obj, pixelWidth)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetFoldMarginColour(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetFoldMarginColour(_obj, useSetting, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetFoldMarginHiColour(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetFoldMarginHiColour(_obj, useSetting, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LineDuplicate(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_LineDuplicate(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_HomeDisplay(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_HomeDisplay(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_HomeDisplayExtend(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_HomeDisplayExtend(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LineEndDisplay(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_LineEndDisplay(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LineEndDisplayExtend(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_LineEndDisplayExtend(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LineCopy(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_LineCopy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_MoveCaretInsideView(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_MoveCaretInsideView(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LineLength(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_LineLength(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_BraceHighlight(_obj: *u8 /* void* */, pos1: c_int /* int */, pos2: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_BraceHighlight(_obj, pos1, pos2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_BraceBadLight(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_BraceBadLight(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_BraceMatch(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_BraceMatch(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetViewEOL(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetViewEOL(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetViewEOL(_obj: *u8 /* void* */, visible: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetViewEOL(_obj, visible)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetDocPointer(_obj: *u8 /* void* */, docPointer: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SetDocPointer(_obj, docPointer)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetModEventMask(_obj: *u8 /* void* */, mask: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetModEventMask(_obj, mask)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetEdgeColumn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetEdgeColumn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetEdgeColumn(_obj: *u8 /* void* */, column: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetEdgeColumn(_obj, column)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetEdgeMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetEdgeMode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetEdgeMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetEdgeMode(_obj, mode)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetEdgeColour(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetEdgeColour(_obj, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SearchAnchor(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SearchAnchor(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SearchNext(_obj: *u8 /* void* */, flags: c_int /* int */, text: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_SearchNext(_obj, flags, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SearchPrev(_obj: *u8 /* void* */, flags: c_int /* int */, text: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_SearchPrev(_obj, flags, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LinesOnScreen(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_LinesOnScreen(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_UsePopUp(_obj: *u8 /* void* */, allowPopUp: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_UsePopUp(_obj, allowPopUp)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SelectionIsRectangle(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_SelectionIsRectangle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetZoom(_obj: *u8 /* void* */, zoom: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetZoom(_obj, zoom)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetZoom(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetZoom(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_AddRefDocument(_obj: *u8 /* void* */, docPointer: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_AddRefDocument(_obj, docPointer)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ReleaseDocument(_obj: *u8 /* void* */, docPointer: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_ReleaseDocument(_obj, docPointer)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetModEventMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetModEventMask(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetSTCFocus(_obj: *u8 /* void* */, focus: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetSTCFocus(_obj, focus)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetSTCFocus(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetSTCFocus(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetStatus(_obj: *u8 /* void* */, statusCode: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetStatus(_obj, statusCode)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetStatus(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetStatus(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetMouseDownCaptures(_obj: *u8 /* void* */, captures: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetMouseDownCaptures(_obj, captures)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetMouseDownCaptures(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetMouseDownCaptures(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetSTCCursor(_obj: *u8 /* void* */, cursorType: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetSTCCursor(_obj, cursorType)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetSTCCursor(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetSTCCursor(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetControlCharSymbol(_obj: *u8 /* void* */, symbol: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetControlCharSymbol(_obj, symbol)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetControlCharSymbol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetControlCharSymbol(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_WordPartLeft(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_WordPartLeft(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_WordPartLeftExtend(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_WordPartLeftExtend(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_WordPartRight(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_WordPartRight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_WordPartRightExtend(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_WordPartRightExtend(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetVisiblePolicy(_obj: *u8 /* void* */, visiblePolicy: c_int /* int */, visibleSlop: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetVisiblePolicy(_obj, visiblePolicy, visibleSlop)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_DelLineLeft(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_DelLineLeft(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_DelLineRight(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_DelLineRight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetXOffset(_obj: *u8 /* void* */, newOffset: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetXOffset(_obj, newOffset)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetXOffset(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetXOffset(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ChooseCaretX(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_ChooseCaretX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetXCaretPolicy(_obj: *u8 /* void* */, caretPolicy: c_int /* int */, caretSlop: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetXCaretPolicy(_obj, caretPolicy, caretSlop)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetYCaretPolicy(_obj: *u8 /* void* */, caretPolicy: c_int /* int */, caretSlop: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetYCaretPolicy(_obj, caretPolicy, caretSlop)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetPrintWrapMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetPrintWrapMode(_obj, mode)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetPrintWrapMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetPrintWrapMode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetHotspotActiveForeground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetHotspotActiveForeground(_obj, useSetting, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetHotspotActiveBackground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetHotspotActiveBackground(_obj, useSetting, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetHotspotActiveUnderline(_obj: *u8 /* void* */, underline: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetHotspotActiveUnderline(_obj, underline)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_PositionBefore(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_PositionBefore(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_PositionAfter(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_PositionAfter(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CopyRange(_obj: *u8 /* void* */, start: c_int /* int */, end: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_CopyRange(_obj, start, end)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CopyText(_obj: *u8 /* void* */, length: c_int /* int */, text: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_CopyText(_obj, length, text)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StartRecord(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_StartRecord(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StopRecord(_obj: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_StopRecord(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetLexer(_obj: *u8 /* void* */, lexer: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetLexer(_obj, lexer)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLexer(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetLexer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_Colourise(_obj: *u8 /* void* */, start: c_int /* int */, end: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_Colourise(_obj, start, end)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetProperty(_obj: *u8 /* void* */, key: *u8 /* void* */, value: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SetProperty(_obj, key, value)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetKeyWords(_obj: *u8 /* void* */, keywordSet: c_int /* int */, keyWords: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SetKeyWords(_obj, keywordSet, keyWords)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetLexerLanguage(_obj: *u8 /* void* */, language: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SetLexerLanguage(_obj, language)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetCurrentLine(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStyledTextCtrl_GetCurrentLine(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetSpec(_obj: *u8 /* void* */, styleNum: c_int /* int */, spec: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_StyleSetSpec(_obj, styleNum, spec)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetFont(_obj: *u8 /* void* */, styleNum: c_int /* int */, font: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_StyleSetFont(_obj, styleNum, font)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_StyleSetFontAttr(_obj: *u8 /* void* */, styleNum: c_int /* int */, size: c_int /* int */, faceName: *u8 /* void* */, bold: bool /* bool */, italic: bool /* bool */, underline: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_StyleSetFontAttr(_obj, styleNum, size, faceName, bold, italic, underline)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CmdKeyExecute(_obj: *u8 /* void* */, cmd: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_CmdKeyExecute(_obj, cmd)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetMargins(_obj: *u8 /* void* */, left: c_int /* int */, right: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_SetMargins(_obj, left, right)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetSelection(_obj: *u8 /* void* */, startPos: *c_int /* int* */, endPos: *c_int /* int* */) {
        unsafe {
            wxStyledTextCtrl_GetSelection(_obj, startPos, endPos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ScrollToLine(_obj: *u8 /* void* */, line: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_ScrollToLine(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_ScrollToColumn(_obj: *u8 /* void* */, column: c_int /* int */) {
        unsafe {
            wxStyledTextCtrl_ScrollToColumn(_obj, column)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetVScrollBar(_obj: *u8 /* void* */, bar: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SetVScrollBar(_obj, bar)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetHScrollBar(_obj: *u8 /* void* */, bar: *u8 /* void* */) {
        unsafe {
            wxStyledTextCtrl_SetHScrollBar(_obj, bar)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLastKeydownProcessed(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_GetLastKeydownProcessed(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetLastKeydownProcessed(_obj: *u8 /* void* */, val: bool /* bool */) {
        unsafe {
            wxStyledTextCtrl_SetLastKeydownProcessed(_obj, val)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SaveFile(_obj: *u8 /* void* */, filename: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_SaveFile(_obj, filename)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_LoadFile(_obj: *u8 /* void* */, filename: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxStyledTextCtrl_LoadFile(_obj, filename)
        }
    }
}
trait wxDynamicSashWindow {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicSashWindow_Create(parent, id, arg0, arg1, arg2, arg3, style)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDynamicSashWindow_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetHScrollBar(_obj: *u8 /* void* */, child: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicSashWindow_GetHScrollBar(_obj, child)
        }
    }
    #[fixed_stack_segment]
    fn GetVScrollBar(_obj: *u8 /* void* */, child: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDynamicSashWindow_GetVScrollBar(_obj, child)
        }
    }
}
trait wxTimerRunner {
}
trait wxMDIChildFrame {
    #[fixed_stack_segment]
    fn Activate(_obj: *u8 /* void* */) {
        unsafe {
            wxMDIChildFrame_Activate(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMDIChildFrame_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait wxGridEditorCreatedEvent {
    #[fixed_stack_segment]
    fn GetCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridEditorCreatedEvent_GetCol(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetControl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridEditorCreatedEvent_GetControl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridEditorCreatedEvent_GetRow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCol(_obj: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxGridEditorCreatedEvent_SetCol(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetControl(_obj: *u8 /* void* */, ctrl: *u8 /* void* */) {
        unsafe {
            wxGridEditorCreatedEvent_SetControl(_obj, ctrl)
        }
    }
    #[fixed_stack_segment]
    fn SetRow(_obj: *u8 /* void* */, row: c_int /* int */) {
        unsafe {
            wxGridEditorCreatedEvent_SetRow(_obj, row)
        }
    }
}
trait wxPostScriptDC {
    #[fixed_stack_segment]
    fn Create(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPostScriptDC_Create(data)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxPostScriptDC_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn SetResolution(self_: *u8 /* void* */, ppi: c_int /* int */) {
        unsafe {
            wxPostScriptDC_SetResolution(self_, ppi)
        }
    }
    #[fixed_stack_segment]
    fn GetResolution(self_: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPostScriptDC_GetResolution(self_)
        }
    }
}
trait wxcPrintEvent {
}
trait wxColourDialog {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, col: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxColourDialog_Create(_prt, col)
        }
    }
    #[fixed_stack_segment]
    fn GetColourData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxColourDialog_GetColourData(_obj, _ref)
        }
    }
}
trait wxSplitterWindow {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSplitterWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetBorderSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetBorderSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinimumPaneSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetMinimumPaneSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSashPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetSashPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSashSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetSashSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSplitMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSplitterWindow_GetSplitMode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWindow1(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSplitterWindow_GetWindow1(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWindow2(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSplitterWindow_GetWindow2(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Initialize(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSplitterWindow_Initialize(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn IsSplit(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_IsSplit(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ReplaceWindow(_obj: *u8 /* void* */, winOld: *u8 /* void* */, winNew: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_ReplaceWindow(_obj, winOld, winNew)
        }
    }
    #[fixed_stack_segment]
    fn SetBorderSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxSplitterWindow_SetBorderSize(_obj, width)
        }
    }
    #[fixed_stack_segment]
    fn SetMinimumPaneSize(_obj: *u8 /* void* */, min: c_int /* int */) {
        unsafe {
            wxSplitterWindow_SetMinimumPaneSize(_obj, min)
        }
    }
    #[fixed_stack_segment]
    fn SetSashPosition(_obj: *u8 /* void* */, position: c_int /* int */, redraw: bool /* bool */) {
        unsafe {
            wxSplitterWindow_SetSashPosition(_obj, position, redraw)
        }
    }
    #[fixed_stack_segment]
    fn SetSashSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxSplitterWindow_SetSashSize(_obj, width)
        }
    }
    #[fixed_stack_segment]
    fn SetSplitMode(_obj: *u8 /* void* */, mode: c_int /* int */) {
        unsafe {
            wxSplitterWindow_SetSplitMode(_obj, mode)
        }
    }
    #[fixed_stack_segment]
    fn SplitHorizontally(_obj: *u8 /* void* */, window1: *u8 /* void* */, window2: *u8 /* void* */, sashPosition: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_SplitHorizontally(_obj, window1, window2, sashPosition)
        }
    }
    #[fixed_stack_segment]
    fn SplitVertically(_obj: *u8 /* void* */, window1: *u8 /* void* */, window2: *u8 /* void* */, sashPosition: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_SplitVertically(_obj, window1, window2, sashPosition)
        }
    }
    #[fixed_stack_segment]
    fn Unsplit(_obj: *u8 /* void* */, toRemove: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSplitterWindow_Unsplit(_obj, toRemove)
        }
    }
    #[fixed_stack_segment]
    fn GetSashGravity(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxSplitterWindow_GetSashGravity(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetSashGravity(_obj: *u8 /* void* */, gravity: c_double /* double */) {
        unsafe {
            wxSplitterWindow_SetSashGravity(_obj, gravity)
        }
    }
}
trait wxWindowDC {
    #[fixed_stack_segment]
    fn Create(win: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindowDC_Create(win)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxWindowDC_Delete(_obj)
        }
    }
}
trait wxRegion {
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, region: *u8 /* void* */) {
        unsafe {
            wxRegion_Assign(_obj, region)
        }
    }
    #[fixed_stack_segment]
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxRegion_Clear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ContainsPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_ContainsPoint(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn ContainsRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_ContainsRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxRegion_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromRect(arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxRegion_CreateFromRect(arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxRegion_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsEmpty(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegion_IsEmpty(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBox(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxRegion_GetBox(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn IntersectRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_IntersectRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn IntersectRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegion_IntersectRegion(_obj, region)
        }
    }
    #[fixed_stack_segment]
    fn SubtractRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_SubtractRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn SubtractRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegion_SubtractRegion(_obj, region)
        }
    }
    #[fixed_stack_segment]
    fn UnionRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_UnionRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn UnionRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRegion_UnionRegion(_obj, region)
        }
    }
    #[fixed_stack_segment]
    fn XorRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxRegion_XorRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMiniFrame_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait wxGridCellCoordsArray {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxGridCellCoordsArray_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellCoordsArray_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridCellCoordsArray_GetCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Item(_obj: *u8 /* void* */, _idx: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGridCellCoordsArray_Item(_obj, _idx, arg0, arg1)
        }
    }
}
trait wxSocketEvent {
}
trait wxGridCellTextEnterEditor {
    #[fixed_stack_segment]
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellTextEnterEditor_Ctor()
        }
    }
    #[fixed_stack_segment]
    fn wxLogStderr_Create() -> *u8 /* void* */ {
        unsafe {
            wxLogStderr_Create()
        }
    }
    #[fixed_stack_segment]
    fn wxLogStderr_CreateStdOut() -> *u8 /* void* */ {
        unsafe {
            wxLogStderr_CreateStdOut()
        }
    }
    #[fixed_stack_segment]
    fn wxLogNull_Create() -> *u8 /* void* */ {
        unsafe {
            wxLogNull_Create()
        }
    }
    #[fixed_stack_segment]
    fn wxLogTextCtrl_Create(text: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLogTextCtrl_Create(text)
        }
    }
    #[fixed_stack_segment]
    fn wxLogWindow_Create(parent: *u8 /* void* */, title: *wchar_t /* wchar_t* */, showit: bool /* bool */, passthrough: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxLogWindow_Create(parent, title, showit, passthrough)
        }
    }
    #[fixed_stack_segment]
    fn wxLogWindow_GetFrame(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLogWindow_GetFrame(obj)
        }
    }
    #[fixed_stack_segment]
    fn LogError(_msg: *u8 /* void* */) {
        unsafe {
            LogError(_msg)
        }
    }
    #[fixed_stack_segment]
    fn LogFatalError(_msg: *u8 /* void* */) {
        unsafe {
            LogFatalError(_msg)
        }
    }
    #[fixed_stack_segment]
    fn LogWarning(_msg: *u8 /* void* */) {
        unsafe {
            LogWarning(_msg)
        }
    }
    #[fixed_stack_segment]
    fn LogMessage(_msg: *u8 /* void* */) {
        unsafe {
            LogMessage(_msg)
        }
    }
    #[fixed_stack_segment]
    fn LogVerbose(_msg: *u8 /* void* */) {
        unsafe {
            LogVerbose(_msg)
        }
    }
    #[fixed_stack_segment]
    fn LogStatus(_msg: *u8 /* void* */) {
        unsafe {
            LogStatus(_msg)
        }
    }
    #[fixed_stack_segment]
    fn LogSysError(_msg: *u8 /* void* */) {
        unsafe {
            LogSysError(_msg)
        }
    }
    #[fixed_stack_segment]
    fn LogDebug(_msg: *u8 /* void* */) {
        unsafe {
            LogDebug(_msg)
        }
    }
    #[fixed_stack_segment]
    fn LogTrace(mask: *u8 /* void* */, _msg: *u8 /* void* */) {
        unsafe {
            LogTrace(mask, _msg)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_AddTraceMask(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxLog_AddTraceMask(_obj, str)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_DontCreateOnDemand(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_DontCreateOnDemand(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_Flush(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_Flush(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_FlushActive(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_FlushActive(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_GetActiveTarget() -> *u8 /* void* */ {
        unsafe {
            wxLog_GetActiveTarget()
        }
    }
    #[fixed_stack_segment]
    fn wxLog_GetTimestamp(_obj: *u8 /* void* */) -> *char /* char* */ {
        unsafe {
            wxLog_GetTimestamp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_GetTraceMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxLog_GetTraceMask(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_GetVerbose(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxLog_GetVerbose(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_HasPendingMessages(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLog_HasPendingMessages(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_IsAllowedTraceMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLog_IsAllowedTraceMask(_obj, mask)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_OnLog(_obj: *u8 /* void* */, level: c_int /* int */, szString: *wchar_t /* wchar_t* */, t: c_int /* int */) {
        unsafe {
            wxLog_OnLog(_obj, level, szString, t)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_RemoveTraceMask(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxLog_RemoveTraceMask(_obj, str)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_Resume(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_Resume(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_SetActiveTarget(pLogger: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxLog_SetActiveTarget(pLogger)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_SetTimestamp(_obj: *u8 /* void* */, ts: *wchar_t /* wchar_t* */) {
        unsafe {
            wxLog_SetTimestamp(_obj, ts)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_SetTraceMask(_obj: *u8 /* void* */, ulMask: c_int /* int */) {
        unsafe {
            wxLog_SetTraceMask(_obj, ulMask)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_SetVerbose(_obj: *u8 /* void* */, bVerbose: c_int /* int */) {
        unsafe {
            wxLog_SetVerbose(_obj, bVerbose)
        }
    }
    #[fixed_stack_segment]
    fn wxLog_Suspend(_obj: *u8 /* void* */) {
        unsafe {
            wxLog_Suspend(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxProcess_Open(cmd: *u8 /* void* */, flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_Open(cmd, flags)
        }
    }
    #[fixed_stack_segment]
    fn wxProcess_IsErrorAvailable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProcess_IsErrorAvailable(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxProcess_IsInputAvailable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProcess_IsInputAvailable(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxProcess_IsInputOpened(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProcess_IsInputOpened(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxKill(pid: c_int /* int */, signal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxKill(pid, signal)
        }
    }
    #[fixed_stack_segment]
    fn wxStreamBase_Delete(obj: *u8 /* void* */) {
        unsafe {
            wxStreamBase_Delete(obj)
        }
    }
    #[fixed_stack_segment]
    fn wxGetColourFromUser(parent: *u8 /* void* */, colInit: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGetColourFromUser(parent, colInit, colour)
        }
    }
    #[fixed_stack_segment]
    fn wxGetFontFromUser(parent: *u8 /* void* */, fontInit: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxGetFontFromUser(parent, fontInit, font)
        }
    }
    #[fixed_stack_segment]
    fn wxGetPasswordFromUser(message: *wchar_t /* wchar_t* */, caption: *wchar_t /* wchar_t* */, defaultText: *wchar_t /* wchar_t* */, parent: *u8 /* void* */, _buf: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxGetPasswordFromUser(message, caption, defaultText, parent, _buf)
        }
    }
    #[fixed_stack_segment]
    fn wxGetTextFromUser(message: *wchar_t /* wchar_t* */, caption: *wchar_t /* wchar_t* */, defaultText: *wchar_t /* wchar_t* */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, center: bool /* bool */, _buf: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxGetTextFromUser(message, caption, defaultText, parent, arg0, arg1, center, _buf)
        }
    }
    #[fixed_stack_segment]
    fn wxGetNumberFromUser(message: *u8 /* void* */, prompt: *u8 /* void* */, caption: *u8 /* void* */, value: c_long /* long */, min: c_long /* long */, max: c_long /* long */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_long /* long */ {
        unsafe {
            wxGetNumberFromUser(message, prompt, caption, value, min, max, parent, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxcBell() {
        unsafe {
            wxcBell()
        }
    }
    #[fixed_stack_segment]
    fn wxcBeginBusyCursor() {
        unsafe {
            wxcBeginBusyCursor()
        }
    }
    #[fixed_stack_segment]
    fn wxcEndBusyCursor() {
        unsafe {
            wxcEndBusyCursor()
        }
    }
    #[fixed_stack_segment]
    fn wxcIsBusy() {
        unsafe {
            wxcIsBusy()
        }
    }
    #[fixed_stack_segment]
    fn wxTextCtrl_EmulateKeyPress(_obj: *u8 /* void* */, keyevent: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_EmulateKeyPress(_obj, keyevent)
        }
    }
    #[fixed_stack_segment]
    fn wxTextCtrl_GetDefaultStyle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetDefaultStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTextCtrl_GetRange(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetRange(_obj, from, to)
        }
    }
    #[fixed_stack_segment]
    fn wxTextCtrl_GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextCtrl_GetStringSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTextCtrl_IsMultiLine(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_IsMultiLine(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTextCtrl_IsSingleLine(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_IsSingleLine(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTextCtrl_SetDefaultStyle(_obj: *u8 /* void* */, style: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_SetDefaultStyle(_obj, style)
        }
    }
    #[fixed_stack_segment]
    fn wxTextCtrl_SetMaxLength(_obj: *u8 /* void* */, len: c_long /* long */) {
        unsafe {
            wxTextCtrl_SetMaxLength(_obj, len)
        }
    }
    #[fixed_stack_segment]
    fn wxTextCtrl_SetStyle(_obj: *u8 /* void* */, start: c_long /* long */, end: c_long /* long */, style: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextCtrl_SetStyle(_obj, start, end, style)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_Create(colText: *u8 /* void* */, colBack: *u8 /* void* */, font: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTextAttr_Create(colText, colBack, font)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxTextAttr_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxTextAttr_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_GetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxTextAttr_GetBackgroundColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_GetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxTextAttr_GetFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_GetTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxTextAttr_GetTextColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextAttr_HasBackgroundColour(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_HasFont(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextAttr_HasFont(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextAttr_HasTextColour(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_IsDefault(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTextAttr_IsDefault(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_SetTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxTextAttr_SetTextColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxTextAttr_SetBackgroundColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn wxTextAttr_SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxTextAttr_SetFont(_obj, font)
        }
    }
}
trait wxDebugContext {
}
trait wxLayoutAlgorithm {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxLayoutAlgorithm_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxLayoutAlgorithm_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LayoutFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLayoutAlgorithm_LayoutFrame(_obj, frame, mainWindow)
        }
    }
    #[fixed_stack_segment]
    fn LayoutMDIFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, use_: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxLayoutAlgorithm_LayoutMDIFrame(_obj, frame, arg0, arg1, arg2, arg3, use_)
        }
    }
    #[fixed_stack_segment]
    fn LayoutWindow(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxLayoutAlgorithm_LayoutWindow(_obj, frame, mainWindow)
        }
    }
}
trait wxStringProperty {
    #[fixed_stack_segment]
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStringProperty_Create(label, name, value)
        }
    }
}
trait wxObjectRefData {
}
trait wxRadioBox {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _dim: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxRadioBox_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, arg4, arg5, _dim, _stl)
        }
    }
    #[fixed_stack_segment]
    fn EnableItem(_obj: *u8 /* void* */, item: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxRadioBox_EnableItem(_obj, item, enable)
        }
    }
    #[fixed_stack_segment]
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRadioBox_FindString(_obj, s)
        }
    }
    #[fixed_stack_segment]
    fn GetItemLabel(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxRadioBox_GetItemLabel(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberOfRowsOrCols(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRadioBox_GetNumberOfRowsOrCols(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRadioBox_GetSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxRadioBox_GetStringSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Number(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxRadioBox_Number(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetItemBitmap(_obj: *u8 /* void* */, item: c_int /* int */, bitmap: *u8 /* void* */) {
        unsafe {
            wxRadioBox_SetItemBitmap(_obj, item, bitmap)
        }
    }
    #[fixed_stack_segment]
    fn SetItemLabel(_obj: *u8 /* void* */, item: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxRadioBox_SetItemLabel(_obj, item, label)
        }
    }
    #[fixed_stack_segment]
    fn SetNumberOfRowsOrCols(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxRadioBox_SetNumberOfRowsOrCols(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn SetSelection(_obj: *u8 /* void* */, _n: c_int /* int */) {
        unsafe {
            wxRadioBox_SetSelection(_obj, _n)
        }
    }
    #[fixed_stack_segment]
    fn SetStringSelection(_obj: *u8 /* void* */, s: *u8 /* void* */) {
        unsafe {
            wxRadioBox_SetStringSelection(_obj, s)
        }
    }
    #[fixed_stack_segment]
    fn ShowItem(_obj: *u8 /* void* */, item: c_int /* int */, show: bool /* bool */) {
        unsafe {
            wxRadioBox_ShowItem(_obj, item, show)
        }
    }
}
trait wxArrayString {
}
trait wxTopLevelWindow {
    #[fixed_stack_segment]
    fn EnableCloseButton(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_EnableCloseButton(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultButton(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTopLevelWindow_GetDefaultButton(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultItem(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTopLevelWindow_GetDefaultItem(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetIcon(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTopLevelWindow_GetIcon(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTopLevelWindow_GetTitle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Iconize(_obj: *u8 /* void* */, iconize: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_Iconize(_obj, iconize)
        }
    }
    #[fixed_stack_segment]
    fn IsActive(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_IsActive(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsIconized(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_IsIconized(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsMaximized(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTopLevelWindow_IsMaximized(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Maximize(_obj: *u8 /* void* */, maximize: bool /* bool */) {
        unsafe {
            wxTopLevelWindow_Maximize(_obj, maximize)
        }
    }
    #[fixed_stack_segment]
    fn RequestUserAttention(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxTopLevelWindow_RequestUserAttention(_obj, flags)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultButton(_obj: *u8 /* void* */, pBut: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetDefaultButton(_obj, pBut)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultItem(_obj: *u8 /* void* */, pBut: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetDefaultItem(_obj, pBut)
        }
    }
    #[fixed_stack_segment]
    fn SetIcon(_obj: *u8 /* void* */, pIcon: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetIcon(_obj, pIcon)
        }
    }
    #[fixed_stack_segment]
    fn SetIcons(_obj: *u8 /* void* */, _icons: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetIcons(_obj, _icons)
        }
    }
    #[fixed_stack_segment]
    fn SetMaxSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxTopLevelWindow_SetMaxSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetMinSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxTopLevelWindow_SetMinSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetTitle(_obj: *u8 /* void* */, pString: *u8 /* void* */) {
        unsafe {
            wxTopLevelWindow_SetTitle(_obj, pString)
        }
    }
}
trait ELJConnection {
    #[fixed_stack_segment]
    fn Advise(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */, size: c_int /* int */, format: c_int /* int */) -> c_int /* int */ {
        unsafe {
            ELJConnection_Advise(_obj, item, data, size, format)
        }
    }
    #[fixed_stack_segment]
    fn Compress(_obj: *u8 /* void* */, on: c_int /* int */) {
        unsafe {
            ELJConnection_Compress(_obj, on)
        }
    }
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJConnection_Create(_obj, buffer, size)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJConnection_CreateDefault(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJConnection_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Disconnect(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJConnection_Disconnect(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Execute(_obj: *u8 /* void* */, data: *u8 /* void* */, size: c_int /* int */, format: c_int /* int */) -> bool /* bool */ {
        unsafe {
            ELJConnection_Execute(_obj, data, size, format)
        }
    }
    #[fixed_stack_segment]
    fn Poke(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */, size: c_int /* int */, format: c_int /* int */) -> bool /* bool */ {
        unsafe {
            ELJConnection_Poke(_obj, item, data, size, format)
        }
    }
    #[fixed_stack_segment]
    fn Request(_obj: *u8 /* void* */, item: *u8 /* void* */, size: *u8 /* void* */, format: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            ELJConnection_Request(_obj, item, size, format)
        }
    }
    #[fixed_stack_segment]
    fn SetOnAdvise(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) {
        unsafe {
            ELJConnection_SetOnAdvise(_obj, _fnc)
        }
    }
    #[fixed_stack_segment]
    fn SetOnDisconnect(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) {
        unsafe {
            ELJConnection_SetOnDisconnect(_obj, _fnc)
        }
    }
    #[fixed_stack_segment]
    fn SetOnExecute(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) {
        unsafe {
            ELJConnection_SetOnExecute(_obj, _fnc)
        }
    }
    #[fixed_stack_segment]
    fn SetOnPoke(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) {
        unsafe {
            ELJConnection_SetOnPoke(_obj, _fnc)
        }
    }
    #[fixed_stack_segment]
    fn SetOnRequest(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) {
        unsafe {
            ELJConnection_SetOnRequest(_obj, _fnc)
        }
    }
    #[fixed_stack_segment]
    fn SetOnStartAdvise(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) {
        unsafe {
            ELJConnection_SetOnStartAdvise(_obj, _fnc)
        }
    }
    #[fixed_stack_segment]
    fn SetOnStopAdvise(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) {
        unsafe {
            ELJConnection_SetOnStopAdvise(_obj, _fnc)
        }
    }
    #[fixed_stack_segment]
    fn StartAdvise(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJConnection_StartAdvise(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn StopAdvise(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJConnection_StopAdvise(_obj, item)
        }
    }
}
trait wxProcess {
    #[fixed_stack_segment]
    fn CloseOutput(_obj: *u8 /* void* */) {
        unsafe {
            wxProcess_CloseOutput(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault(_prt: *u8 /* void* */, _id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_CreateDefault(_prt, _id)
        }
    }
    #[fixed_stack_segment]
    fn CreateRedirect(_prt: *u8 /* void* */, _rdr: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_CreateRedirect(_prt, _rdr)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxProcess_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Detach(_obj: *u8 /* void* */) {
        unsafe {
            wxProcess_Detach(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetErrorStream(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_GetErrorStream(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetInputStream(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_GetInputStream(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOutputStream(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxProcess_GetOutputStream(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsRedirected(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxProcess_IsRedirected(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            cbRowLayoutPlugin_Create(pPanel, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbRowLayoutPlugin_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbRowLayoutPlugin_Delete(_obj)
        }
    }
}
trait cbResizeRowEvent {
    #[fixed_stack_segment]
    fn ForUpperHandle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbResizeRowEvent_ForUpperHandle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HandleOfs(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbResizeRowEvent_HandleOfs(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Row(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbResizeRowEvent_Row(_obj)
        }
    }
}
trait wxInputStream {
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxInputStream_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Eof(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxInputStream_Eof(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetC(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxInputStream_GetC(_obj)
        }
    }
    #[fixed_stack_segment]
    fn LastRead(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputStream_LastRead(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Peek(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxInputStream_Peek(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Read(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */) {
        unsafe {
            wxInputStream_Read(_obj, buffer, size)
        }
    }
    #[fixed_stack_segment]
    fn SeekI(_obj: *u8 /* void* */, pos: c_int /* int */, mode: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxInputStream_SeekI(_obj, pos, mode)
        }
    }
    #[fixed_stack_segment]
    fn Tell(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputStream_Tell(_obj)
        }
    }
    #[fixed_stack_segment]
    fn UngetBuffer(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxInputStream_UngetBuffer(_obj, buffer, size)
        }
    }
    #[fixed_stack_segment]
    fn Ungetch(_obj: *u8 /* void* */, c: wchar_t /* wchar_t */) -> c_int /* int */ {
        unsafe {
            wxInputStream_Ungetch(_obj, c)
        }
    }
}
trait wxLogGUI {
}
trait wxSlider {
    #[fixed_stack_segment]
    fn ClearSel(_obj: *u8 /* void* */) {
        unsafe {
            wxSlider_ClearSel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ClearTicks(_obj: *u8 /* void* */) {
        unsafe {
            wxSlider_ClearTicks(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _init: c_int /* int */, _min: c_int /* int */, _max: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxSlider_Create(_prt, _id, _init, _min, _max, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetLineSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetLineSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPageSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetPageSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelEnd(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetSelEnd(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelStart(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetSelStart(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetThumbLength(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetThumbLength(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTickFreq(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetTickFreq(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSlider_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetLineSize(_obj: *u8 /* void* */, lineSize: c_int /* int */) {
        unsafe {
            wxSlider_SetLineSize(_obj, lineSize)
        }
    }
    #[fixed_stack_segment]
    fn SetPageSize(_obj: *u8 /* void* */, pageSize: c_int /* int */) {
        unsafe {
            wxSlider_SetPageSize(_obj, pageSize)
        }
    }
    #[fixed_stack_segment]
    fn SetRange(_obj: *u8 /* void* */, minValue: c_int /* int */, maxValue: c_int /* int */) {
        unsafe {
            wxSlider_SetRange(_obj, minValue, maxValue)
        }
    }
    #[fixed_stack_segment]
    fn SetSelection(_obj: *u8 /* void* */, minPos: c_int /* int */, maxPos: c_int /* int */) {
        unsafe {
            wxSlider_SetSelection(_obj, minPos, maxPos)
        }
    }
    #[fixed_stack_segment]
    fn SetThumbLength(_obj: *u8 /* void* */, len: c_int /* int */) {
        unsafe {
            wxSlider_SetThumbLength(_obj, len)
        }
    }
    #[fixed_stack_segment]
    fn SetTick(_obj: *u8 /* void* */, tickPos: c_int /* int */) {
        unsafe {
            wxSlider_SetTick(_obj, tickPos)
        }
    }
    #[fixed_stack_segment]
    fn SetTickFreq(_obj: *u8 /* void* */, n: c_int /* int */, pos: c_int /* int */) {
        unsafe {
            wxSlider_SetTickFreq(_obj, n, pos)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, value: c_int /* int */) {
        unsafe {
            wxSlider_SetValue(_obj, value)
        }
    }
}
trait wxAcceleratorTable {
    #[fixed_stack_segment]
    fn Create(n: c_int /* int */, entries: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxAcceleratorTable_Create(n, entries)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxAcceleratorTable_Delete(_obj)
        }
    }
}
trait wxStringList {
}
trait wxCalendarEvent {
    #[fixed_stack_segment]
    fn GetDate(_obj: *u8 /* void* */, _dte: *u8 /* void* */) {
        unsafe {
            wxCalendarEvent_GetDate(_obj, _dte)
        }
    }
    #[fixed_stack_segment]
    fn GetWeekDay(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCalendarEvent_GetWeekDay(_obj)
        }
    }
}
trait wxPlotOnOffCurve {
    #[fixed_stack_segment]
    fn Add(_obj: *u8 /* void* */, on: c_int /* int */, off: c_int /* int */, clientData: *u8 /* void* */) {
        unsafe {
            wxPlotOnOffCurve_Add(_obj, on, off, clientData)
        }
    }
    #[fixed_stack_segment]
    fn Create(offsetY: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPlotOnOffCurve_Create(offsetY)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPlotOnOffCurve_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DrawOffLine(_obj: *u8 /* void* */, dc: *u8 /* void* */, y: c_int /* int */, start: c_int /* int */, end: c_int /* int */) {
        unsafe {
            wxPlotOnOffCurve_DrawOffLine(_obj, dc, y, start, end)
        }
    }
    #[fixed_stack_segment]
    fn DrawOnLine(_obj: *u8 /* void* */, dc: *u8 /* void* */, y: c_int /* int */, start: c_int /* int */, end: c_int /* int */, clientData: *u8 /* void* */) {
        unsafe {
            wxPlotOnOffCurve_DrawOnLine(_obj, dc, y, start, end, clientData)
        }
    }
    #[fixed_stack_segment]
    fn GetAt(_obj: *u8 /* void* */, index: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPlotOnOffCurve_GetAt(_obj, index)
        }
    }
    #[fixed_stack_segment]
    fn GetClientData(_obj: *u8 /* void* */, index: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPlotOnOffCurve_GetClientData(_obj, index)
        }
    }
    #[fixed_stack_segment]
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPlotOnOffCurve_GetCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEndX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPlotOnOffCurve_GetEndX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOff(_obj: *u8 /* void* */, index: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxPlotOnOffCurve_GetOff(_obj, index)
        }
    }
    #[fixed_stack_segment]
    fn GetOffsetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPlotOnOffCurve_GetOffsetY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOn(_obj: *u8 /* void* */, index: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxPlotOnOffCurve_GetOn(_obj, index)
        }
    }
    #[fixed_stack_segment]
    fn GetStartX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPlotOnOffCurve_GetStartX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetOffsetY(_obj: *u8 /* void* */, offsetY: c_int /* int */) {
        unsafe {
            wxPlotOnOffCurve_SetOffsetY(_obj, offsetY)
        }
    }
}
trait wxStaticText {
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn AddBook(_obj: *u8 /* void* */, book: *u8 /* void* */, show_wait_msg: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_AddBook(_obj, book, show_wait_msg)
        }
    }
    #[fixed_stack_segment]
    fn Create(_style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlHelpController_Create(_style)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Display(_obj: *u8 /* void* */, x: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxHtmlHelpController_Display(_obj, x)
        }
    }
    #[fixed_stack_segment]
    fn DisplayBlock(_obj: *u8 /* void* */, blockNo: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_DisplayBlock(_obj, blockNo)
        }
    }
    #[fixed_stack_segment]
    fn DisplayContents(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxHtmlHelpController_DisplayContents(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DisplayIndex(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxHtmlHelpController_DisplayIndex(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DisplayNumber(_obj: *u8 /* void* */, id: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxHtmlHelpController_DisplayNumber(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn DisplaySection(_obj: *u8 /* void* */, section: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_DisplaySection(_obj, section)
        }
    }
    #[fixed_stack_segment]
    fn DisplaySectionNumber(_obj: *u8 /* void* */, sectionNo: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_DisplaySectionNumber(_obj, sectionNo)
        }
    }
    #[fixed_stack_segment]
    fn GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlHelpController_GetFrame(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFrameParameters(_obj: *u8 /* void* */, title: *u8 /* void* */, width: *c_int /* int* */, height: *c_int /* int* */, pos_x: *c_int /* int* */, pos_y: *c_int /* int* */, newFrameEachTime: *c_int /* int* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlHelpController_GetFrameParameters(_obj, title, width, height, pos_x, pos_y, newFrameEachTime)
        }
    }
    #[fixed_stack_segment]
    fn Initialize(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_Initialize(_obj, file)
        }
    }
    #[fixed_stack_segment]
    fn KeywordSearch(_obj: *u8 /* void* */, keyword: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_KeywordSearch(_obj, keyword)
        }
    }
    #[fixed_stack_segment]
    fn LoadFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_LoadFile(_obj, file)
        }
    }
    #[fixed_stack_segment]
    fn Quit(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlHelpController_Quit(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ReadCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_ReadCustomization(_obj, cfg, path)
        }
    }
    #[fixed_stack_segment]
    fn SetFrameParameters(_obj: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, pos_x: c_int /* int */, pos_y: c_int /* int */, newFrameEachTime: bool /* bool */) {
        unsafe {
            wxHtmlHelpController_SetFrameParameters(_obj, title, arg0, arg1, pos_x, pos_y, newFrameEachTime)
        }
    }
    #[fixed_stack_segment]
    fn SetTempDir(_obj: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_SetTempDir(_obj, path)
        }
    }
    #[fixed_stack_segment]
    fn SetTitleFormat(_obj: *u8 /* void* */, format: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_SetTitleFormat(_obj, format)
        }
    }
    #[fixed_stack_segment]
    fn SetViewer(_obj: *u8 /* void* */, viewer: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxHtmlHelpController_SetViewer(_obj, viewer, flags)
        }
    }
    #[fixed_stack_segment]
    fn UseConfig(_obj: *u8 /* void* */, config: *u8 /* void* */, rootpath: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_UseConfig(_obj, config, rootpath)
        }
    }
    #[fixed_stack_segment]
    fn WriteCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlHelpController_WriteCustomization(_obj, cfg, path)
        }
    }
}
trait wxSocketBase {
}
trait wxRemotelyScrolledTreeCtrl {
    #[fixed_stack_segment]
    fn AdjustRemoteScrollbars(_obj: *u8 /* void* */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_AdjustRemoteScrollbars(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CalcTreeSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_CalcTreeSize(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn CalcTreeSizeItem(_obj: *u8 /* void* */, id: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_CalcTreeSizeItem(_obj, id, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _cmp: *u8 /* void* */, parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxRemotelyScrolledTreeCtrl_Create(_obj, _cmp, parent, id, arg0, arg1, arg2, arg3, style)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCompanionWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxRemotelyScrolledTreeCtrl_GetCompanionWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxRemotelyScrolledTreeCtrl_GetScrollPos(_obj, orient)
        }
    }
    #[fixed_stack_segment]
    fn GetScrolledWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxRemotelyScrolledTreeCtrl_GetScrolledWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_GetViewStart(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn HideVScrollbar(_obj: *u8 /* void* */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_HideVScrollbar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_PrepareDC(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn ScrollToLine(_obj: *u8 /* void* */, posHoriz: c_int /* int */, posVert: c_int /* int */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_ScrollToLine(_obj, posHoriz, posVert)
        }
    }
    #[fixed_stack_segment]
    fn SetCompanionWindow(_obj: *u8 /* void* */, companion: *u8 /* void* */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_SetCompanionWindow(_obj, companion)
        }
    }
    #[fixed_stack_segment]
    fn SetScrollbars(_obj: *u8 /* void* */, pixelsPerUnitX: c_int /* int */, pixelsPerUnitY: c_int /* int */, noUnitsX: c_int /* int */, noUnitsY: c_int /* int */, xPos: c_int /* int */, yPos: c_int /* int */, noRefresh: c_int /* int */) {
        unsafe {
            wxRemotelyScrolledTreeCtrl_SetScrollbars(_obj, pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh)
        }
    }
}
trait ELJTextDropTarget {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJTextDropTarget_Create(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnData(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnDragOver(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnDrop(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnEnter(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJTextDropTarget_SetOnLeave(_obj, _func)
        }
    }
}
trait wxAutoBufferedPaintDC {
    #[fixed_stack_segment]
    fn Create(window: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxAutoBufferedPaintDC_Create(window)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxAutoBufferedPaintDC_Delete(self_)
        }
    }
}
trait wxFontData {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxFontData_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFontData_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EnableEffects(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxFontData_EnableEffects(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn GetAllowSymbols(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontData_GetAllowSymbols(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetChosenFont(_obj: *u8 /* void* */, ref_: *u8 /* void* */) {
        unsafe {
            wxFontData_GetChosenFont(_obj, ref_)
        }
    }
    #[fixed_stack_segment]
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxFontData_GetColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetEnableEffects(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontData_GetEnableEffects(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEncoding(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFontData_GetEncoding(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetInitialFont(_obj: *u8 /* void* */, ref_: *u8 /* void* */) {
        unsafe {
            wxFontData_GetInitialFont(_obj, ref_)
        }
    }
    #[fixed_stack_segment]
    fn GetShowHelp(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFontData_GetShowHelp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetAllowSymbols(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxFontData_SetAllowSymbols(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn SetChosenFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxFontData_SetChosenFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn SetColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxFontData_SetColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */) {
        unsafe {
            wxFontData_SetEncoding(_obj, encoding)
        }
    }
    #[fixed_stack_segment]
    fn SetInitialFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxFontData_SetInitialFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn SetRange(_obj: *u8 /* void* */, minRange: c_int /* int */, maxRange: c_int /* int */) {
        unsafe {
            wxFontData_SetRange(_obj, minRange, maxRange)
        }
    }
    #[fixed_stack_segment]
    fn SetShowHelp(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxFontData_SetShowHelp(_obj, flag)
        }
    }
}
trait wxButton {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxButton_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxButton_SetBackgroundColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetDefault(_obj: *u8 /* void* */) {
        unsafe {
            wxButton_SetDefault(_obj)
        }
    }
}
trait wxGauge {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _rng: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGauge_Create(_prt, _id, _rng, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetBezelFace(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGauge_GetBezelFace(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRange(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGauge_GetRange(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetShadowWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGauge_GetShadowWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGauge_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetBezelFace(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxGauge_SetBezelFace(_obj, w)
        }
    }
    #[fixed_stack_segment]
    fn SetRange(_obj: *u8 /* void* */, r: c_int /* int */) {
        unsafe {
            wxGauge_SetRange(_obj, r)
        }
    }
    #[fixed_stack_segment]
    fn SetShadowWidth(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxGauge_SetShadowWidth(_obj, w)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxGauge_SetValue(_obj, pos)
        }
    }
}
trait wxBrush {
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, brush: *u8 /* void* */) {
        unsafe {
            wxBrush_Assign(_obj, brush)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxBrush_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBrush_CreateFromBitmap(bitmap)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromColour(col: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBrush_CreateFromColour(col, style)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBrush_CreateFromStock(id)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxBrush_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBrush_GetColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetStipple(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxBrush_GetStipple(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBrush_GetStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsEqual(_obj: *u8 /* void* */, brush: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBrush_IsEqual(_obj, brush)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBrush_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxBrush_SetColour(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetColourSingle(_obj: *u8 /* void* */, r: wchar_t /* wchar_t */, g: wchar_t /* wchar_t */, b: wchar_t /* wchar_t */) {
        unsafe {
            wxBrush_SetColourSingle(_obj, r, g, b)
        }
    }
    #[fixed_stack_segment]
    fn SetStipple(_obj: *u8 /* void* */, stipple: *u8 /* void* */) {
        unsafe {
            wxBrush_SetStipple(_obj, stipple)
        }
    }
    #[fixed_stack_segment]
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxBrush_SetStyle(_obj, style)
        }
    }
}
trait wxTreeCtrl {
    #[fixed_stack_segment]
    fn AddRoot(_obj: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_AddRoot(_obj, text, image, selectedImage, data, _item)
        }
    }
    #[fixed_stack_segment]
    fn AppendItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_AppendItem(_obj, parent, text, image, selectedImage, data, _item)
        }
    }
    #[fixed_stack_segment]
    fn Collapse(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Collapse(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn CollapseAndReset(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_CollapseAndReset(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _cmp: *u8 /* void* */, _prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_Create(_obj, _cmp, _prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Delete(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn DeleteAllItems(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_DeleteAllItems(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DeleteChildren(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_DeleteChildren(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn EditLabel(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_EditLabel(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn EndEditLabel(_obj: *u8 /* void* */, item: *u8 /* void* */, discardChanges: bool /* bool */) {
        unsafe {
            wxTreeCtrl_EndEditLabel(_obj, item, discardChanges)
        }
    }
    #[fixed_stack_segment]
    fn EnsureVisible(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_EnsureVisible(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn Expand(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Expand(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn GetBoundingRect(_obj: *u8 /* void* */, item: *u8 /* void* */, textOnly: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetBoundingRect(_obj, item, textOnly)
        }
    }
    #[fixed_stack_segment]
    fn GetChildrenCount(_obj: *u8 /* void* */, item: *u8 /* void* */, recursively: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetChildrenCount(_obj, item, recursively)
        }
    }
    #[fixed_stack_segment]
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEditControl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetEditControl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFirstChild(_obj: *u8 /* void* */, item: *u8 /* void* */, cookie: *c_int /* int* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetFirstChild(_obj, item, cookie, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetFirstVisibleItem(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetFirstVisibleItem(_obj, item, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetImageList(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetImageList(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetIndent(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetIndent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetItemData(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetItemData(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn GetItemImage(_obj: *u8 /* void* */, item: *u8 /* void* */, which: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetItemImage(_obj, item, which)
        }
    }
    #[fixed_stack_segment]
    fn GetItemText(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetItemText(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn GetLastChild(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetLastChild(_obj, item, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetNextChild(_obj: *u8 /* void* */, item: *u8 /* void* */, cookie: *c_int /* int* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetNextChild(_obj, item, cookie, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetNextSibling(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetNextSibling(_obj, item, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetNextVisible(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetNextVisible(_obj, item, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetParent(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetParent(_obj, item, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetPrevSibling(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetPrevSibling(_obj, item, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetPrevVisible(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetPrevVisible(_obj, item, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetRootItem(_obj: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetRootItem(_obj, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetSelection(_obj: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_GetSelection(_obj, _item)
        }
    }
    #[fixed_stack_segment]
    fn GetSelections(_obj: *u8 /* void* */, selections: *intptr_t /* intptr_t* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetSelections(_obj, selections)
        }
    }
    #[fixed_stack_segment]
    fn GetSpacing(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_GetSpacing(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStateImageList(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetStateImageList(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *c_int /* int* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_HitTest(_obj, arg0, arg1, flags, _item)
        }
    }
    #[fixed_stack_segment]
    fn InsertItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, idPrevious: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_InsertItem(_obj, parent, idPrevious, text, image, selectedImage, data, _item)
        }
    }
    #[fixed_stack_segment]
    fn InsertItemByIndex(_obj: *u8 /* void* */, parent: *u8 /* void* */, index: c_int /* int */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_InsertItemByIndex(_obj, parent, index, text, image, selectedImage, data, _item)
        }
    }
    #[fixed_stack_segment]
    fn IsBold(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeCtrl_IsBold(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn IsExpanded(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeCtrl_IsExpanded(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn IsSelected(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeCtrl_IsSelected(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn IsVisible(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeCtrl_IsVisible(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn ItemHasChildren(_obj: *u8 /* void* */, item: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_ItemHasChildren(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn OnCompareItems(_obj: *u8 /* void* */, item1: *u8 /* void* */, item2: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeCtrl_OnCompareItems(_obj, item1, item2)
        }
    }
    #[fixed_stack_segment]
    fn PrependItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_PrependItem(_obj, parent, text, image, selectedImage, data, _item)
        }
    }
    #[fixed_stack_segment]
    fn ScrollTo(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_ScrollTo(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn SelectItem(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SelectItem(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetImageList(_obj, imageList)
        }
    }
    #[fixed_stack_segment]
    fn SetIndent(_obj: *u8 /* void* */, indent: c_int /* int */) {
        unsafe {
            wxTreeCtrl_SetIndent(_obj, indent)
        }
    }
    #[fixed_stack_segment]
    fn SetItemBackgroundColour(_obj: *u8 /* void* */, item: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemBackgroundColour(_obj, item, col)
        }
    }
    #[fixed_stack_segment]
    fn SetItemBold(_obj: *u8 /* void* */, item: *u8 /* void* */, bold: bool /* bool */) {
        unsafe {
            wxTreeCtrl_SetItemBold(_obj, item, bold)
        }
    }
    #[fixed_stack_segment]
    fn SetItemData(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemData(_obj, item, data)
        }
    }
    #[fixed_stack_segment]
    fn SetItemDropHighlight(_obj: *u8 /* void* */, item: *u8 /* void* */, highlight: bool /* bool */) {
        unsafe {
            wxTreeCtrl_SetItemDropHighlight(_obj, item, highlight)
        }
    }
    #[fixed_stack_segment]
    fn SetItemFont(_obj: *u8 /* void* */, item: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemFont(_obj, item, font)
        }
    }
    #[fixed_stack_segment]
    fn SetItemHasChildren(_obj: *u8 /* void* */, item: *u8 /* void* */, hasChildren: bool /* bool */) {
        unsafe {
            wxTreeCtrl_SetItemHasChildren(_obj, item, hasChildren)
        }
    }
    #[fixed_stack_segment]
    fn SetItemImage(_obj: *u8 /* void* */, item: *u8 /* void* */, image: c_int /* int */, which: c_int /* int */) {
        unsafe {
            wxTreeCtrl_SetItemImage(_obj, item, image, which)
        }
    }
    #[fixed_stack_segment]
    fn SetItemText(_obj: *u8 /* void* */, item: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemText(_obj, item, text)
        }
    }
    #[fixed_stack_segment]
    fn SetItemTextColour(_obj: *u8 /* void* */, item: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemTextColour(_obj, item, col)
        }
    }
    #[fixed_stack_segment]
    fn SetSpacing(_obj: *u8 /* void* */, spacing: c_int /* int */) {
        unsafe {
            wxTreeCtrl_SetSpacing(_obj, spacing)
        }
    }
    #[fixed_stack_segment]
    fn SetStateImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetStateImageList(_obj, imageList)
        }
    }
    #[fixed_stack_segment]
    fn SortChildren(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SortChildren(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn Toggle(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Toggle(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn Unselect(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_Unselect(_obj)
        }
    }
    #[fixed_stack_segment]
    fn UnselectAll(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_UnselectAll(_obj)
        }
    }
}
trait wxNotifyEvent {
    #[fixed_stack_segment]
    fn Allow(_obj: *u8 /* void* */) {
        unsafe {
            wxNotifyEvent_Allow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxNotifyEvent_CopyObject(_obj, object_dest)
        }
    }
    #[fixed_stack_segment]
    fn IsAllowed(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNotifyEvent_IsAllowed(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Veto(_obj: *u8 /* void* */) {
        unsafe {
            wxNotifyEvent_Veto(_obj)
        }
    }
}
trait wxQuantize {
}
trait cbMiniButton {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            cbMiniButton_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbMiniButton_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Dim(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbMiniButton_Dim(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn DragStarted(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbMiniButton_DragStarted(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            cbMiniButton_Enable(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn Enabled(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbMiniButton_Enabled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */ {
        unsafe {
            cbMiniButton_HitTest(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn IsPressed(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            cbMiniButton_IsPressed(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Layout(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbMiniButton_Layout(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Pane(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbMiniButton_Pane(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Plugin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbMiniButton_Plugin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbMiniButton_Pos(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Pressed(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbMiniButton_Pressed(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Refresh(_obj: *u8 /* void* */) {
        unsafe {
            cbMiniButton_Refresh(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Reset(_obj: *u8 /* void* */) {
        unsafe {
            cbMiniButton_Reset(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetPos(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            cbMiniButton_SetPos(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Visible(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbMiniButton_Visible(_obj)
        }
    }
    #[fixed_stack_segment]
    fn WasClicked(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbMiniButton_WasClicked(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Wnd(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbMiniButton_Wnd(_obj)
        }
    }
}
trait wxPopupWindow {
}
trait wxFileOutputStream {
}
trait wxFileConfig {
    #[fixed_stack_segment]
    fn wxConfigBase_Get() -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_Get()
        }
    }
    #[fixed_stack_segment]
    fn wxConfigBase_Set(self_: *u8 /* void* */) {
        unsafe {
            wxConfigBase_Set(self_)
        }
    }
    #[fixed_stack_segment]
    fn Create(inp: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileConfig_Create(inp)
        }
    }
    #[fixed_stack_segment]
    fn wxBitmap_CreateFromImage(image: *u8 /* void* */, depth: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBitmap_CreateFromImage(image, depth)
        }
    }
    #[fixed_stack_segment]
    fn wxImage_CreateFromDataEx(arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */, isStaticData: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromDataEx(arg0, arg1, data, isStaticData)
        }
    }
    #[fixed_stack_segment]
    fn wxImage_Delete(image: *u8 /* void* */) {
        unsafe {
            wxImage_Delete(image)
        }
    }
    #[fixed_stack_segment]
    fn wxColour_CreateFromInt(rgb: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateFromInt(rgb)
        }
    }
    #[fixed_stack_segment]
    fn wxColour_GetInt(colour: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxColour_GetInt(colour)
        }
    }
    #[fixed_stack_segment]
    fn wxColour_CreateFromUnsignedInt(rgba: uint32_t /* uint32_t */) -> *u8 /* void* */ {
        unsafe {
            wxColour_CreateFromUnsignedInt(rgba)
        }
    }
    #[fixed_stack_segment]
    fn wxColour_GetUnsignedInt(colour: *u8 /* void* */) -> uint32_t /* uint32_t */ {
        unsafe {
            wxColour_GetUnsignedInt(colour)
        }
    }
    #[fixed_stack_segment]
    fn wxcSystemSettingsGetColour(systemColour: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxcSystemSettingsGetColour(systemColour)
        }
    }
    #[fixed_stack_segment]
    fn wxcSetPixelRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgb: c_int /* int */) {
        unsafe {
            wxcSetPixelRGB(buffer, width, arg0, arg1, rgb)
        }
    }
    #[fixed_stack_segment]
    fn wxcGetPixelRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxcGetPixelRGB(buffer, width, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxcSetPixelRowRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgbStart: c_int /* int */, rgbEnd: c_int /* int */, count: c_int /* int */) {
        unsafe {
            wxcSetPixelRowRGB(buffer, width, arg0, arg1, rgbStart, rgbEnd, count)
        }
    }
    #[fixed_stack_segment]
    fn wxcInitPixelsRGB(buffer: *uint8_t /* uint8_t* */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: c_int /* int */) {
        unsafe {
            wxcInitPixelsRGB(buffer, arg0, arg1, rgba)
        }
    }
    #[fixed_stack_segment]
    fn wxcSetPixelRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: uint32_t /* uint32_t */) {
        unsafe {
            wxcSetPixelRGBA(buffer, width, arg0, arg1, rgba)
        }
    }
    #[fixed_stack_segment]
    fn wxcGetPixelRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> uint32_t /* uint32_t */ {
        unsafe {
            wxcGetPixelRGBA(buffer, width, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxcSetPixelRowRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgbaStart: c_int /* int */, rgbEnd: c_int /* int */, count: uint32_t /* uint32_t */) {
        unsafe {
            wxcSetPixelRowRGBA(buffer, width, arg0, arg1, rgbaStart, rgbEnd, count)
        }
    }
    #[fixed_stack_segment]
    fn wxcInitPixelsRGBA(buffer: *uint8_t /* uint8_t* */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: uint32_t /* uint32_t */) {
        unsafe {
            wxcInitPixelsRGBA(buffer, arg0, arg1, rgba)
        }
    }
    #[fixed_stack_segment]
    fn wxcMalloc(size: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxcMalloc(size)
        }
    }
    #[fixed_stack_segment]
    fn wxcFree(p: *u8 /* void* */) {
        unsafe {
            wxcFree(p)
        }
    }
    #[fixed_stack_segment]
    fn wxcWakeUpIdle() {
        unsafe {
            wxcWakeUpIdle()
        }
    }
    #[fixed_stack_segment]
    fn wxGetApplicationDir() -> *u8 /* void* */ {
        unsafe {
            wxGetApplicationDir()
        }
    }
    #[fixed_stack_segment]
    fn wxGetApplicationPath() -> *u8 /* void* */ {
        unsafe {
            wxGetApplicationPath()
        }
    }
    #[fixed_stack_segment]
    fn ELJApp_InitializeC(closure: *u8 /* void* */, _argc: c_int /* int */, _argv: *wchar_t /* wchar_t* */) {
        unsafe {
            ELJApp_InitializeC(closure, _argc, _argv)
        }
    }
    #[fixed_stack_segment]
    fn ELJApp_GetIdleInterval() -> c_int /* int */ {
        unsafe {
            ELJApp_GetIdleInterval()
        }
    }
    #[fixed_stack_segment]
    fn ELJApp_SetIdleInterval(interval: c_int /* int */) {
        unsafe {
            ELJApp_SetIdleInterval(interval)
        }
    }
}
trait wxExprDatabase {
}
trait wxHelpEvent {
    #[fixed_stack_segment]
    fn GetLink(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpEvent_GetLink(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTarget(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHelpEvent_GetTarget(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetLink(_obj: *u8 /* void* */, link: *u8 /* void* */) {
        unsafe {
            wxHelpEvent_SetLink(_obj, link)
        }
    }
    #[fixed_stack_segment]
    fn SetPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxHelpEvent_SetPosition(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetTarget(_obj: *u8 /* void* */, target: *u8 /* void* */) {
        unsafe {
            wxHelpEvent_SetTarget(_obj, target)
        }
    }
}
trait cbRightUpEvent {
    #[fixed_stack_segment]
    fn Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbRightUpEvent_Pos(_obj, arg0, arg1)
        }
    }
}
trait wxToolBar {
    #[fixed_stack_segment]
    fn AddControl(_obj: *u8 /* void* */, ctrl: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxToolBar_AddControl(_obj, ctrl)
        }
    }
    #[fixed_stack_segment]
    fn AddSeparator(_obj: *u8 /* void* */) {
        unsafe {
            wxToolBar_AddSeparator(_obj)
        }
    }
    #[fixed_stack_segment]
    fn AddTool(_obj: *u8 /* void* */, id: c_int /* int */, bmp: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */) {
        unsafe {
            wxToolBar_AddTool(_obj, id, bmp, shelp, lhelp)
        }
    }
    #[fixed_stack_segment]
    fn AddToolEx(_obj: *u8 /* void* */, id: c_int /* int */, bmp1: *u8 /* void* */, bmp2: *u8 /* void* */, isToggle: bool /* bool */, arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */) {
        unsafe {
            wxToolBar_AddToolEx(_obj, id, bmp1, bmp2, isToggle, arg0, arg1, data, shelp, lhelp)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxToolBar_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DeleteTool(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxToolBar_DeleteTool(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn DeleteToolByPos(_obj: *u8 /* void* */, pos: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxToolBar_DeleteToolByPos(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn EnableTool(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxToolBar_EnableTool(_obj, id, enable)
        }
    }
    #[fixed_stack_segment]
    fn GetMargins(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetMargins(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetToolBitmapSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolBitmapSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetToolClientData(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolClientData(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn GetToolEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxToolBar_GetToolEnabled(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn GetToolLongHelp(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolLongHelp(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn GetToolPacking(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxToolBar_GetToolPacking(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetToolShortHelp(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolShortHelp(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn GetToolSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxToolBar_GetToolSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetToolState(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxToolBar_GetToolState(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn InsertControl(_obj: *u8 /* void* */, pos: c_int /* int */, ctrl: *u8 /* void* */) {
        unsafe {
            wxToolBar_InsertControl(_obj, pos, ctrl)
        }
    }
    #[fixed_stack_segment]
    fn InsertSeparator(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxToolBar_InsertSeparator(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn InsertTool(_obj: *u8 /* void* */, pos: c_int /* int */, id: c_int /* int */, bmp1: *u8 /* void* */, bmp2: *u8 /* void* */, isToggle: bool /* bool */, data: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */) {
        unsafe {
            wxToolBar_InsertTool(_obj, pos, id, bmp1, bmp2, isToggle, data, shelp, lhelp)
        }
    }
    #[fixed_stack_segment]
    fn Realize(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxToolBar_Realize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RemoveTool(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxToolBar_RemoveTool(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn SetMargins(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxToolBar_SetMargins(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetToolBitmapSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxToolBar_SetToolBitmapSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetToolClientData(_obj: *u8 /* void* */, id: c_int /* int */, data: *u8 /* void* */) {
        unsafe {
            wxToolBar_SetToolClientData(_obj, id, data)
        }
    }
    #[fixed_stack_segment]
    fn SetToolLongHelp(_obj: *u8 /* void* */, id: c_int /* int */, str: *u8 /* void* */) {
        unsafe {
            wxToolBar_SetToolLongHelp(_obj, id, str)
        }
    }
    #[fixed_stack_segment]
    fn SetToolPacking(_obj: *u8 /* void* */, packing: c_int /* int */) {
        unsafe {
            wxToolBar_SetToolPacking(_obj, packing)
        }
    }
    #[fixed_stack_segment]
    fn SetToolSeparation(_obj: *u8 /* void* */, separation: c_int /* int */) {
        unsafe {
            wxToolBar_SetToolSeparation(_obj, separation)
        }
    }
    #[fixed_stack_segment]
    fn SetToolShortHelp(_obj: *u8 /* void* */, id: c_int /* int */, str: *u8 /* void* */) {
        unsafe {
            wxToolBar_SetToolShortHelp(_obj, id, str)
        }
    }
    #[fixed_stack_segment]
    fn ToggleTool(_obj: *u8 /* void* */, id: c_int /* int */, toggle: bool /* bool */) {
        unsafe {
            wxToolBar_ToggleTool(_obj, id, toggle)
        }
    }
}
trait wxIdleEvent {
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxIdleEvent_CopyObject(_obj, object_dest)
        }
    }
    #[fixed_stack_segment]
    fn MoreRequested(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIdleEvent_MoreRequested(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: c_float /* float */) -> *u8 /* void* */ {
        unsafe {
            wxFloatProperty_Create(label, name, value)
        }
    }
}
trait wxContextHelp {
    #[fixed_stack_segment]
    fn BeginContextHelp(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxContextHelp_BeginContextHelp(_obj, win)
        }
    }
    #[fixed_stack_segment]
    fn Create(win: *u8 /* void* */, beginHelp: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxContextHelp_Create(win, beginHelp)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxContextHelp_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EndContextHelp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxContextHelp_EndContextHelp(_obj)
        }
    }
}
trait wxPoint {
    #[fixed_stack_segment]
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPoint_Create(arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Destroy(_obj: *u8 /* void* */) {
        unsafe {
            wxPoint_Destroy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPoint_GetX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPoint_GetY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetX(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxPoint_SetX(_obj, w)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */, _min: c_int /* int */, _max: c_int /* int */, _init: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSpinCtrl_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl, _min, _max, _init)
        }
    }
    #[fixed_stack_segment]
    fn GetMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinCtrl_GetMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinCtrl_GetMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinCtrl_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetRange(_obj: *u8 /* void* */, min_val: c_int /* int */, max_val: c_int /* int */) {
        unsafe {
            wxSpinCtrl_SetRange(_obj, min_val, max_val)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxSpinCtrl_SetValue(_obj, val)
        }
    }
}
trait wxGenericDragImage {
    #[fixed_stack_segment]
    fn wxDragImage_Create(image: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDragImage_Create(image, x, y)
        }
    }
    #[fixed_stack_segment]
    fn wxDragIcon(icon: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDragIcon(icon, x, y)
        }
    }
    #[fixed_stack_segment]
    fn wxDragString(test: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxDragString(test, x, y)
        }
    }
    #[fixed_stack_segment]
    fn wxDragTreeItem(treeCtrl: *u8 /* void* */, id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDragTreeItem(treeCtrl, id)
        }
    }
    #[fixed_stack_segment]
    fn wxDragListItem(treeCtrl: *u8 /* void* */, id: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxDragListItem(treeCtrl, id)
        }
    }
    #[fixed_stack_segment]
    fn Create(cursor: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGenericDragImage_Create(cursor)
        }
    }
    #[fixed_stack_segment]
    fn wxGenericDragIcon(icon: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGenericDragIcon(icon)
        }
    }
    #[fixed_stack_segment]
    fn wxGenericDragString(test: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGenericDragString(test)
        }
    }
    #[fixed_stack_segment]
    fn wxGenericDragTreeItem(treeCtrl: *u8 /* void* */, id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGenericDragTreeItem(treeCtrl, id)
        }
    }
    #[fixed_stack_segment]
    fn wxGenericDragListItem(treeCtrl: *u8 /* void* */, id: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxGenericDragListItem(treeCtrl, id)
        }
    }
    #[fixed_stack_segment]
    fn wxDragImage_Delete(self_: *u8 /* void* */) {
        unsafe {
            wxDragImage_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxDragImage_BeginDragFullScreen(self_: *u8 /* void* */, x_pos: c_int /* int */, y_pos: c_int /* int */, window: *u8 /* void* */, fullScreen: bool /* bool */, rect: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDragImage_BeginDragFullScreen(self_, x_pos, y_pos, window, fullScreen, rect)
        }
    }
    #[fixed_stack_segment]
    fn wxDragImage_BeginDrag(self_: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */, window: *u8 /* void* */, boundingWindow: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDragImage_BeginDrag(self_, x, y, window, boundingWindow)
        }
    }
    #[fixed_stack_segment]
    fn DoDrawImage(self_: *u8 /* void* */, dc: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxGenericDragImage_DoDrawImage(self_, dc, x, y)
        }
    }
    #[fixed_stack_segment]
    fn wxDragImage_EndDrag(self_: *u8 /* void* */) {
        unsafe {
            wxDragImage_EndDrag(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetImageRect(self_: *u8 /* void* */, x_pos: c_int /* int */, y_pos: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGenericDragImage_GetImageRect(self_, x_pos, y_pos)
        }
    }
    #[fixed_stack_segment]
    fn wxDragImage_Hide(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDragImage_Hide(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxDragImage_Move(self_: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxDragImage_Move(self_, x, y)
        }
    }
    #[fixed_stack_segment]
    fn wxDragImage_Show(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxDragImage_Show(self_)
        }
    }
    #[fixed_stack_segment]
    fn UpdateBackingFromWindow(self_: *u8 /* void* */, windowDC: *u8 /* void* */, destDC: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */, w: c_int /* int */, h: c_int /* int */, xdest: c_int /* int */, ydest: c_int /* int */, width: c_int /* int */, height: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxGenericDragImage_UpdateBackingFromWindow(self_, windowDC, destDC, x, y, w, h, xdest, ydest, width, height)
        }
    }
}
trait wxScrollEvent {
    #[fixed_stack_segment]
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollEvent_GetOrientation(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollEvent_GetPosition(_obj)
        }
    }
}
trait wxPrintPreview {
    #[fixed_stack_segment]
    fn CreateFromData(printout: *u8 /* void* */, printoutForPrinting: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_CreateFromData(printout, printoutForPrinting, data)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromDialogData(printout: *u8 /* void* */, printoutForPrinting: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_CreateFromDialogData(printout, printoutForPrinting, data)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DetermineScaling(_obj: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_DetermineScaling(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DrawBlankPage(_obj: *u8 /* void* */, canvas: *u8 /* void* */, dc: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_DrawBlankPage(_obj, canvas, dc)
        }
    }
    #[fixed_stack_segment]
    fn GetCanvas(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_GetCanvas(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCurrentPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintPreview_GetCurrentPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_GetFrame(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaxPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintPreview_GetMaxPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintPreview_GetMinPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintDialogData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_GetPrintDialogData(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintout(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_GetPrintout(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintoutForPrinting(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintPreview_GetPrintoutForPrinting(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetZoom(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintPreview_GetZoom(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PaintPage(_obj: *u8 /* void* */, canvas: *u8 /* void* */, dc: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_PaintPage(_obj, canvas, dc)
        }
    }
    #[fixed_stack_segment]
    fn Print(_obj: *u8 /* void* */, interactive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_Print(_obj, interactive)
        }
    }
    #[fixed_stack_segment]
    fn RenderPage(_obj: *u8 /* void* */, pageNum: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_RenderPage(_obj, pageNum)
        }
    }
    #[fixed_stack_segment]
    fn SetCanvas(_obj: *u8 /* void* */, canvas: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_SetCanvas(_obj, canvas)
        }
    }
    #[fixed_stack_segment]
    fn SetCurrentPage(_obj: *u8 /* void* */, pageNum: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxPrintPreview_SetCurrentPage(_obj, pageNum)
        }
    }
    #[fixed_stack_segment]
    fn SetFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_SetFrame(_obj, frame)
        }
    }
    #[fixed_stack_segment]
    fn SetOk(_obj: *u8 /* void* */, ok: bool /* bool */) {
        unsafe {
            wxPrintPreview_SetOk(_obj, ok)
        }
    }
    #[fixed_stack_segment]
    fn SetPrintout(_obj: *u8 /* void* */, printout: *u8 /* void* */) {
        unsafe {
            wxPrintPreview_SetPrintout(_obj, printout)
        }
    }
    #[fixed_stack_segment]
    fn SetZoom(_obj: *u8 /* void* */, percent: c_int /* int */) {
        unsafe {
            wxPrintPreview_SetZoom(_obj, percent)
        }
    }
}
trait wxPalette {
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, palette: *u8 /* void* */) {
        unsafe {
            wxPalette_Assign(_obj, palette)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxPalette_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateRGB(n: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPalette_CreateRGB(n, red, green, blue)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPalette_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPixel(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) -> c_int /* int */ {
        unsafe {
            wxPalette_GetPixel(_obj, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn GetRGB(_obj: *u8 /* void* */, pixel: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPalette_GetRGB(_obj, pixel, red, green, blue)
        }
    }
    #[fixed_stack_segment]
    fn IsEqual(_obj: *u8 /* void* */, palette: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPalette_IsEqual(_obj, palette)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPalette_IsOk(_obj)
        }
    }
}
trait wxGridRangeSelectEvent {
    #[fixed_stack_segment]
    fn GetTopLeftCoords(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGridRangeSelectEvent_GetTopLeftCoords(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetBottomRightCoords(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGridRangeSelectEvent_GetBottomRightCoords(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetTopRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridRangeSelectEvent_GetTopRow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBottomRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridRangeSelectEvent_GetBottomRow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLeftCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridRangeSelectEvent_GetLeftCol(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRightCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridRangeSelectEvent_GetRightCol(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Selecting(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridRangeSelectEvent_Selecting(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridRangeSelectEvent_ControlDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridRangeSelectEvent_MetaDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridRangeSelectEvent_ShiftDown(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(_file: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMetafile_Create(_file)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMetafile_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMetafile_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Play(_obj: *u8 /* void* */, _dc: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMetafile_Play(_obj, _dc)
        }
    }
    #[fixed_stack_segment]
    fn SetClipboard(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMetafile_SetClipboard(_obj, arg0, arg1)
        }
    }
}
trait wxListItem {
    #[fixed_stack_segment]
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxListItem_Clear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ClearAttributes(_obj: *u8 /* void* */) {
        unsafe {
            wxListItem_ClearAttributes(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxListItem_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxListItem_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetAlign(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetAlign(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetAttributes(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListItem_GetAttributes(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListItem_GetBackgroundColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetColumn(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetColumn(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetData(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetData(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListItem_GetFont(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetImage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetImage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetMask(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetState(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetState(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxListItem_GetText(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxListItem_GetTextColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListItem_GetWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasAttributes(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListItem_HasAttributes(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetAlign(_obj: *u8 /* void* */, align: c_int /* int */) {
        unsafe {
            wxListItem_SetAlign(_obj, align)
        }
    }
    #[fixed_stack_segment]
    fn SetBackgroundColour(_obj: *u8 /* void* */, colBack: *u8 /* void* */) {
        unsafe {
            wxListItem_SetBackgroundColour(_obj, colBack)
        }
    }
    #[fixed_stack_segment]
    fn SetColumn(_obj: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxListItem_SetColumn(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetData(_obj: *u8 /* void* */, data: c_int /* int */) {
        unsafe {
            wxListItem_SetData(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn SetDataPointer(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxListItem_SetDataPointer(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxListItem_SetFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn SetId(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxListItem_SetId(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn SetImage(_obj: *u8 /* void* */, image: c_int /* int */) {
        unsafe {
            wxListItem_SetImage(_obj, image)
        }
    }
    #[fixed_stack_segment]
    fn SetMask(_obj: *u8 /* void* */, mask: c_int /* int */) {
        unsafe {
            wxListItem_SetMask(_obj, mask)
        }
    }
    #[fixed_stack_segment]
    fn SetState(_obj: *u8 /* void* */, state: c_int /* int */) {
        unsafe {
            wxListItem_SetState(_obj, state)
        }
    }
    #[fixed_stack_segment]
    fn SetStateMask(_obj: *u8 /* void* */, stateMask: c_int /* int */) {
        unsafe {
            wxListItem_SetStateMask(_obj, stateMask)
        }
    }
    #[fixed_stack_segment]
    fn SetText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxListItem_SetText(_obj, text)
        }
    }
    #[fixed_stack_segment]
    fn SetTextColour(_obj: *u8 /* void* */, colText: *u8 /* void* */) {
        unsafe {
            wxListItem_SetTextColour(_obj, colText)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn CanRedo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCommandProcessor_CanRedo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CanUndo(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCommandProcessor_CanUndo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ClearCommands(_obj: *u8 /* void* */) {
        unsafe {
            wxCommandProcessor_ClearCommands(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCommandProcessor_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCommands(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCommandProcessor_GetCommands(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetEditMenu(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCommandProcessor_GetEditMenu(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaxCommands(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCommandProcessor_GetMaxCommands(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Initialize(_obj: *u8 /* void* */) {
        unsafe {
            wxCommandProcessor_Initialize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Redo(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCommandProcessor_Redo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetEditMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxCommandProcessor_SetEditMenu(_obj, menu)
        }
    }
    #[fixed_stack_segment]
    fn SetMenuStrings(_obj: *u8 /* void* */) {
        unsafe {
            wxCommandProcessor_SetMenuStrings(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Submit(_obj: *u8 /* void* */, command: *u8 /* void* */, storeIt: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxCommandProcessor_Submit(_obj, command, storeIt)
        }
    }
    #[fixed_stack_segment]
    fn Undo(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCommandProcessor_Undo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxCommandProcessor(maxCommands: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCommandProcessor_wxCommandProcessor(maxCommands)
        }
    }
}
trait wxInitDialogEvent {
}
trait wxTreeItemId {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxTreeItemId_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeItemId_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxTreeItemId_IsOk(_obj)
        }
    }
}
trait wxSashLayoutWindow {
    #[fixed_stack_segment]
    fn Create(_par: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSashLayoutWindow_Create(_par, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashLayoutWindow_GetAlignment(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashLayoutWindow_GetOrientation(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetAlignment(_obj: *u8 /* void* */, align: c_int /* int */) {
        unsafe {
            wxSashLayoutWindow_SetAlignment(_obj, align)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSashLayoutWindow_SetDefaultSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */) {
        unsafe {
            wxSashLayoutWindow_SetOrientation(_obj, orient)
        }
    }
}
trait cbHintAnimationPlugin {
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            cbHintAnimationPlugin_Create(pPanel, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbHintAnimationPlugin_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbHintAnimationPlugin_Delete(_obj)
        }
    }
}
trait wxPrinterDC {
    #[fixed_stack_segment]
    fn Create(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinterDC_Create(data)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxPrinterDC_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetPaperRect(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinterDC_GetPaperRect(self_)
        }
    }
}
trait wxWindowDestroyEvent {
    #[fixed_stack_segment]
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindowDestroyEvent_GetWindow(_obj)
        }
    }
}
trait wxIcon {
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, other: *u8 /* void* */) {
        unsafe {
            wxIcon_Assign(_obj, other)
        }
    }
    #[fixed_stack_segment]
    fn CopyFromBitmap(_obj: *u8 /* void* */, bmp: *u8 /* void* */) {
        unsafe {
            wxIcon_CopyFromBitmap(_obj, bmp)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxIcon_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateLoad(name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxIcon_CreateLoad(name, type_, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxIcon_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FromRaw(data: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxIcon_FromRaw(data, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn FromXPM(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxIcon_FromXPM(data)
        }
    }
    #[fixed_stack_segment]
    fn GetDepth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIcon_GetDepth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIcon_GetHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxIcon_GetWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsEqual(_obj: *u8 /* void* */, other: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIcon_IsEqual(_obj, other)
        }
    }
    #[fixed_stack_segment]
    fn Load(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxIcon_Load(_obj, name, type_, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIcon_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetDepth(_obj: *u8 /* void* */, depth: c_int /* int */) {
        unsafe {
            wxIcon_SetDepth(_obj, depth)
        }
    }
    #[fixed_stack_segment]
    fn SetHeight(_obj: *u8 /* void* */, height: c_int /* int */) {
        unsafe {
            wxIcon_SetHeight(_obj, height)
        }
    }
    #[fixed_stack_segment]
    fn SetWidth(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxIcon_SetWidth(_obj, width)
        }
    }
}
trait cbPluginBase {
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbPluginBase_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPaneMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbPluginBase_GetPaneMask(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsReady(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            cbPluginBase_IsReady(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Plugin(_swt: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            cbPluginBase_Plugin(_swt)
        }
    }
    #[fixed_stack_segment]
    fn ProcessEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbPluginBase_ProcessEvent(_obj, event)
        }
    }
}
trait wxBufferedPaintDC {
    #[fixed_stack_segment]
    fn Create(window: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBufferedPaintDC_Create(window, style)
        }
    }
    #[fixed_stack_segment]
    fn CreateWithBitmap(window: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBufferedPaintDC_CreateWithBitmap(window, bitmap, style)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxBufferedPaintDC_Delete(self_)
        }
    }
}
trait ELJPlotCurve {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _str: *u8 /* void* */, _end: *u8 /* void* */, _y: *u8 /* void* */, offsetY: c_int /* int */, startY: c_double /* double */, endY: c_double /* double */) -> *u8 /* void* */ {
        unsafe {
            ELJPlotCurve_Create(_obj, _str, _end, _y, offsetY, startY, endY)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJPlotCurve_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEndY(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            ELJPlotCurve_GetEndY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOffsetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJPlotCurve_GetOffsetY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStartY(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            ELJPlotCurve_GetStartY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetEndY(_obj: *u8 /* void* */, endY: c_double /* double */) {
        unsafe {
            ELJPlotCurve_SetEndY(_obj, endY)
        }
    }
    #[fixed_stack_segment]
    fn SetOffsetY(_obj: *u8 /* void* */, offsetY: c_int /* int */) {
        unsafe {
            ELJPlotCurve_SetOffsetY(_obj, offsetY)
        }
    }
    #[fixed_stack_segment]
    fn SetPenNormal(_obj: *u8 /* void* */, pen: *u8 /* void* */) {
        unsafe {
            ELJPlotCurve_SetPenNormal(_obj, pen)
        }
    }
    #[fixed_stack_segment]
    fn SetPenSelected(_obj: *u8 /* void* */, pen: *u8 /* void* */) {
        unsafe {
            ELJPlotCurve_SetPenSelected(_obj, pen)
        }
    }
    #[fixed_stack_segment]
    fn SetStartY(_obj: *u8 /* void* */, startY: c_double /* double */) {
        unsafe {
            ELJPlotCurve_SetStartY(_obj, startY)
        }
    }
}
trait wxGraphicsBrush {
}
trait wxToolTip {
}
trait wxcHtmlWindow {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */, _txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcHtmlWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl, _txt)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */, _txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl, _txt)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_AppendToPage(_obj: *u8 /* void* */, source: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_AppendToPage(_obj, source)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_GetInternalRepresentation(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetInternalRepresentation(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_GetOpenedAnchor(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetOpenedAnchor(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_GetOpenedPage(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetOpenedPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_GetOpenedPageTitle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetOpenedPageTitle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_GetRelatedFrame(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxHtmlWindow_GetRelatedFrame(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_HistoryBack(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_HistoryBack(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_HistoryCanBack(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_HistoryCanBack(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_HistoryCanForward(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_HistoryCanForward(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_HistoryClear(_obj: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_HistoryClear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_HistoryForward(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_HistoryForward(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_LoadPage(_obj: *u8 /* void* */, location: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxHtmlWindow_LoadPage(_obj, location)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_ReadCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_ReadCustomization(_obj, cfg, path)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_SetBorders(_obj: *u8 /* void* */, b: c_int /* int */) {
        unsafe {
            wxHtmlWindow_SetBorders(_obj, b)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_SetFonts(_obj: *u8 /* void* */, normal_face: *u8 /* void* */, fixed_face: *u8 /* void* */, sizes: *c_int /* int* */) {
        unsafe {
            wxHtmlWindow_SetFonts(_obj, normal_face, fixed_face, sizes)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_SetPage(_obj: *u8 /* void* */, source: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_SetPage(_obj, source)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_SetRelatedFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, format: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_SetRelatedFrame(_obj, frame, format)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_SetRelatedStatusBar(_obj: *u8 /* void* */, bar: c_int /* int */) {
        unsafe {
            wxHtmlWindow_SetRelatedStatusBar(_obj, bar)
        }
    }
    #[fixed_stack_segment]
    fn wxHtmlWindow_WriteCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxHtmlWindow_WriteCustomization(_obj, cfg, path)
        }
    }
}
trait wxValidator {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxValidator_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxValidator_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxValidator_GetWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetBellOnError(doIt: bool /* bool */) {
        unsafe {
            wxValidator_SetBellOnError(doIt)
        }
    }
    #[fixed_stack_segment]
    fn SetWindow(_obj: *u8 /* void* */, win: *u8 /* void* */) {
        unsafe {
            wxValidator_SetWindow(_obj, win)
        }
    }
    #[fixed_stack_segment]
    fn TransferFromWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxValidator_TransferFromWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn TransferToWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxValidator_TransferToWindow(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn GetData(_obj: *u8 /* void* */) {
        unsafe {
            wxDropTarget_GetData(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetDataObject(_obj: *u8 /* void* */, _dat: *u8 /* void* */) {
        unsafe {
            wxDropTarget_SetDataObject(_obj, _dat)
        }
    }
}
trait cbDockPane {
    #[fixed_stack_segment]
    fn BarPresent(_obj: *u8 /* void* */, pBar: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbDockPane_BarPresent(_obj, pBar)
        }
    }
    #[fixed_stack_segment]
    fn Create(alignment: c_int /* int */, pPanel: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDockPane_Create(alignment, pPanel)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbDockPane_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbDockPane_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbDockPane_GetAlignment(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBarInfoByWindow(_obj: *u8 /* void* */, pBarWnd: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDockPane_GetBarInfoByWindow(_obj, pBarWnd)
        }
    }
    #[fixed_stack_segment]
    fn GetBarResizeRange(_obj: *u8 /* void* */, pBar: *u8 /* void* */, from: *u8 /* void* */, till: *u8 /* void* */, forLeftHandle: c_int /* int */) {
        unsafe {
            cbDockPane_GetBarResizeRange(_obj, pBar, from, till, forLeftHandle)
        }
    }
    #[fixed_stack_segment]
    fn GetDockingState(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbDockPane_GetDockingState(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFirstRow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDockPane_GetFirstRow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPaneHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbDockPane_GetPaneHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRealRect(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            cbDockPane_GetRealRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn GetRowList(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbDockPane_GetRowList(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetRowResizeRange(_obj: *u8 /* void* */, pRow: *u8 /* void* */, from: *u8 /* void* */, till: *u8 /* void* */, forUpperHandle: c_int /* int */) {
        unsafe {
            cbDockPane_GetRowResizeRange(_obj, pRow, from, till, forUpperHandle)
        }
    }
    #[fixed_stack_segment]
    fn HitTestPaneItems(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, ppRow: *u8 /* void* */, ppBar: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            cbDockPane_HitTestPaneItems(_obj, arg0, arg1, ppRow, ppBar)
        }
    }
    #[fixed_stack_segment]
    fn InsertBarByCoord(_obj: *u8 /* void* */, pBar: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            cbDockPane_InsertBarByCoord(_obj, pBar, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn InsertBarByInfo(_obj: *u8 /* void* */, pBarInfo: *u8 /* void* */) {
        unsafe {
            cbDockPane_InsertBarByInfo(_obj, pBarInfo)
        }
    }
    #[fixed_stack_segment]
    fn InsertBarToRow(_obj: *u8 /* void* */, pBar: *u8 /* void* */, pIntoRow: *u8 /* void* */) {
        unsafe {
            cbDockPane_InsertBarToRow(_obj, pBar, pIntoRow)
        }
    }
    #[fixed_stack_segment]
    fn InsertRow(_obj: *u8 /* void* */, pRow: *u8 /* void* */, pBeforeRow: *u8 /* void* */) {
        unsafe {
            cbDockPane_InsertRow(_obj, pRow, pBeforeRow)
        }
    }
    #[fixed_stack_segment]
    fn IsHorizontal(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            cbDockPane_IsHorizontal(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MatchesMask(_obj: *u8 /* void* */, paneMask: c_int /* int */) -> c_int /* int */ {
        unsafe {
            cbDockPane_MatchesMask(_obj, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn RemoveBar(_obj: *u8 /* void* */, pBar: *u8 /* void* */) {
        unsafe {
            cbDockPane_RemoveBar(_obj, pBar)
        }
    }
    #[fixed_stack_segment]
    fn RemoveRow(_obj: *u8 /* void* */, pRow: *u8 /* void* */) {
        unsafe {
            cbDockPane_RemoveRow(_obj, pRow)
        }
    }
    #[fixed_stack_segment]
    fn SetBoundsInParent(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            cbDockPane_SetBoundsInParent(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn SetMargins(_obj: *u8 /* void* */, top: c_int /* int */, bottom: c_int /* int */, left: c_int /* int */, right: c_int /* int */) {
        unsafe {
            cbDockPane_SetMargins(_obj, top, bottom, left, right)
        }
    }
    #[fixed_stack_segment]
    fn SetPaneWidth(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            cbDockPane_SetPaneWidth(_obj, width)
        }
    }
}
trait wxToggleButton {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxToggleButton_Create(parent, id, label, arg0, arg1, arg2, arg3, style)
        }
    }
    #[fixed_stack_segment]
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxToggleButton_Enable(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxToggleButton_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetLabel(_obj: *u8 /* void* */, label: *u8 /* void* */) {
        unsafe {
            wxToggleButton_SetLabel(_obj, label)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, state: bool /* bool */) {
        unsafe {
            wxToggleButton_SetValue(_obj, state)
        }
    }
}
trait wxControl {
    #[fixed_stack_segment]
    fn Command(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxControl_Command(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxControl_GetLabel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetLabel(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxControl_SetLabel(_obj, text)
        }
    }
}
trait wxContextHelpButton {
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Append(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxComboBox_Append(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn AppendData(_obj: *u8 /* void* */, item: *u8 /* void* */, d: *u8 /* void* */) {
        unsafe {
            wxComboBox_AppendData(_obj, item, d)
        }
    }
    #[fixed_stack_segment]
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_Clear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Copy(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_Copy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, arg4, arg5, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Cut(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_Cut(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxComboBox_Delete(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_FindString(_obj, s)
        }
    }
    #[fixed_stack_segment]
    fn GetClientData(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_GetClientData(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_GetCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetInsertionPoint(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_GetInsertionPoint(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLastPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_GetLastPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxComboBox_GetSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_GetString(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_GetStringSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxComboBox_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Paste(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_Paste(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Remove(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */) {
        unsafe {
            wxComboBox_Remove(_obj, from, to)
        }
    }
    #[fixed_stack_segment]
    fn Replace(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */, value: *u8 /* void* */) {
        unsafe {
            wxComboBox_Replace(_obj, from, to, value)
        }
    }
    #[fixed_stack_segment]
    fn SetClientData(_obj: *u8 /* void* */, n: c_int /* int */, clientData: *u8 /* void* */) {
        unsafe {
            wxComboBox_SetClientData(_obj, n, clientData)
        }
    }
    #[fixed_stack_segment]
    fn SetEditable(_obj: *u8 /* void* */, editable: bool /* bool */) {
        unsafe {
            wxComboBox_SetEditable(_obj, editable)
        }
    }
    #[fixed_stack_segment]
    fn SetInsertionPoint(_obj: *u8 /* void* */, pos: c_int /* int */) {
        unsafe {
            wxComboBox_SetInsertionPoint(_obj, pos)
        }
    }
    #[fixed_stack_segment]
    fn SetInsertionPointEnd(_obj: *u8 /* void* */) {
        unsafe {
            wxComboBox_SetInsertionPointEnd(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetSelection(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxComboBox_SetSelection(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn SetTextSelection(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */) {
        unsafe {
            wxComboBox_SetTextSelection(_obj, from, to)
        }
    }
}
trait wxHtmlCell {
}
trait wxFont {
    #[fixed_stack_segment]
    fn Create(pointSize: c_int /* int */, family: c_int /* int */, style: c_int /* int */, weight: c_int /* int */, underlined: bool /* bool */, face: *u8 /* void* */, enc: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFont_Create(pointSize, family, style, weight, underlined, face, enc)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFont_CreateFromStock(id)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxFont_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFont_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultEncoding(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetDefaultEncoding(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEncoding(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetEncoding(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFaceName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFont_GetFaceName(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFamily(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetFamily(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFamilyString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFont_GetFamilyString(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPointSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetPointSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStyleString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFont_GetStyleString(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetUnderlined(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetUnderlined(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFont_GetWeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWeightString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFont_GetWeightString(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFont_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */) {
        unsafe {
            wxFont_SetDefaultEncoding(_obj, encoding)
        }
    }
    #[fixed_stack_segment]
    fn SetEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */) {
        unsafe {
            wxFont_SetEncoding(_obj, encoding)
        }
    }
    #[fixed_stack_segment]
    fn SetFaceName(_obj: *u8 /* void* */, faceName: *u8 /* void* */) {
        unsafe {
            wxFont_SetFaceName(_obj, faceName)
        }
    }
    #[fixed_stack_segment]
    fn SetFamily(_obj: *u8 /* void* */, family: c_int /* int */) {
        unsafe {
            wxFont_SetFamily(_obj, family)
        }
    }
    #[fixed_stack_segment]
    fn SetPointSize(_obj: *u8 /* void* */, pointSize: c_int /* int */) {
        unsafe {
            wxFont_SetPointSize(_obj, pointSize)
        }
    }
    #[fixed_stack_segment]
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxFont_SetStyle(_obj, style)
        }
    }
    #[fixed_stack_segment]
    fn SetUnderlined(_obj: *u8 /* void* */, underlined: c_int /* int */) {
        unsafe {
            wxFont_SetUnderlined(_obj, underlined)
        }
    }
    #[fixed_stack_segment]
    fn SetWeight(_obj: *u8 /* void* */, weight: c_int /* int */) {
        unsafe {
            wxFont_SetWeight(_obj, weight)
        }
    }
}
trait cbAntiflickerPlugin {
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            cbAntiflickerPlugin_Create(pPanel, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbAntiflickerPlugin_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbAntiflickerPlugin_Delete(_obj)
        }
    }
}
trait wxMemoryOutputStream {
}
trait wxNavigationKeyEvent {
    #[fixed_stack_segment]
    fn GetCurrentFocus(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxNavigationKeyEvent_GetCurrentFocus(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDirection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNavigationKeyEvent_GetDirection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsWindowChange(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxNavigationKeyEvent_IsWindowChange(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCurrentFocus(_obj: *u8 /* void* */, win: *u8 /* void* */) {
        unsafe {
            wxNavigationKeyEvent_SetCurrentFocus(_obj, win)
        }
    }
    #[fixed_stack_segment]
    fn SetDirection(_obj: *u8 /* void* */, bForward: bool /* bool */) {
        unsafe {
            wxNavigationKeyEvent_SetDirection(_obj, bForward)
        }
    }
    #[fixed_stack_segment]
    fn SetWindowChange(_obj: *u8 /* void* */, bIs: bool /* bool */) {
        unsafe {
            wxNavigationKeyEvent_SetWindowChange(_obj, bIs)
        }
    }
    #[fixed_stack_segment]
    fn ShouldPropagate(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxNavigationKeyEvent_ShouldPropagate(_obj)
        }
    }
}
trait wxDateProperty {
    #[fixed_stack_segment]
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxDateProperty_Create(label, name, value)
        }
    }
}
trait wxZlibInputStream {
}
trait cbPaneDrawPlugin {
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            cbPaneDrawPlugin_Create(pPanel, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbPaneDrawPlugin_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbPaneDrawPlugin_Delete(_obj)
        }
    }
}
trait wxProcessEvent {
    #[fixed_stack_segment]
    fn GetExitCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxProcessEvent_GetExitCode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPid(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxProcessEvent_GetPid(_obj)
        }
    }
}
trait wxPrinter {
    #[fixed_stack_segment]
    fn Create(data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinter_Create(data)
        }
    }
    #[fixed_stack_segment]
    fn CreateAbortWindow(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinter_CreateAbortWindow(_obj, parent, printout)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPrinter_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetAbort(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrinter_GetAbort(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLastError(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrinter_GetLastError(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintDialogData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPrinter_GetPrintDialogData(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn Print(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */, prompt: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxPrinter_Print(_obj, parent, printout, prompt)
        }
    }
    #[fixed_stack_segment]
    fn PrintDialog(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrinter_PrintDialog(_obj, parent)
        }
    }
    #[fixed_stack_segment]
    fn ReportError(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */, message: *u8 /* void* */) {
        unsafe {
            wxPrinter_ReportError(_obj, parent, printout, message)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Ctor(min: c_int /* int */, max: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellNumberEditor_Ctor(min, max)
        }
    }
}
trait wxFlexGridSizer {
    #[fixed_stack_segment]
    fn AddGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */) {
        unsafe {
            wxFlexGridSizer_AddGrowableCol(_obj, idx)
        }
    }
    #[fixed_stack_segment]
    fn AddGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */) {
        unsafe {
            wxFlexGridSizer_AddGrowableRow(_obj, idx)
        }
    }
    #[fixed_stack_segment]
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFlexGridSizer_CalcMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(rows: c_int /* int */, cols: c_int /* int */, vgap: c_int /* int */, hgap: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFlexGridSizer_Create(rows, cols, vgap, hgap)
        }
    }
    #[fixed_stack_segment]
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxFlexGridSizer_RecalcSizes(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RemoveGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */) {
        unsafe {
            wxFlexGridSizer_RemoveGrowableCol(_obj, idx)
        }
    }
    #[fixed_stack_segment]
    fn RemoveGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */) {
        unsafe {
            wxFlexGridSizer_RemoveGrowableRow(_obj, idx)
        }
    }
}
trait wxMouseCaptureChangedEvent {
}
trait wxGraphicsObject {
}
trait wxInputSinkEvent {
    #[fixed_stack_segment]
    fn LastError(obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputSinkEvent_LastError(obj)
        }
    }
    #[fixed_stack_segment]
    fn LastRead(obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxInputSinkEvent_LastRead(obj)
        }
    }
    #[fixed_stack_segment]
    fn LastInput(obj: *u8 /* void* */) -> *char /* char* */ {
        unsafe {
            wxInputSinkEvent_LastInput(obj)
        }
    }
}
trait wxPrintout {
}
trait wxXmlResource {
    #[fixed_stack_segment]
    fn AddHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxXmlResource_AddHandler(_obj, handler)
        }
    }
    #[fixed_stack_segment]
    fn AddSubclassFactory(_obj: *u8 /* void* */, factory: *u8 /* void* */) {
        unsafe {
            wxXmlResource_AddSubclassFactory(_obj, factory)
        }
    }
    #[fixed_stack_segment]
    fn AttachUnknownControl(_obj: *u8 /* void* */, control: *u8 /* void* */, parent: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxXmlResource_AttachUnknownControl(_obj, control, parent)
        }
    }
    #[fixed_stack_segment]
    fn ClearHandlers(_obj: *u8 /* void* */) {
        unsafe {
            wxXmlResource_ClearHandlers(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CompareVersion(_obj: *u8 /* void* */, major: c_int /* int */, minor: c_int /* int */, release: c_int /* int */, revision: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxXmlResource_CompareVersion(_obj, major, minor, release, revision)
        }
    }
    #[fixed_stack_segment]
    fn Create(flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_Create(flags)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromFile(filemask: *u8 /* void* */, flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_CreateFromFile(filemask, flags)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxXmlResource_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Get() -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_Get()
        }
    }
    #[fixed_stack_segment]
    fn GetDomain(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetDomain(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxXmlResource_GetFlags(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetVersion(_obj: *u8 /* void* */) -> c_long /* long */ {
        unsafe {
            wxXmlResource_GetVersion(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetXRCID(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxXmlResource_GetXRCID(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn InitAllHandlers(_obj: *u8 /* void* */) {
        unsafe {
            wxXmlResource_InitAllHandlers(_obj)
        }
    }
    #[fixed_stack_segment]
    fn InsertHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxXmlResource_InsertHandler(_obj, handler)
        }
    }
    #[fixed_stack_segment]
    fn Load(_obj: *u8 /* void* */, filemask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxXmlResource_Load(_obj, filemask)
        }
    }
    #[fixed_stack_segment]
    fn LoadBitmap(_obj: *u8 /* void* */, name: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxXmlResource_LoadBitmap(_obj, name, _ref)
        }
    }
    #[fixed_stack_segment]
    fn LoadDialog(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadDialog(_obj, parent, name)
        }
    }
    #[fixed_stack_segment]
    fn LoadFrame(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadFrame(_obj, parent, name)
        }
    }
    #[fixed_stack_segment]
    fn LoadIcon(_obj: *u8 /* void* */, name: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxXmlResource_LoadIcon(_obj, name, _ref)
        }
    }
    #[fixed_stack_segment]
    fn LoadMenu(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadMenu(_obj, name)
        }
    }
    #[fixed_stack_segment]
    fn LoadMenuBar(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadMenuBar(_obj, parent, name)
        }
    }
    #[fixed_stack_segment]
    fn LoadPanel(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadPanel(_obj, parent, name)
        }
    }
    #[fixed_stack_segment]
    fn LoadToolBar(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_LoadToolBar(_obj, parent, name)
        }
    }
    #[fixed_stack_segment]
    fn GetSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSizer(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetBoxSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetBoxSizer(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetStaticBoxSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticBoxSizer(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetGridSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetGridSizer(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetFlexGridSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetFlexGridSizer(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetBitmapButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetBitmapButton(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetButton(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetCalendarCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetCalendarCtrl(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetCheckBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetCheckBox(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetCheckListBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetCheckListBox(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetChoice(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetChoice(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetComboBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetComboBox(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetGauge(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetGauge(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetGrid(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetGrid(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetHtmlWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetHtmlWindow(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetListBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetListBox(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetListCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetListCtrl(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetMDIChildFrame(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMDIChildFrame(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetMDIParentFrame(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMDIParentFrame(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetMenu(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMenu(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetMenuBar(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMenuBar(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetMenuItem(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetMenuItem(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetNotebook(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetNotebook(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetPanel(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetPanel(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetRadioButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetRadioButton(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetRadioBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetRadioBox(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetScrollBar(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetScrollBar(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetScrolledWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetScrolledWindow(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetSlider(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSlider(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetSpinButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSpinButton(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetSpinCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSpinCtrl(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetSplitterWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetSplitterWindow(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetStaticBitmap(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticBitmap(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetStaticBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticBox(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetStaticLine(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticLine(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetStaticText(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStaticText(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetTextCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetTextCtrl(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn GetTreeCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetTreeCtrl(_obj, str_id)
        }
    }
    #[fixed_stack_segment]
    fn Unload(_obj: *u8 /* void* */, filemask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxXmlResource_Unload(_obj, filemask)
        }
    }
    #[fixed_stack_segment]
    fn Set(_obj: *u8 /* void* */, res: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_Set(_obj, res)
        }
    }
    #[fixed_stack_segment]
    fn SetDomain(_obj: *u8 /* void* */, domain: *u8 /* void* */) {
        unsafe {
            wxXmlResource_SetDomain(_obj, domain)
        }
    }
    #[fixed_stack_segment]
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxXmlResource_SetFlags(_obj, flags)
        }
    }
    #[fixed_stack_segment]
    fn GetStyledTextCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxXmlResource_GetStyledTextCtrl(_obj, str_id)
        }
    }
}
trait wxClientBase {
}
trait wxPrintDialog {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintDialog_Create(parent, data)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintDC(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintDialog_GetPrintDC(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPrintDialog_GetPrintData(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, bitmap: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBitmap_Create(_prt, _id, bitmap, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_GetBitmap(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetIcon(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_GetIcon(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn SetBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_SetBitmap(_obj, bitmap)
        }
    }
    #[fixed_stack_segment]
    fn SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) {
        unsafe {
            wxStaticBitmap_SetIcon(_obj, icon)
        }
    }
}
trait wxSVGFileDC {
    #[fixed_stack_segment]
    fn Create(fileName: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSVGFileDC_Create(fileName)
        }
    }
    #[fixed_stack_segment]
    fn CreateWithSize(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSVGFileDC_CreateWithSize(fileName, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn CreateWithSizeAndResolution(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, a_dpi: c_float /* float */) -> *u8 /* void* */ {
        unsafe {
            wxSVGFileDC_CreateWithSizeAndResolution(fileName, arg0, arg1, a_dpi)
        }
    }
    #[fixed_stack_segment]
    fn Delete(obj: *u8 /* void* */) {
        unsafe {
            wxSVGFileDC_Delete(obj)
        }
    }
}
trait wxStringClientData {
}
trait wxQueryLayoutInfoEvent {
    #[fixed_stack_segment]
    fn Create(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxQueryLayoutInfoEvent_Create(id)
        }
    }
    #[fixed_stack_segment]
    fn GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetAlignment(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetFlags(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetOrientation(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRequestedLength(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetRequestedLength(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxQueryLayoutInfoEvent_GetSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetAlignment(_obj: *u8 /* void* */, align: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetAlignment(_obj, align)
        }
    }
    #[fixed_stack_segment]
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetFlags(_obj, flags)
        }
    }
    #[fixed_stack_segment]
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetOrientation(_obj, orient)
        }
    }
    #[fixed_stack_segment]
    fn SetRequestedLength(_obj: *u8 /* void* */, length: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetRequestedLength(_obj, length)
        }
    }
    #[fixed_stack_segment]
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxQueryLayoutInfoEvent_SetSize(_obj, arg0, arg1)
        }
    }
}
trait wxGridCellAutoWrapStringRenderer {
    #[fixed_stack_segment]
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellAutoWrapStringRenderer_Ctor()
        }
    }
}
trait wxFileDialog {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _dir: *u8 /* void* */, _fle: *u8 /* void* */, _wcd: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_Create(_prt, _msg, _dir, _fle, _wcd, arg0, arg1, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetDirectory(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetDirectory(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFilename(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetFilename(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFilenames(_obj: *u8 /* void* */, paths: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxFileDialog_GetFilenames(_obj, paths)
        }
    }
    #[fixed_stack_segment]
    fn GetFilterIndex(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileDialog_GetFilterIndex(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMessage(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetMessage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetPath(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPaths(_obj: *u8 /* void* */, paths: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxFileDialog_GetPaths(_obj, paths)
        }
    }
    #[fixed_stack_segment]
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileDialog_GetStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWildcard(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileDialog_GetWildcard(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetDirectory(_obj: *u8 /* void* */, dir: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetDirectory(_obj, dir)
        }
    }
    #[fixed_stack_segment]
    fn SetFilename(_obj: *u8 /* void* */, name: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetFilename(_obj, name)
        }
    }
    #[fixed_stack_segment]
    fn SetFilterIndex(_obj: *u8 /* void* */, filterIndex: c_int /* int */) {
        unsafe {
            wxFileDialog_SetFilterIndex(_obj, filterIndex)
        }
    }
    #[fixed_stack_segment]
    fn SetMessage(_obj: *u8 /* void* */, message: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetMessage(_obj, message)
        }
    }
    #[fixed_stack_segment]
    fn SetPath(_obj: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetPath(_obj, path)
        }
    }
    #[fixed_stack_segment]
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxFileDialog_SetStyle(_obj, style)
        }
    }
    #[fixed_stack_segment]
    fn SetWildcard(_obj: *u8 /* void* */, wildCard: *u8 /* void* */) {
        unsafe {
            wxFileDialog_SetWildcard(_obj, wildCard)
        }
    }
}
trait wxDllLoader {
}
trait ELJLog {
    #[fixed_stack_segment]
    fn AddTraceMask(_obj: *u8 /* void* */, str: *wchar_t /* wchar_t* */) {
        unsafe {
            ELJLog_AddTraceMask(_obj, str)
        }
    }
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJLog_Create(_obj, _fnc)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DontCreateOnDemand(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_DontCreateOnDemand(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EnableLogging(_obj: *u8 /* void* */, doIt: bool /* bool */) -> c_int /* int */ {
        unsafe {
            ELJLog_EnableLogging(_obj, doIt)
        }
    }
    #[fixed_stack_segment]
    fn Flush(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_Flush(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FlushActive(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_FlushActive(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetActiveTarget() -> *u8 /* void* */ {
        unsafe {
            ELJLog_GetActiveTarget()
        }
    }
    #[fixed_stack_segment]
    fn GetTimestamp(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJLog_GetTimestamp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTraceMask(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJLog_GetTraceMask(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetVerbose(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJLog_GetVerbose(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasPendingMessages(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJLog_HasPendingMessages(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsAllowedTraceMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJLog_IsAllowedTraceMask(_obj, mask)
        }
    }
    #[fixed_stack_segment]
    fn IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            ELJLog_IsEnabled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn OnLog(_obj: *u8 /* void* */, level: c_int /* int */, szString: *u8 /* void* */, t: c_int /* int */) {
        unsafe {
            ELJLog_OnLog(_obj, level, szString, t)
        }
    }
    #[fixed_stack_segment]
    fn RemoveTraceMask(_obj: *u8 /* void* */, str: *wchar_t /* wchar_t* */) {
        unsafe {
            ELJLog_RemoveTraceMask(_obj, str)
        }
    }
    #[fixed_stack_segment]
    fn Resume(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_Resume(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetActiveTarget(pLogger: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJLog_SetActiveTarget(pLogger)
        }
    }
    #[fixed_stack_segment]
    fn SetTimestamp(_obj: *u8 /* void* */, ts: *u8 /* void* */) {
        unsafe {
            ELJLog_SetTimestamp(_obj, ts)
        }
    }
    #[fixed_stack_segment]
    fn SetTraceMask(_obj: *u8 /* void* */, ulMask: c_int /* int */) {
        unsafe {
            ELJLog_SetTraceMask(_obj, ulMask)
        }
    }
    #[fixed_stack_segment]
    fn SetVerbose(_obj: *u8 /* void* */, bVerbose: c_int /* int */) {
        unsafe {
            ELJLog_SetVerbose(_obj, bVerbose)
        }
    }
    #[fixed_stack_segment]
    fn Suspend(_obj: *u8 /* void* */) {
        unsafe {
            ELJLog_Suspend(_obj)
        }
    }
}
trait wxConnectionBase {
}
trait wxMenuItem {
    #[fixed_stack_segment]
    fn Check(_obj: *u8 /* void* */, check: bool /* bool */) {
        unsafe {
            wxMenuItem_Check(_obj, check)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMenuItem_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxMenuItem_Enable(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn GetHelp(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetHelp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuItem_GetId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetLabel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLabelFromText(text: *wchar_t /* wchar_t* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetLabelFromText(text)
        }
    }
    #[fixed_stack_segment]
    fn GetMenu(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetMenu(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSubMenu(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetSubMenu(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuItem_GetText(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsCheckable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsCheckable(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsChecked(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsChecked(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsEnabled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsSeparator(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsSeparator(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsSubMenu(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenuItem_IsSubMenu(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCheckable(_obj: *u8 /* void* */, checkable: bool /* bool */) {
        unsafe {
            wxMenuItem_SetCheckable(_obj, checkable)
        }
    }
    #[fixed_stack_segment]
    fn SetHelp(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxMenuItem_SetHelp(_obj, str)
        }
    }
    #[fixed_stack_segment]
    fn SetId(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxMenuItem_SetId(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn SetSubMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxMenuItem_SetSubMenu(_obj, menu)
        }
    }
    #[fixed_stack_segment]
    fn SetText(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxMenuItem_SetText(_obj, str)
        }
    }
}
trait wxDynamicLibrary {
}
trait wxSound {
    #[fixed_stack_segment]
    fn Create(fileName: *u8 /* void* */, isResource: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxSound_Create(fileName, isResource)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxSound_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSound_IsOk(self_)
        }
    }
    #[fixed_stack_segment]
    fn Play(self_: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSound_Play(self_, flag)
        }
    }
    #[fixed_stack_segment]
    fn Stop(self_: *u8 /* void* */) {
        unsafe {
            wxSound_Stop(self_)
        }
    }
}
trait wxMoveEvent {
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxMoveEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMoveEvent_GetPosition(_obj)
        }
    }
}
trait wxToolWindow {
    #[fixed_stack_segment]
    fn AddMiniButton(_obj: *u8 /* void* */, _btn: *u8 /* void* */) {
        unsafe {
            wxToolWindow_AddMiniButton(_obj, _btn)
        }
    }
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _btn: *u8 /* void* */, _ttl: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxToolWindow_Create(_obj, _btn, _ttl)
        }
    }
    #[fixed_stack_segment]
    fn GetClient(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxToolWindow_GetClient(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetClient(_obj: *u8 /* void* */, _wnd: *u8 /* void* */) {
        unsafe {
            wxToolWindow_SetClient(_obj, _wnd)
        }
    }
    #[fixed_stack_segment]
    fn SetTitleFont(_obj: *u8 /* void* */, _fnt: *u8 /* void* */) {
        unsafe {
            wxToolWindow_SetTitleFont(_obj, _fnt)
        }
    }
}
trait wxFontEnumerator {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFontEnumerator_Create(_obj, _fnc)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFontEnumerator_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EnumerateEncodings(_obj: *u8 /* void* */, facename: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontEnumerator_EnumerateEncodings(_obj, facename)
        }
    }
    #[fixed_stack_segment]
    fn EnumerateFacenames(_obj: *u8 /* void* */, encoding: c_int /* int */, fixedWidthOnly: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxFontEnumerator_EnumerateFacenames(_obj, encoding, fixedWidthOnly)
        }
    }
}
trait wxMultiCellCanvas {
    #[fixed_stack_segment]
    fn Add(_obj: *u8 /* void* */, win: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) {
        unsafe {
            wxMultiCellCanvas_Add(_obj, win, row, col)
        }
    }
    #[fixed_stack_segment]
    fn CalculateConstraints(_obj: *u8 /* void* */) {
        unsafe {
            wxMultiCellCanvas_CalculateConstraints(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, numRows: c_int /* int */, numCols: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMultiCellCanvas_Create(parent, numRows, numCols)
        }
    }
    #[fixed_stack_segment]
    fn MaxCols(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellCanvas_MaxCols(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MaxRows(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMultiCellCanvas_MaxRows(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetMinCellSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxMultiCellCanvas_SetMinCellSize(_obj, arg0, arg1)
        }
    }
}
trait cbStartBarDraggingEvent {
    #[fixed_stack_segment]
    fn Bar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbStartBarDraggingEvent_Bar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbStartBarDraggingEvent_Pos(_obj, arg0, arg1)
        }
    }
}
trait wxGridSizeEvent {
    #[fixed_stack_segment]
    fn GetRowOrCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizeEvent_GetRowOrCol(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridSizeEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridSizeEvent_ControlDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridSizeEvent_MetaDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridSizeEvent_ShiftDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridSizeEvent_AltDown(_obj)
        }
    }
}
trait wxFTP {
}
trait wxWizard {
    #[fixed_stack_segment]
    fn Chain(f: *u8 /* void* */, s: *u8 /* void* */) {
        unsafe {
            wxWizard_Chain(f, s)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWizard_Create(_prt, _id, _txt, _bmp, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn GetCurrentPage(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizard_GetCurrentPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPageSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWizard_GetPageSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RunWizard(_obj: *u8 /* void* */, firstPage: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxWizard_RunWizard(_obj, firstPage)
        }
    }
    #[fixed_stack_segment]
    fn SetPageSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxWizard_SetPageSize(_obj, arg0, arg1)
        }
    }
}
trait wxHtmlWinParser {
}
trait wxFontMapper {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxFontMapper_Create()
        }
    }
    #[fixed_stack_segment]
    fn GetAltForEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */, alt_encoding: *u8 /* void* */, _buf: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontMapper_GetAltForEncoding(_obj, encoding, alt_encoding, _buf)
        }
    }
    #[fixed_stack_segment]
    fn IsEncodingAvailable(_obj: *u8 /* void* */, encoding: c_int /* int */, _buf: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFontMapper_IsEncodingAvailable(_obj, encoding, _buf)
        }
    }
}
trait wxPrintData {
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxPrintData_Assign(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxPrintData_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPrintData_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCollate(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintData_GetCollate(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintData_GetColour(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDuplex(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetDuplex(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFilename(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetFilename(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFontMetricPath(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetFontMetricPath(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetNoCopies(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetNoCopies(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetOrientation(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPaperId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetPaperId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPaperSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPaperSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPreviewCommand(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPreviewCommand(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintMode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetPrintMode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrinterCommand(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPrinterCommand(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrinterName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPrinterName(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrinterOptions(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintData_GetPrinterOptions(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrinterScaleX(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxPrintData_GetPrinterScaleX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrinterScaleY(_obj: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxPrintData_GetPrinterScaleY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrinterTranslateX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetPrinterTranslateX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrinterTranslateY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetPrinterTranslateY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetQuality(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintData_GetQuality(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCollate(_obj: *u8 /* void* */, flag: c_int /* int */) {
        unsafe {
            wxPrintData_SetCollate(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn SetColour(_obj: *u8 /* void* */, colour: c_int /* int */) {
        unsafe {
            wxPrintData_SetColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetDuplex(_obj: *u8 /* void* */, duplex: c_int /* int */) {
        unsafe {
            wxPrintData_SetDuplex(_obj, duplex)
        }
    }
    #[fixed_stack_segment]
    fn SetFilename(_obj: *u8 /* void* */, filename: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetFilename(_obj, filename)
        }
    }
    #[fixed_stack_segment]
    fn SetFontMetricPath(_obj: *u8 /* void* */, path: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetFontMetricPath(_obj, path)
        }
    }
    #[fixed_stack_segment]
    fn SetNoCopies(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintData_SetNoCopies(_obj, v)
        }
    }
    #[fixed_stack_segment]
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */) {
        unsafe {
            wxPrintData_SetOrientation(_obj, orient)
        }
    }
    #[fixed_stack_segment]
    fn SetPaperId(_obj: *u8 /* void* */, sizeId: c_int /* int */) {
        unsafe {
            wxPrintData_SetPaperId(_obj, sizeId)
        }
    }
    #[fixed_stack_segment]
    fn SetPaperSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPrintData_SetPaperSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetPreviewCommand(_obj: *u8 /* void* */, command: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetPreviewCommand(_obj, command)
        }
    }
    #[fixed_stack_segment]
    fn SetPrintMode(_obj: *u8 /* void* */, printMode: c_int /* int */) {
        unsafe {
            wxPrintData_SetPrintMode(_obj, printMode)
        }
    }
    #[fixed_stack_segment]
    fn SetPrinterCommand(_obj: *u8 /* void* */, command: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetPrinterCommand(_obj, command)
        }
    }
    #[fixed_stack_segment]
    fn SetPrinterName(_obj: *u8 /* void* */, name: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetPrinterName(_obj, name)
        }
    }
    #[fixed_stack_segment]
    fn SetPrinterOptions(_obj: *u8 /* void* */, options: *u8 /* void* */) {
        unsafe {
            wxPrintData_SetPrinterOptions(_obj, options)
        }
    }
    #[fixed_stack_segment]
    fn SetPrinterScaleX(_obj: *u8 /* void* */, x: c_double /* double */) {
        unsafe {
            wxPrintData_SetPrinterScaleX(_obj, x)
        }
    }
    #[fixed_stack_segment]
    fn SetPrinterScaleY(_obj: *u8 /* void* */, y: c_double /* double */) {
        unsafe {
            wxPrintData_SetPrinterScaleY(_obj, y)
        }
    }
    #[fixed_stack_segment]
    fn SetPrinterScaling(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */) {
        unsafe {
            wxPrintData_SetPrinterScaling(_obj, x, y)
        }
    }
    #[fixed_stack_segment]
    fn SetPrinterTranslateX(_obj: *u8 /* void* */, x: c_int /* int */) {
        unsafe {
            wxPrintData_SetPrinterTranslateX(_obj, x)
        }
    }
    #[fixed_stack_segment]
    fn SetPrinterTranslateY(_obj: *u8 /* void* */, y: c_int /* int */) {
        unsafe {
            wxPrintData_SetPrinterTranslateY(_obj, y)
        }
    }
    #[fixed_stack_segment]
    fn SetPrinterTranslation(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxPrintData_SetPrinterTranslation(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetQuality(_obj: *u8 /* void* */, quality: c_int /* int */) {
        unsafe {
            wxPrintData_SetQuality(_obj, quality)
        }
    }
}
trait wxSashEvent {
    #[fixed_stack_segment]
    fn Create(id: c_int /* int */, edge: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSashEvent_Create(id, edge)
        }
    }
    #[fixed_stack_segment]
    fn GetDragRect(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSashEvent_GetDragRect(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDragStatus(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashEvent_GetDragStatus(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEdge(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSashEvent_GetEdge(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetDragRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxSashEvent_SetDragRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn SetDragStatus(_obj: *u8 /* void* */, status: c_int /* int */) {
        unsafe {
            wxSashEvent_SetDragStatus(_obj, status)
        }
    }
    #[fixed_stack_segment]
    fn SetEdge(_obj: *u8 /* void* */, edge: c_int /* int */) {
        unsafe {
            wxSashEvent_SetEdge(_obj, edge)
        }
    }
}
trait cbPluginEvent {
    #[fixed_stack_segment]
    fn Pane(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbPluginEvent_Pane(_obj)
        }
    }
}
trait wxPropertyGridEvent {
    #[fixed_stack_segment]
    fn HasProperty(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPropertyGridEvent_HasProperty(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetProperty(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPropertyGridEvent_GetProperty(_obj)
        }
    }
}
trait wxSingleChoiceDialog {
}
trait wxObject {
    #[fixed_stack_segment]
    fn GetClassInfo(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxObject_GetClassInfo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsKindOf(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxObject_IsKindOf(_obj, classInfo)
        }
    }
    #[fixed_stack_segment]
    fn IsScrolledWindow(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxObject_IsScrolledWindow(_obj)
        }
    }
}
trait wxDropSource {
    #[fixed_stack_segment]
    fn DropSource_Create(data: *u8 /* void* */, win: *u8 /* void* */, copy: *u8 /* void* */, move: *u8 /* void* */, none: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            DropSource_Create(data, win, copy, move, none)
        }
    }
    #[fixed_stack_segment]
    fn DropSource_Delete(_obj: *u8 /* void* */) {
        unsafe {
            DropSource_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DropSource_DoDragDrop(_obj: *u8 /* void* */, _move: c_int /* int */) -> c_int /* int */ {
        unsafe {
            DropSource_DoDragDrop(_obj, _move)
        }
    }
}
trait wxHtmlParser {
}
trait wxFindReplaceData {
    #[fixed_stack_segment]
    fn Create(flags: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceData_Create(flags)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceData_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFindReplaceData_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFindString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceData_GetFindString(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFindReplaceData_GetFlags(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetReplaceString(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceData_GetReplaceString(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetFindString(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxFindReplaceData_SetFindString(_obj, str)
        }
    }
    #[fixed_stack_segment]
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxFindReplaceData_SetFlags(_obj, flags)
        }
    }
    #[fixed_stack_segment]
    fn SetReplaceString(_obj: *u8 /* void* */, str: *u8 /* void* */) {
        unsafe {
            wxFindReplaceData_SetReplaceString(_obj, str)
        }
    }
}
trait wxToolLayoutItem {
    #[fixed_stack_segment]
    fn IsSeparator(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxToolLayoutItem_IsSeparator(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Rect(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxToolLayoutItem_Rect(_obj, arg0, arg1, arg2, arg3)
        }
    }
}
trait cbLeftDownEvent {
    #[fixed_stack_segment]
    fn Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbLeftDownEvent_Pos(_obj, arg0, arg1)
        }
    }
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
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStatusBar_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetBorderX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStatusBar_GetBorderX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBorderY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStatusBar_GetBorderY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFieldsCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxStatusBar_GetFieldsCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStatusText(_obj: *u8 /* void* */, number: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStatusBar_GetStatusText(_obj, number)
        }
    }
    #[fixed_stack_segment]
    fn SetFieldsCount(_obj: *u8 /* void* */, number: c_int /* int */, widths: *c_int /* int* */) {
        unsafe {
            wxStatusBar_SetFieldsCount(_obj, number, widths)
        }
    }
    #[fixed_stack_segment]
    fn SetMinHeight(_obj: *u8 /* void* */, height: c_int /* int */) {
        unsafe {
            wxStatusBar_SetMinHeight(_obj, height)
        }
    }
    #[fixed_stack_segment]
    fn SetStatusText(_obj: *u8 /* void* */, text: *u8 /* void* */, number: c_int /* int */) {
        unsafe {
            wxStatusBar_SetStatusText(_obj, text, number)
        }
    }
    #[fixed_stack_segment]
    fn SetStatusWidths(_obj: *u8 /* void* */, n: c_int /* int */, widths: *c_int /* int* */) {
        unsafe {
            wxStatusBar_SetStatusWidths(_obj, n, widths)
        }
    }
}
trait wxTextDataObject {
    #[fixed_stack_segment]
    fn TextDataObject_Create(_txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            TextDataObject_Create(_txt)
        }
    }
    #[fixed_stack_segment]
    fn TextDataObject_Delete(_obj: *u8 /* void* */) {
        unsafe {
            TextDataObject_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn TextDataObject_GetTextLength(_obj: *u8 /* void* */) -> size_t /* size_t */ {
        unsafe {
            TextDataObject_GetTextLength(_obj)
        }
    }
    #[fixed_stack_segment]
    fn TextDataObject_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            TextDataObject_GetText(_obj)
        }
    }
    #[fixed_stack_segment]
    fn TextDataObject_SetText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            TextDataObject_SetText(_obj, text)
        }
    }
}
trait wxSizer {
    #[fixed_stack_segment]
    fn Add(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_Add(_obj, arg0, arg1, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn AddSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_AddSizer(_obj, sizer, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn AddWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_AddWindow(_obj, window, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_CalcMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Fit(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizer_Fit(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn GetChildren(_obj: *u8 /* void* */, _res: *u8 /* void* */, _cnt: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxSizer_GetChildren(_obj, _res, _cnt)
        }
    }
    #[fixed_stack_segment]
    fn GetMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetMinSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Insert(_obj: *u8 /* void* */, before: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_Insert(_obj, before, arg0, arg1, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn InsertSizer(_obj: *u8 /* void* */, before: c_int /* int */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_InsertSizer(_obj, before, sizer, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn InsertWindow(_obj: *u8 /* void* */, before: c_int /* int */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_InsertWindow(_obj, before, window, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn Layout(_obj: *u8 /* void* */) {
        unsafe {
            wxSizer_Layout(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Prepend(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_Prepend(_obj, arg0, arg1, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn PrependSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_PrependSizer(_obj, sizer, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn PrependWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) {
        unsafe {
            wxSizer_PrependWindow(_obj, window, option, flag, border, userData)
        }
    }
    #[fixed_stack_segment]
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxSizer_RecalcSizes(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetDimension(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxSizer_SetDimension(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn SetItemMinSize(_obj: *u8 /* void* */, pos: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizer_SetItemMinSize(_obj, pos, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetItemMinSizeSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizer_SetItemMinSizeSizer(_obj, sizer, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetItemMinSizeWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizer_SetItemMinSizeWindow(_obj, window, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetMinSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxSizer_SetMinSize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetSizeHints(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizer_SetSizeHints(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn AddSpacer(_obj: *u8 /* void* */, size: c_int /* int */) {
        unsafe {
            wxSizer_AddSpacer(_obj, size)
        }
    }
    #[fixed_stack_segment]
    fn AddStretchSpacer(_obj: *u8 /* void* */, size: c_int /* int */) {
        unsafe {
            wxSizer_AddStretchSpacer(_obj, size)
        }
    }
    #[fixed_stack_segment]
    fn Clear(_obj: *u8 /* void* */, delete_windows: bool /* bool */) {
        unsafe {
            wxSizer_Clear(_obj, delete_windows)
        }
    }
    #[fixed_stack_segment]
    fn DetachWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_DetachWindow(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn DetachSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_DetachSizer(_obj, sizer)
        }
    }
    #[fixed_stack_segment]
    fn Detach(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSizer_Detach(_obj, index)
        }
    }
    #[fixed_stack_segment]
    fn FitInside(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizer_FitInside(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn GetContainingWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetContainingWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetItemWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, recursive: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetItemWindow(_obj, window, recursive)
        }
    }
    #[fixed_stack_segment]
    fn GetItemSizer(_obj: *u8 /* void* */, window: *u8 /* void* */, recursive: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetItemSizer(_obj, window, recursive)
        }
    }
    #[fixed_stack_segment]
    fn GetItem(_obj: *u8 /* void* */, index: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_GetItem(_obj, index)
        }
    }
    #[fixed_stack_segment]
    fn HideWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_HideWindow(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn HideSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_HideSizer(_obj, sizer)
        }
    }
    #[fixed_stack_segment]
    fn Hide(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSizer_Hide(_obj, index)
        }
    }
    #[fixed_stack_segment]
    fn InsertSpacer(_obj: *u8 /* void* */, index: c_int /* int */, size: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_InsertSpacer(_obj, index, size)
        }
    }
    #[fixed_stack_segment]
    fn InsertStretchSpacer(_obj: *u8 /* void* */, index: c_int /* int */, prop: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_InsertStretchSpacer(_obj, index, prop)
        }
    }
    #[fixed_stack_segment]
    fn IsShownWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_IsShownWindow(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn IsShownSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_IsShownSizer(_obj, sizer)
        }
    }
    #[fixed_stack_segment]
    fn IsShown(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxSizer_IsShown(_obj, index)
        }
    }
    #[fixed_stack_segment]
    fn PrependSpacer(_obj: *u8 /* void* */, size: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_PrependSpacer(_obj, size)
        }
    }
    #[fixed_stack_segment]
    fn PrependStretchSpacer(_obj: *u8 /* void* */, prop: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSizer_PrependStretchSpacer(_obj, prop)
        }
    }
    #[fixed_stack_segment]
    fn ReplaceWindow(_obj: *u8 /* void* */, oldwin: *u8 /* void* */, newwin: *u8 /* void* */, recursive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_ReplaceWindow(_obj, oldwin, newwin, recursive)
        }
    }
    #[fixed_stack_segment]
    fn ReplaceSizer(_obj: *u8 /* void* */, oldsz: *u8 /* void* */, newsz: *u8 /* void* */, recursive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_ReplaceSizer(_obj, oldsz, newsz, recursive)
        }
    }
    #[fixed_stack_segment]
    fn Replace(_obj: *u8 /* void* */, oldindex: c_int /* int */, newitem: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSizer_Replace(_obj, oldindex, newitem)
        }
    }
    #[fixed_stack_segment]
    fn SetVirtualSizeHints(_obj: *u8 /* void* */, window: *u8 /* void* */) {
        unsafe {
            wxSizer_SetVirtualSizeHints(_obj, window)
        }
    }
    #[fixed_stack_segment]
    fn ShowWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, show: bool /* bool */, recursive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_ShowWindow(_obj, window, show, recursive)
        }
    }
    #[fixed_stack_segment]
    fn ShowSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, show: bool /* bool */, recursive: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_ShowSizer(_obj, sizer, show, recursive)
        }
    }
    #[fixed_stack_segment]
    fn Show(_obj: *u8 /* void* */, sizer: *u8 /* void* */, index: c_int /* int */, show: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxSizer_Show(_obj, sizer, index, show)
        }
    }
}
trait wxMenu {
    #[fixed_stack_segment]
    fn Append(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */) {
        unsafe {
            wxMenu_Append(_obj, id, text, help, isCheckable)
        }
    }
    #[fixed_stack_segment]
    fn AppendItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_AppendItem(_obj, _itm)
        }
    }
    #[fixed_stack_segment]
    fn AppendSeparator(_obj: *u8 /* void* */) {
        unsafe {
            wxMenu_AppendSeparator(_obj)
        }
    }
    #[fixed_stack_segment]
    fn AppendSub(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */) {
        unsafe {
            wxMenu_AppendSub(_obj, id, text, submenu, help)
        }
    }
    #[fixed_stack_segment]
    fn Break(_obj: *u8 /* void* */) {
        unsafe {
            wxMenu_Break(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Check(_obj: *u8 /* void* */, id: c_int /* int */, check: bool /* bool */) {
        unsafe {
            wxMenu_Check(_obj, id, check)
        }
    }
    #[fixed_stack_segment]
    fn Create(title: *u8 /* void* */, style: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_Create(title, style)
        }
    }
    #[fixed_stack_segment]
    fn DeleteById(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxMenu_DeleteById(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn DeleteByItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_DeleteByItem(_obj, _itm)
        }
    }
    #[fixed_stack_segment]
    fn DeletePointer(_obj: *u8 /* void* */) {
        unsafe {
            wxMenu_DeletePointer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DestroyById(_obj: *u8 /* void* */, id: c_int /* int */) {
        unsafe {
            wxMenu_DestroyById(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn DestroyByItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_DestroyByItem(_obj, _itm)
        }
    }
    #[fixed_stack_segment]
    fn Enable(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */) {
        unsafe {
            wxMenu_Enable(_obj, id, enable)
        }
    }
    #[fixed_stack_segment]
    fn FindItem(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_FindItem(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn FindItemByLabel(_obj: *u8 /* void* */, itemString: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenu_FindItemByLabel(_obj, itemString)
        }
    }
    #[fixed_stack_segment]
    fn GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetClientData(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetHelpString(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetHelpString(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn GetInvokingWindow(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetInvokingWindow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLabel(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetLabel(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn GetMenuItemCount(_obj: *u8 /* void* */) -> size_t /* size_t */ {
        unsafe {
            wxMenu_GetMenuItemCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMenuItems(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenu_GetMenuItems(_obj, _lst)
        }
    }
    #[fixed_stack_segment]
    fn GetParent(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetParent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenu_GetStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetTitle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Insert(_obj: *u8 /* void* */, pos: size_t /* size_t */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */) {
        unsafe {
            wxMenu_Insert(_obj, pos, id, text, help, isCheckable)
        }
    }
    #[fixed_stack_segment]
    fn InsertItem(_obj: *u8 /* void* */, pos: size_t /* size_t */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_InsertItem(_obj, pos, _itm)
        }
    }
    #[fixed_stack_segment]
    fn InsertSub(_obj: *u8 /* void* */, pos: size_t /* size_t */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */) {
        unsafe {
            wxMenu_InsertSub(_obj, pos, id, text, submenu, help)
        }
    }
    #[fixed_stack_segment]
    fn IsAttached(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMenu_IsAttached(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsChecked(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMenu_IsChecked(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn IsEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxMenu_IsEnabled(_obj, id)
        }
    }
    #[fixed_stack_segment]
    fn Prepend(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */) {
        unsafe {
            wxMenu_Prepend(_obj, id, text, help, isCheckable)
        }
    }
    #[fixed_stack_segment]
    fn PrependItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_PrependItem(_obj, _itm)
        }
    }
    #[fixed_stack_segment]
    fn PrependSub(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */) {
        unsafe {
            wxMenu_PrependSub(_obj, id, text, submenu, help)
        }
    }
    #[fixed_stack_segment]
    fn RemoveById(_obj: *u8 /* void* */, id: c_int /* int */, _itm: *u8 /* void* */) {
        unsafe {
            wxMenu_RemoveById(_obj, id, _itm)
        }
    }
    #[fixed_stack_segment]
    fn RemoveByItem(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxMenu_RemoveByItem(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn SetClientData(_obj: *u8 /* void* */, clientData: *u8 /* void* */) {
        unsafe {
            wxMenu_SetClientData(_obj, clientData)
        }
    }
    #[fixed_stack_segment]
    fn SetEventHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxMenu_SetEventHandler(_obj, handler)
        }
    }
    #[fixed_stack_segment]
    fn SetHelpString(_obj: *u8 /* void* */, id: c_int /* int */, helpString: *u8 /* void* */) {
        unsafe {
            wxMenu_SetHelpString(_obj, id, helpString)
        }
    }
    #[fixed_stack_segment]
    fn SetInvokingWindow(_obj: *u8 /* void* */, win: *u8 /* void* */) {
        unsafe {
            wxMenu_SetInvokingWindow(_obj, win)
        }
    }
    #[fixed_stack_segment]
    fn SetLabel(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxMenu_SetLabel(_obj, id, label)
        }
    }
    #[fixed_stack_segment]
    fn SetParent(_obj: *u8 /* void* */, parent: *u8 /* void* */) {
        unsafe {
            wxMenu_SetParent(_obj, parent)
        }
    }
    #[fixed_stack_segment]
    fn SetTitle(_obj: *u8 /* void* */, title: *u8 /* void* */) {
        unsafe {
            wxMenu_SetTitle(_obj, title)
        }
    }
    #[fixed_stack_segment]
    fn UpdateUI(_obj: *u8 /* void* */, source: *u8 /* void* */) {
        unsafe {
            wxMenu_UpdateUI(_obj, source)
        }
    }
}
trait wxFileType {
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFileType_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ExpandCommand(_obj: *u8 /* void* */, _cmd: *u8 /* void* */, _params: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileType_ExpandCommand(_obj, _cmd, _params)
        }
    }
    #[fixed_stack_segment]
    fn GetDescription(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileType_GetDescription(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetExtensions(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetExtensions(_obj, _lst)
        }
    }
    #[fixed_stack_segment]
    fn GetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetIcon(_obj, icon)
        }
    }
    #[fixed_stack_segment]
    fn GetMimeType(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileType_GetMimeType(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMimeTypes(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetMimeTypes(_obj, _lst)
        }
    }
    #[fixed_stack_segment]
    fn GetOpenCommand(_obj: *u8 /* void* */, _buf: *u8 /* void* */, _params: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetOpenCommand(_obj, _buf, _params)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintCommand(_obj: *u8 /* void* */, _buf: *u8 /* void* */, _params: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileType_GetPrintCommand(_obj, _buf, _params)
        }
    }
}
trait wxRadioButton {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxRadioButton_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxRadioButton_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, value: bool /* bool */) {
        unsafe {
            wxRadioButton_SetValue(_obj, value)
        }
    }
}
trait wxFontList {
}
trait cbDrawRowHandlesEvent {
    #[fixed_stack_segment]
    fn Dc(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawRowHandlesEvent_Dc(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Row(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawRowHandlesEvent_Row(_obj)
        }
    }
}
trait wxCheckListBox {
    #[fixed_stack_segment]
    fn Check(_obj: *u8 /* void* */, item: c_int /* int */, check: bool /* bool */) {
        unsafe {
            wxCheckListBox_Check(_obj, item, check)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCheckListBox_Create(_prt, _id, arg0, arg1, arg2, arg3, arg4, arg5, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCheckListBox_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn CanVeto(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCloseEvent_CanVeto(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxCloseEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetLoggingOff(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCloseEvent_GetLoggingOff(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetVeto(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCloseEvent_GetVeto(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCanVeto(_obj: *u8 /* void* */, canVeto: bool /* bool */) {
        unsafe {
            wxCloseEvent_SetCanVeto(_obj, canVeto)
        }
    }
    #[fixed_stack_segment]
    fn SetLoggingOff(_obj: *u8 /* void* */, logOff: bool /* bool */) {
        unsafe {
            wxCloseEvent_SetLoggingOff(_obj, logOff)
        }
    }
    #[fixed_stack_segment]
    fn Veto(_obj: *u8 /* void* */, veto: bool /* bool */) {
        unsafe {
            wxCloseEvent_Veto(_obj, veto)
        }
    }
}
trait wxFileProperty {
    #[fixed_stack_segment]
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFileProperty_Create(label, name, value)
        }
    }
}
trait wxChoice {
    #[fixed_stack_segment]
    fn Append(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxChoice_Append(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxChoice_Clear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxChoice_Create(_prt, _id, arg0, arg1, arg2, arg3, arg4, arg5, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxChoice_Delete(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxChoice_FindString(_obj, s)
        }
    }
    #[fixed_stack_segment]
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxChoice_GetCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxChoice_GetSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxChoice_GetString(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn SetSelection(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxChoice_SetSelection(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn SetString(_obj: *u8 /* void* */, n: c_int /* int */, s: *u8 /* void* */) {
        unsafe {
            wxChoice_SetString(_obj, n, s)
        }
    }
}
trait wxIntProperty {
    #[fixed_stack_segment]
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxIntProperty_Create(label, name, value)
        }
    }
}
trait wxDbColFor {
}
trait wxMenuEvent {
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxMenuEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMenuId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMenuEvent_GetMenuId(_obj)
        }
    }
}
trait wxCalendarDateAttr {
    #[fixed_stack_segment]
    fn Create(_ctxt: *u8 /* void* */, _cbck: *u8 /* void* */, _cbrd: *u8 /* void* */, _fnt: *u8 /* void* */, _brd: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxCalendarDateAttr_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_GetBackgroundColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetBorder(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCalendarDateAttr_GetBorder(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBorderColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_GetBorderColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_GetFont(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_GetTextColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasBackgroundColour(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasBorder(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasBorder(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasBorderColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasBorderColour(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasFont(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasFont(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_HasTextColour(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsHoliday(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCalendarDateAttr_IsHoliday(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetBackgroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_SetBackgroundColour(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetBorder(_obj: *u8 /* void* */, border: c_int /* int */) {
        unsafe {
            wxCalendarDateAttr_SetBorder(_obj, border)
        }
    }
    #[fixed_stack_segment]
    fn SetBorderColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_SetBorderColour(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_SetFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn SetHoliday(_obj: *u8 /* void* */, holiday: c_int /* int */) {
        unsafe {
            wxCalendarDateAttr_SetHoliday(_obj, holiday)
        }
    }
    #[fixed_stack_segment]
    fn SetTextColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxCalendarDateAttr_SetTextColour(_obj, col)
        }
    }
}
trait wxGridCellNumberRenderer {
    #[fixed_stack_segment]
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellNumberRenderer_Ctor()
        }
    }
}
trait wxSTCDoc {
}
trait wxMessageDialog {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _cap: *u8 /* void* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxMessageDialog_Create(_prt, _msg, _cap, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMessageDialog_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ShowModal(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMessageDialog_ShowModal(_obj)
        }
    }
}
trait wxPrivateDropTarget {
}
trait wxEncodingConverter {
    #[fixed_stack_segment]
    fn Convert(_obj: *u8 /* void* */, input: *u8 /* void* */, output: *u8 /* void* */) {
        unsafe {
            wxEncodingConverter_Convert(_obj, input, output)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxEncodingConverter_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxEncodingConverter_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetAllEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEncodingConverter_GetAllEquivalents(_obj, enc, _lst)
        }
    }
    #[fixed_stack_segment]
    fn GetPlatformEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, platform: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEncodingConverter_GetPlatformEquivalents(_obj, enc, platform, _lst)
        }
    }
    #[fixed_stack_segment]
    fn Init(_obj: *u8 /* void* */, input_enc: c_int /* int */, output_enc: c_int /* int */, method: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxEncodingConverter_Init(_obj, input_enc, output_enc, method)
        }
    }
}
trait wxCriticalSection {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxCriticalSection_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCriticalSection_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Enter(_obj: *u8 /* void* */) {
        unsafe {
            wxCriticalSection_Enter(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Leave(_obj: *u8 /* void* */) {
        unsafe {
            wxCriticalSection_Leave(_obj)
        }
    }
}
trait wxTextFile {
}
trait wxUpdateUIEvent {
    #[fixed_stack_segment]
    fn Check(_obj: *u8 /* void* */, check: bool /* bool */) {
        unsafe {
            wxUpdateUIEvent_Check(_obj, check)
        }
    }
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxUpdateUIEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxUpdateUIEvent_Enable(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn GetChecked(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetChecked(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetEnabled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSetChecked(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetSetChecked(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSetEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetSetEnabled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSetText(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxUpdateUIEvent_GetSetText(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxUpdateUIEvent_GetText(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetText(_obj: *u8 /* void* */, text: *u8 /* void* */) {
        unsafe {
            wxUpdateUIEvent_SetText(_obj, text)
        }
    }
}
trait wxTimeSpan {
}
trait wxGridSizer {
    #[fixed_stack_segment]
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridSizer_CalcMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(rows: c_int /* int */, cols: c_int /* int */, vgap: c_int /* int */, hgap: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridSizer_Create(rows, cols, vgap, hgap)
        }
    }
    #[fixed_stack_segment]
    fn GetCols(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizer_GetCols(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetHGap(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizer_GetHGap(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRows(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizer_GetRows(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetVGap(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridSizer_GetVGap(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxGridSizer_RecalcSizes(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCols(_obj: *u8 /* void* */, cols: c_int /* int */) {
        unsafe {
            wxGridSizer_SetCols(_obj, cols)
        }
    }
    #[fixed_stack_segment]
    fn SetHGap(_obj: *u8 /* void* */, gap: c_int /* int */) {
        unsafe {
            wxGridSizer_SetHGap(_obj, gap)
        }
    }
    #[fixed_stack_segment]
    fn SetRows(_obj: *u8 /* void* */, rows: c_int /* int */) {
        unsafe {
            wxGridSizer_SetRows(_obj, rows)
        }
    }
    #[fixed_stack_segment]
    fn SetVGap(_obj: *u8 /* void* */, gap: c_int /* int */) {
        unsafe {
            wxGridSizer_SetVGap(_obj, gap)
        }
    }
}
trait wxThread {
}
trait ELJMessageParameters {
    #[fixed_stack_segment]
    fn wxMessageParameters_Create(_file: *wchar_t /* wchar_t* */, _type: *wchar_t /* wchar_t* */, _object: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMessageParameters_Create(_file, _type, _object, _func)
        }
    }
    #[fixed_stack_segment]
    fn wxMessageParameters_Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMessageParameters_Delete(_obj)
        }
    }
}
trait wxPopupTransientWindow {
}
trait ELJLocale {
}
trait wxEditableListBox {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, label: *wchar_t /* wchar_t* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxEditableListBox_Create(parent, id, label, arg0, arg1, arg2, arg3, style)
        }
    }
    #[fixed_stack_segment]
    fn GetDelButton(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEditableListBox_GetDelButton(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDownButton(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEditableListBox_GetDownButton(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEditButton(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEditableListBox_GetEditButton(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetListCtrl(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEditableListBox_GetListCtrl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetNewButton(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEditableListBox_GetNewButton(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStrings(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxEditableListBox_GetStrings(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetUpButton(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEditableListBox_GetUpButton(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetStrings(_obj: *u8 /* void* */, strings: *u8 /* void* */, _n: c_int /* int */) {
        unsafe {
            wxEditableListBox_SetStrings(_obj, strings, _n)
        }
    }
}
trait cbDrawPaneBkGroundEvent {
    #[fixed_stack_segment]
    fn Dc(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawPaneBkGroundEvent_Dc(_obj)
        }
    }
}
trait wxcPrintoutHandler {
    #[fixed_stack_segment]
    fn wxcPrintout_Create(title: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcPrintout_Create(title)
        }
    }
    #[fixed_stack_segment]
    fn wxcPrintout_Delete(self_: *u8 /* void* */) {
        unsafe {
            wxcPrintout_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxcPrintout_SetPageLimits(self_: *u8 /* void* */, startPage: c_int /* int */, endPage: c_int /* int */, fromPage: c_int /* int */, toPage: c_int /* int */) {
        unsafe {
            wxcPrintout_SetPageLimits(self_, startPage, endPage, fromPage, toPage)
        }
    }
    #[fixed_stack_segment]
    fn wxcPrintout_GetEvtHandler(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcPrintout_GetEvtHandler(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxcPrintEvent_GetPrintout(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcPrintEvent_GetPrintout(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxcPrintEvent_GetPage(self_: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxcPrintEvent_GetPage(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxcPrintEvent_GetEndPage(self_: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxcPrintEvent_GetEndPage(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxcPrintEvent_GetContinue(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxcPrintEvent_GetContinue(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxcPrintEvent_SetContinue(self_: *u8 /* void* */, cont: bool /* bool */) {
        unsafe {
            wxcPrintEvent_SetContinue(self_, cont)
        }
    }
    #[fixed_stack_segment]
    fn wxcPrintEvent_SetPageLimits(self_: *u8 /* void* */, startPage: c_int /* int */, endPage: c_int /* int */, fromPage: c_int /* int */, toPage: c_int /* int */) {
        unsafe {
            wxcPrintEvent_SetPageLimits(self_, startPage, endPage, fromPage, toPage)
        }
    }
    #[fixed_stack_segment]
    fn wxInputStream_CanRead(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxInputStream_CanRead(self_)
        }
    }
}
trait wxSize {
    #[fixed_stack_segment]
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxSize_Create(arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Destroy(_obj: *u8 /* void* */) {
        unsafe {
            wxSize_Destroy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSize_GetHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSize_GetWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetHeight(_obj: *u8 /* void* */, h: c_int /* int */) {
        unsafe {
            wxSize_SetHeight(_obj, h)
        }
    }
    #[fixed_stack_segment]
    fn SetWidth(_obj: *u8 /* void* */, w: c_int /* int */) {
        unsafe {
            wxSize_SetWidth(_obj, w)
        }
    }
}
trait cbDynToolBarDimHandler {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            cbDynToolBarDimHandler_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbDynToolBarDimHandler_Delete(_obj)
        }
    }
}
trait cbGCUpdatesMgr {
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbGCUpdatesMgr_Create(pPanel)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbGCUpdatesMgr_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbGCUpdatesMgr_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn UpdateNow(_obj: *u8 /* void* */) {
        unsafe {
            cbGCUpdatesMgr_UpdateNow(_obj)
        }
    }
}
trait wxFileDataObject {
    #[fixed_stack_segment]
    fn FileDataObject_AddFile(_obj: *u8 /* void* */, _fle: *u8 /* void* */) {
        unsafe {
            FileDataObject_AddFile(_obj, _fle)
        }
    }
    #[fixed_stack_segment]
    fn FileDataObject_Create(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */) -> *u8 /* void* */ {
        unsafe {
            FileDataObject_Create(arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn FileDataObject_Delete(_obj: *u8 /* void* */) {
        unsafe {
            FileDataObject_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn FileDataObject_GetFilenames(_obj: *u8 /* void* */, _lst: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            FileDataObject_GetFilenames(_obj, _lst)
        }
    }
}
trait wxGraphicsFont {
}
trait wxDocParentFrame {
}
trait wxSliderMSW {
    #[fixed_stack_segment]
    fn wxObject_Delete(obj: *u8 /* void* */) {
        unsafe {
            wxObject_Delete(obj)
        }
    }
    #[fixed_stack_segment]
    fn wxFrame_GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrame_GetTitle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxFrame_SetTitle(_frame: *u8 /* void* */, _txt: *u8 /* void* */) {
        unsafe {
            wxFrame_SetTitle(_frame, _txt)
        }
    }
    #[fixed_stack_segment]
    fn wxFrame_SetShape(self_: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFrame_SetShape(self_, region)
        }
    }
    #[fixed_stack_segment]
    fn wxFrame_ShowFullScreen(self_: *u8 /* void* */, show: bool /* bool */, style: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxFrame_ShowFullScreen(self_, show, style)
        }
    }
    #[fixed_stack_segment]
    fn wxFrame_IsFullScreen(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFrame_IsFullScreen(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxFrame_Centre(self_: *u8 /* void* */, orientation: c_int /* int */) {
        unsafe {
            wxFrame_Centre(self_, orientation)
        }
    }
    #[fixed_stack_segment]
    fn wxCursor_Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCursor_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxDateTime_Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxDateTime_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxMouseEvent_GetWheelDelta(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetWheelDelta(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxMouseEvent_GetWheelRotation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetWheelRotation(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxMouseEvent_GetButton(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMouseEvent_GetButton(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxcGetMousePosition() -> *u8 /* void* */ {
        unsafe {
            wxcGetMousePosition()
        }
    }
    #[fixed_stack_segment]
    fn wxDC_GetUserScaleX(dc: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxDC_GetUserScaleX(dc)
        }
    }
    #[fixed_stack_segment]
    fn wxDC_GetUserScaleY(dc: *u8 /* void* */) -> c_double /* double */ {
        unsafe {
            wxDC_GetUserScaleY(dc)
        }
    }
    #[fixed_stack_segment]
    fn wxWindow_ConvertDialogToPixelsEx(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ConvertDialogToPixelsEx(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxWindow_ConvertPixelsToDialogEx(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ConvertPixelsToDialogEx(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxWindow_ScreenToClient2(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxWindow_ScreenToClient2(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn wxString_Create(buffer: *wchar_t /* wchar_t* */) -> *u8 /* void* */ {
        unsafe {
            wxString_Create(buffer)
        }
    }
    #[fixed_stack_segment]
    fn wxString_CreateLen(buffer: *wchar_t /* wchar_t* */, len: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxString_CreateLen(buffer, len)
        }
    }
    #[fixed_stack_segment]
    fn wxString_Delete(s: *u8 /* void* */) {
        unsafe {
            wxString_Delete(s)
        }
    }
    #[fixed_stack_segment]
    fn wxString_GetString(s: *u8 /* void* */, buffer: *wchar_t /* wchar_t* */) -> c_int /* int */ {
        unsafe {
            wxString_GetString(s, buffer)
        }
    }
    #[fixed_stack_segment]
    fn wxString_Length(s: *u8 /* void* */) -> size_t /* size_t */ {
        unsafe {
            wxString_Length(s)
        }
    }
    #[fixed_stack_segment]
    fn wxMenu_GetMenuBar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenu_GetMenuBar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxMenuBar_GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMenuBar_GetFrame(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxListEvent_GetCacheFrom(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetCacheFrom(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxListEvent_GetCacheTo(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListEvent_GetCacheTo(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxListCtrl_AssignImageList(_obj: *u8 /* void* */, images: *u8 /* void* */, which: c_int /* int */) {
        unsafe {
            wxListCtrl_AssignImageList(_obj, images, which)
        }
    }
    #[fixed_stack_segment]
    fn wxListCtrl_GetColumn2(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) {
        unsafe {
            wxListCtrl_GetColumn2(_obj, col, item)
        }
    }
    #[fixed_stack_segment]
    fn wxListCtrl_GetItem2(_obj: *u8 /* void* */, info: *u8 /* void* */) {
        unsafe {
            wxListCtrl_GetItem2(_obj, info)
        }
    }
    #[fixed_stack_segment]
    fn wxListCtrl_GetItemPosition2(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListCtrl_GetItemPosition2(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn wxListCtrl_SortItems2(_obj: *u8 /* void* */, closure: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxListCtrl_SortItems2(_obj, closure)
        }
    }
}
trait wxURL {
}
trait wxStyledTextCtrl {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, style)
        }
    }
}
trait wxHtmlContainerCell {
}
trait wxFrameLayout {
    #[fixed_stack_segment]
    fn Activate(_obj: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_Activate(_obj)
        }
    }
    #[fixed_stack_segment]
    fn AddBar(_obj: *u8 /* void* */, pBarWnd: *u8 /* void* */, dimInfo: *u8 /* void* */, alignment: c_int /* int */, rowNo: c_int /* int */, columnPos: c_int /* int */, name: *wchar_t /* wchar_t* */, spyEvents: c_int /* int */, state: c_int /* int */) {
        unsafe {
            wxFrameLayout_AddBar(_obj, pBarWnd, dimInfo, alignment, rowNo, columnPos, name, spyEvents, state)
        }
    }
    #[fixed_stack_segment]
    fn AddPlugin(_obj: *u8 /* void* */, pPlInfo: *u8 /* void* */, paneMask: c_int /* int */) {
        unsafe {
            wxFrameLayout_AddPlugin(_obj, pPlInfo, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn AddPluginBefore(_obj: *u8 /* void* */, pNextPlInfo: *u8 /* void* */, pPlInfo: *u8 /* void* */, paneMask: c_int /* int */) {
        unsafe {
            wxFrameLayout_AddPluginBefore(_obj, pNextPlInfo, pPlInfo, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn ApplyBarProperties(_obj: *u8 /* void* */, pBar: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_ApplyBarProperties(_obj, pBar)
        }
    }
    #[fixed_stack_segment]
    fn CaptureEventsForPane(_obj: *u8 /* void* */, toPane: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_CaptureEventsForPane(_obj, toPane)
        }
    }
    #[fixed_stack_segment]
    fn CaptureEventsForPlugin(_obj: *u8 /* void* */, pPlugin: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_CaptureEventsForPlugin(_obj, pPlugin)
        }
    }
    #[fixed_stack_segment]
    fn Create(pParentFrame: *u8 /* void* */, pFrameClient: *u8 /* void* */, activateNow: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFrameLayout_Create(pParentFrame, pFrameClient, activateNow)
        }
    }
    #[fixed_stack_segment]
    fn Deactivate(_obj: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_Deactivate(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DestroyBarWindows(_obj: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_DestroyBarWindows(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EnableFloating(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxFrameLayout_EnableFloating(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn FindBarByName(_obj: *u8 /* void* */, name: *wchar_t /* wchar_t* */) -> *u8 /* void* */ {
        unsafe {
            wxFrameLayout_FindBarByName(_obj, name)
        }
    }
    #[fixed_stack_segment]
    fn FindBarByWindow(_obj: *u8 /* void* */, pWnd: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrameLayout_FindBarByWindow(_obj, pWnd)
        }
    }
    #[fixed_stack_segment]
    fn FindPlugin(_obj: *u8 /* void* */, pPlInfo: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrameLayout_FindPlugin(_obj, pPlInfo)
        }
    }
    #[fixed_stack_segment]
    fn FirePluginEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_FirePluginEvent(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn GetBars(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFrameLayout_GetBars(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetClientHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFrameLayout_GetClientHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetClientRect(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxFrameLayout_GetClientRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn GetClientWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFrameLayout_GetClientWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFrameClient(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrameLayout_GetFrameClient(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPane(_obj: *u8 /* void* */, alignment: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFrameLayout_GetPane(_obj, alignment)
        }
    }
    #[fixed_stack_segment]
    fn GetPaneProperties(_obj: *u8 /* void* */, props: *u8 /* void* */, alignment: c_int /* int */) {
        unsafe {
            wxFrameLayout_GetPaneProperties(_obj, props, alignment)
        }
    }
    #[fixed_stack_segment]
    fn GetParentFrame(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrameLayout_GetParentFrame(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTopPlugin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrameLayout_GetTopPlugin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetUpdatesManager(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFrameLayout_GetUpdatesManager(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasTopPlugin(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFrameLayout_HasTopPlugin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HideBarWindows(_obj: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_HideBarWindows(_obj)
        }
    }
    #[fixed_stack_segment]
    fn InverseVisibility(_obj: *u8 /* void* */, pBar: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_InverseVisibility(_obj, pBar)
        }
    }
    #[fixed_stack_segment]
    fn OnLButtonDown(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_OnLButtonDown(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn OnLButtonUp(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_OnLButtonUp(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn OnLDblClick(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_OnLDblClick(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn OnMouseMove(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_OnMouseMove(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn OnRButtonDown(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_OnRButtonDown(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn OnRButtonUp(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_OnRButtonUp(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn OnSize(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_OnSize(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn PopAllPlugins(_obj: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_PopAllPlugins(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PopPlugin(_obj: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_PopPlugin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PushDefaultPlugins(_obj: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_PushDefaultPlugins(_obj)
        }
    }
    #[fixed_stack_segment]
    fn PushPlugin(_obj: *u8 /* void* */, pPugin: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_PushPlugin(_obj, pPugin)
        }
    }
    #[fixed_stack_segment]
    fn RecalcLayout(_obj: *u8 /* void* */, repositionBarsNow: c_int /* int */) {
        unsafe {
            wxFrameLayout_RecalcLayout(_obj, repositionBarsNow)
        }
    }
    #[fixed_stack_segment]
    fn RedockBar(_obj: *u8 /* void* */, pBar: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, pToPane: *u8 /* void* */, updateNow: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxFrameLayout_RedockBar(_obj, pBar, arg0, arg1, arg2, arg3, pToPane, updateNow)
        }
    }
    #[fixed_stack_segment]
    fn RefreshNow(_obj: *u8 /* void* */, recalcLayout: c_int /* int */) {
        unsafe {
            wxFrameLayout_RefreshNow(_obj, recalcLayout)
        }
    }
    #[fixed_stack_segment]
    fn ReleaseEventsFromPane(_obj: *u8 /* void* */, fromPane: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_ReleaseEventsFromPane(_obj, fromPane)
        }
    }
    #[fixed_stack_segment]
    fn ReleaseEventsFromPlugin(_obj: *u8 /* void* */, pPlugin: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_ReleaseEventsFromPlugin(_obj, pPlugin)
        }
    }
    #[fixed_stack_segment]
    fn RemoveBar(_obj: *u8 /* void* */, pBar: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_RemoveBar(_obj, pBar)
        }
    }
    #[fixed_stack_segment]
    fn RemovePlugin(_obj: *u8 /* void* */, pPlInfo: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_RemovePlugin(_obj, pPlInfo)
        }
    }
    #[fixed_stack_segment]
    fn SetBarState(_obj: *u8 /* void* */, pBar: *u8 /* void* */, newStatem: c_int /* int */, updateNow: c_int /* int */) {
        unsafe {
            wxFrameLayout_SetBarState(_obj, pBar, newStatem, updateNow)
        }
    }
    #[fixed_stack_segment]
    fn SetFrameClient(_obj: *u8 /* void* */, pFrameClient: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_SetFrameClient(_obj, pFrameClient)
        }
    }
    #[fixed_stack_segment]
    fn SetMargins(_obj: *u8 /* void* */, top: c_int /* int */, bottom: c_int /* int */, left: c_int /* int */, right: c_int /* int */, paneMask: c_int /* int */) {
        unsafe {
            wxFrameLayout_SetMargins(_obj, top, bottom, left, right, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn SetPaneBackground(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_SetPaneBackground(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetPaneProperties(_obj: *u8 /* void* */, props: *u8 /* void* */, paneMask: c_int /* int */) {
        unsafe {
            wxFrameLayout_SetPaneProperties(_obj, props, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn SetTopPlugin(_obj: *u8 /* void* */, pPlugin: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_SetTopPlugin(_obj, pPlugin)
        }
    }
    #[fixed_stack_segment]
    fn SetUpdatesManager(_obj: *u8 /* void* */, pUMgr: *u8 /* void* */) {
        unsafe {
            wxFrameLayout_SetUpdatesManager(_obj, pUMgr)
        }
    }
}
trait wxDDEServer {
}
trait cbDrawRowDecorEvent {
    #[fixed_stack_segment]
    fn Dc(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawRowDecorEvent_Dc(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Row(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawRowDecorEvent_Row(_obj)
        }
    }
}
trait wxcTreeItemData {
    #[fixed_stack_segment]
    fn Create(closure: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcTreeItemData_Create(closure)
        }
    }
    #[fixed_stack_segment]
    fn GetClientClosure(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxcTreeItemData_GetClientClosure(self_)
        }
    }
    #[fixed_stack_segment]
    fn SetClientClosure(self_: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxcTreeItemData_SetClientClosure(self_, closure)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeItemId_Clone(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeItemId_Clone(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeItemId_CreateFromValue(value: intptr_t /* intptr_t */) -> *u8 /* void* */ {
        unsafe {
            wxTreeItemId_CreateFromValue(value)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeItemId_GetValue(_obj: *u8 /* void* */) -> intptr_t /* intptr_t */ {
        unsafe {
            wxTreeItemId_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeEvent_GetKeyEvent(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeEvent_GetKeyEvent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeEvent_IsEditCancelled(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeEvent_IsEditCancelled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeEvent_Allow(_obj: *u8 /* void* */) {
        unsafe {
            wxTreeEvent_Allow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeCtrl_Create2(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_Create2(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeCtrl_InsertItem2(_obj: *u8 /* void* */, parent: *u8 /* void* */, idPrevious: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, closure: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_InsertItem2(_obj, parent, idPrevious, text, image, selectedImage, closure, _item)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeCtrl_InsertItemByIndex2(_obj: *u8 /* void* */, parent: *u8 /* void* */, index: c_int /* int */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, closure: *u8 /* void* */, _item: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_InsertItemByIndex2(_obj, parent, index, text, image, selectedImage, closure, _item)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeCtrl_GetItemClientClosure(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeCtrl_GetItemClientClosure(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeCtrl_SetItemClientClosure(_obj: *u8 /* void* */, item: *u8 /* void* */, closure: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_SetItemClientClosure(_obj, item, closure)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeCtrl_AssignImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_AssignImageList(_obj, imageList)
        }
    }
    #[fixed_stack_segment]
    fn wxTreeCtrl_AssignStateImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */) {
        unsafe {
            wxTreeCtrl_AssignStateImageList(_obj, imageList)
        }
    }
    #[fixed_stack_segment]
    fn wxDC_GetPixel2(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */) {
        unsafe {
            wxDC_GetPixel2(_obj, arg0, arg1, col)
        }
    }
    #[fixed_stack_segment]
    fn wxScrolledWindow_SetScrollRate(_obj: *u8 /* void* */, xstep: c_int /* int */, ystep: c_int /* int */) {
        unsafe {
            wxScrolledWindow_SetScrollRate(_obj, xstep, ystep)
        }
    }
}
trait wxEvtHandler {
    #[fixed_stack_segment]
    fn AddPendingEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_AddPendingEvent(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn Connect(_obj: *u8 /* void* */, first: c_int /* int */, last: c_int /* int */, type_: c_int /* int */, data: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEvtHandler_Connect(_obj, first, last, type_, data)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Disconnect(_obj: *u8 /* void* */, first: c_int /* int */, last: c_int /* int */, type_: c_int /* int */, id: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxEvtHandler_Disconnect(_obj, first, last, type_, id)
        }
    }
    #[fixed_stack_segment]
    fn GetEvtHandlerEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxEvtHandler_GetEvtHandlerEnabled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetNextHandler(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_GetNextHandler(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPreviousHandler(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEvtHandler_GetPreviousHandler(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ProcessEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxEvtHandler_ProcessEvent(_obj, event)
        }
    }
    #[fixed_stack_segment]
    fn ProcessPendingEvents(_obj: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_ProcessPendingEvents(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetEvtHandlerEnabled(_obj: *u8 /* void* */, enabled: bool /* bool */) {
        unsafe {
            wxEvtHandler_SetEvtHandlerEnabled(_obj, enabled)
        }
    }
    #[fixed_stack_segment]
    fn SetNextHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_SetNextHandler(_obj, handler)
        }
    }
    #[fixed_stack_segment]
    fn SetPreviousHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */) {
        unsafe {
            wxEvtHandler_SetPreviousHandler(_obj, handler)
        }
    }
}
trait wxSpinButton {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */ {
        unsafe {
            wxSpinButton_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinButton_GetMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinButton_GetMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSpinButton_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetRange(_obj: *u8 /* void* */, minVal: c_int /* int */, maxVal: c_int /* int */) {
        unsafe {
            wxSpinButton_SetRange(_obj, minVal, maxVal)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, val: c_int /* int */) {
        unsafe {
            wxSpinButton_SetValue(_obj, val)
        }
    }
}
trait ELJDropTarget {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJDropTarget_Create(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnData(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnDragOver(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnDrop(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnEnter(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJDropTarget_SetOnLeave(_obj, _func)
        }
    }
}
trait wxFindReplaceDialog {
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, data: *u8 /* void* */, title: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceDialog_Create(parent, data, title, style)
        }
    }
    #[fixed_stack_segment]
    fn GetData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxFindReplaceDialog_GetData(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxShowEvent_CopyObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn IsShown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxShowEvent_IsShown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetShow(_obj: *u8 /* void* */, show: bool /* bool */) {
        unsafe {
            wxShowEvent_SetShow(_obj, show)
        }
    }
}
trait cbResizeBarEvent {
    #[fixed_stack_segment]
    fn Bar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbResizeBarEvent_Bar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Row(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbResizeBarEvent_Row(_obj)
        }
    }
}
trait wxHtmlTag {
}
trait wxMemoryDC {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxMemoryDC_Create()
        }
    }
    #[fixed_stack_segment]
    fn CreateCompatible(dc: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMemoryDC_CreateCompatible(dc)
        }
    }
    #[fixed_stack_segment]
    fn CreateWithBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxMemoryDC_CreateWithBitmap(bitmap)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMemoryDC_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, pen: *u8 /* void* */) {
        unsafe {
            wxPen_Assign(_obj, pen)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxPen_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromBitmap(stipple: *u8 /* void* */, width: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPen_CreateFromBitmap(stipple, width)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromColour(col: *u8 /* void* */, width: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPen_CreateFromColour(col, width, style)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPen_CreateFromStock(id)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPen_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCap(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetCap(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPen_GetColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetDashes(_obj: *u8 /* void* */, ptr: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetDashes(_obj, ptr)
        }
    }
    #[fixed_stack_segment]
    fn GetJoin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetJoin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStipple(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPen_GetStipple(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPen_GetWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsEqual(_obj: *u8 /* void* */, pen: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPen_IsEqual(_obj, pen)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPen_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCap(_obj: *u8 /* void* */, cap: c_int /* int */) {
        unsafe {
            wxPen_SetCap(_obj, cap)
        }
    }
    #[fixed_stack_segment]
    fn SetColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxPen_SetColour(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetColourSingle(_obj: *u8 /* void* */, r: wchar_t /* wchar_t */, g: wchar_t /* wchar_t */, b: wchar_t /* wchar_t */) {
        unsafe {
            wxPen_SetColourSingle(_obj, r, g, b)
        }
    }
    #[fixed_stack_segment]
    fn SetDashes(_obj: *u8 /* void* */, nb_dashes: c_int /* int */, dash: *u8 /* void* */) {
        unsafe {
            wxPen_SetDashes(_obj, nb_dashes, dash)
        }
    }
    #[fixed_stack_segment]
    fn SetJoin(_obj: *u8 /* void* */, join: c_int /* int */) {
        unsafe {
            wxPen_SetJoin(_obj, join)
        }
    }
    #[fixed_stack_segment]
    fn SetStipple(_obj: *u8 /* void* */, stipple: *u8 /* void* */) {
        unsafe {
            wxPen_SetStipple(_obj, stipple)
        }
    }
    #[fixed_stack_segment]
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxPen_SetStyle(_obj, style)
        }
    }
    #[fixed_stack_segment]
    fn SetWidth(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxPen_SetWidth(_obj, width)
        }
    }
}
trait wxArtProvider {
    #[fixed_stack_segment]
    fn PopProvider() -> bool /* bool */ {
        unsafe {
            PopProvider()
        }
    }
    #[fixed_stack_segment]
    fn PushProvider(provider: *u8 /* void* */) {
        unsafe {
            PushProvider(provider)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Append(_obj: *u8 /* void* */, item: *u8 /* void* */) {
        unsafe {
            wxListBox_Append(_obj, item)
        }
    }
    #[fixed_stack_segment]
    fn AppendData(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxListBox_AppendData(_obj, item, data)
        }
    }
    #[fixed_stack_segment]
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxListBox_Clear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListBox_Create(_prt, _id, arg0, arg1, arg2, arg3, arg4, arg5, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxListBox_Delete(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListBox_FindString(_obj, s)
        }
    }
    #[fixed_stack_segment]
    fn GetClientData(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListBox_GetClientData(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListBox_GetCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxListBox_GetSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelections(_obj: *u8 /* void* */, aSelections: *c_int /* int* */, allocated: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxListBox_GetSelections(_obj, aSelections, allocated)
        }
    }
    #[fixed_stack_segment]
    fn GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxListBox_GetString(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn InsertItems(_obj: *u8 /* void* */, items: *u8 /* void* */, pos: c_int /* int */, count: c_int /* int */) {
        unsafe {
            wxListBox_InsertItems(_obj, items, pos, count)
        }
    }
    #[fixed_stack_segment]
    fn IsSelected(_obj: *u8 /* void* */, n: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxListBox_IsSelected(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn SetClientData(_obj: *u8 /* void* */, n: c_int /* int */, clientData: *u8 /* void* */) {
        unsafe {
            wxListBox_SetClientData(_obj, n, clientData)
        }
    }
    #[fixed_stack_segment]
    fn SetFirstItem(_obj: *u8 /* void* */, n: c_int /* int */) {
        unsafe {
            wxListBox_SetFirstItem(_obj, n)
        }
    }
    #[fixed_stack_segment]
    fn SetSelection(_obj: *u8 /* void* */, n: c_int /* int */, select: c_int /* int */) {
        unsafe {
            wxListBox_SetSelection(_obj, n, select)
        }
    }
    #[fixed_stack_segment]
    fn SetString(_obj: *u8 /* void* */, n: c_int /* int */, s: *u8 /* void* */) {
        unsafe {
            wxListBox_SetString(_obj, n, s)
        }
    }
    #[fixed_stack_segment]
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
trait wxGraphicsPen {
}
trait wxTipWindow {
    #[fixed_stack_segment]
    fn Close(_obj: *u8 /* void* */) {
        unsafe {
            wxTipWindow_Close(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(parent: *u8 /* void* */, text: *u8 /* void* */, maxLength: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxTipWindow_Create(parent, text, maxLength)
        }
    }
    #[fixed_stack_segment]
    fn SetBoundingRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxTipWindow_SetBoundingRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn SetTipWindowPtr(_obj: *u8 /* void* */, windowPtr: *u8 /* void* */) {
        unsafe {
            wxTipWindow_SetTipWindowPtr(_obj, windowPtr)
        }
    }
}
trait cbRightDownEvent {
    #[fixed_stack_segment]
    fn Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            cbRightDownEvent_Pos(_obj, arg0, arg1)
        }
    }
}
trait wxSplitterEvent {
}
trait ELJFileDropTarget {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJFileDropTarget_Create(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnData(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnDragOver(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnDrop(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnEnter(_obj, _func)
        }
    }
    #[fixed_stack_segment]
    fn SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */) {
        unsafe {
            ELJFileDropTarget_SetOnLeave(_obj, _func)
        }
    }
}
trait wxTreeLayout {
}
trait wxMemoryBuffer {
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_IndicatorGetForeground(_obj: *u8 /* void* */, indic: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_IndicatorGetForeground(_obj, indic)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetCaretLineBackground(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_GetCaretLineBackground(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_SetCaretLineBackground(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxStyledTextCtrl_SetCaretLineBackground(_obj, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetCaretForeground(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_GetCaretForeground(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetLine(_obj: *u8 /* void* */, line: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_GetLine(_obj, line)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_GetText(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetTextRange(_obj: *u8 /* void* */, startPos: c_int /* int */, endPos: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_GetTextRange(_obj, startPos, endPos)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetSelectedText(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_GetSelectedText(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_CreateDocument(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_CreateDocument(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetEdgeColour(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_GetEdgeColour(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_GetDocPointer(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_GetDocPointer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn wxStyledTextCtrl_PointFromPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxStyledTextCtrl_PointFromPosition(_obj)
        }
    }
}
trait cbSimpleCustomizationPlugin {
    #[fixed_stack_segment]
    fn Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            cbSimpleCustomizationPlugin_Create(pPanel, paneMask)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            cbSimpleCustomizationPlugin_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbSimpleCustomizationPlugin_Delete(_obj)
        }
    }
}
trait ELJServer {
    #[fixed_stack_segment]
    fn Create(_eobj: *u8 /* void* */, _cnct: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            ELJServer_Create(_eobj, _cnct)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            ELJServer_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Initialize(_obj: *u8 /* void* */, name: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            ELJServer_Initialize(_obj, name)
        }
    }
}
trait wxMemoryFSHandler {
}
trait cbDrawBarHandlesEvent {
    #[fixed_stack_segment]
    fn Bar(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawBarHandlesEvent_Bar(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Dc(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawBarHandlesEvent_Dc(_obj)
        }
    }
}
trait wxGridCellAttr {
    #[fixed_stack_segment]
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellAttr_Ctor()
        }
    }
    #[fixed_stack_segment]
    fn DecRef(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_DecRef(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGridCellAttr_GetAlignment(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_GetBackgroundColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetEditor(_obj: *u8 /* void* */, grid: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellAttr_GetEditor(_obj, grid, row, col)
        }
    }
    #[fixed_stack_segment]
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_GetFont(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetRenderer(_obj: *u8 /* void* */, grid: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellAttr_GetRenderer(_obj, grid, row, col)
        }
    }
    #[fixed_stack_segment]
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_GetTextColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn HasAlignment(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasAlignment(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasBackgroundColour(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasEditor(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasEditor(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasFont(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasFont(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasRenderer(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasRenderer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_HasTextColour(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IncRef(_obj: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_IncRef(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsReadOnly(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridCellAttr_IsReadOnly(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetAlignment(_obj: *u8 /* void* */, hAlign: c_int /* int */, vAlign: c_int /* int */) {
        unsafe {
            wxGridCellAttr_SetAlignment(_obj, hAlign, vAlign)
        }
    }
    #[fixed_stack_segment]
    fn SetBackgroundColour(_obj: *u8 /* void* */, colBack: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetBackgroundColour(_obj, colBack)
        }
    }
    #[fixed_stack_segment]
    fn SetDefAttr(_obj: *u8 /* void* */, defAttr: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetDefAttr(_obj, defAttr)
        }
    }
    #[fixed_stack_segment]
    fn SetEditor(_obj: *u8 /* void* */, editor: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetEditor(_obj, editor)
        }
    }
    #[fixed_stack_segment]
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn SetReadOnly(_obj: *u8 /* void* */, isReadOnly: bool /* bool */) {
        unsafe {
            wxGridCellAttr_SetReadOnly(_obj, isReadOnly)
        }
    }
    #[fixed_stack_segment]
    fn SetRenderer(_obj: *u8 /* void* */, renderer: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetRenderer(_obj, renderer)
        }
    }
    #[fixed_stack_segment]
    fn SetTextColour(_obj: *u8 /* void* */, colText: *u8 /* void* */) {
        unsafe {
            wxGridCellAttr_SetTextColour(_obj, colText)
        }
    }
}
trait wxCalculateLayoutEvent {
    #[fixed_stack_segment]
    fn Create(id: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCalculateLayoutEvent_Create(id)
        }
    }
    #[fixed_stack_segment]
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCalculateLayoutEvent_GetFlags(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxCalculateLayoutEvent_GetRect(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */) {
        unsafe {
            wxCalculateLayoutEvent_SetFlags(_obj, flags)
        }
    }
    #[fixed_stack_segment]
    fn SetRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxCalculateLayoutEvent_SetRect(_obj, arg0, arg1, arg2, arg3)
        }
    }
}
trait wxBusyCursor {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxBusyCursor_Create()
        }
    }
    #[fixed_stack_segment]
    fn CreateWithCursor(_cur: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBusyCursor_CreateWithCursor(_cur)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxBusyCursor_Delete(_obj)
        }
    }
}
trait wxJoystick {
    #[fixed_stack_segment]
    fn Create(joystick: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxJoystick_Create(joystick)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxJoystick_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetButtonState(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetButtonState(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetManufacturerId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetManufacturerId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaxAxes(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetMaxAxes(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaxButtons(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetMaxButtons(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMovementThreshold(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetMovementThreshold(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberAxes(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetNumberAxes(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberButtons(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetNumberButtons(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberJoysticks(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetNumberJoysticks(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPOVCTSPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetPOVCTSPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPOVPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetPOVPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPollingMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetPollingMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPollingMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetPollingMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxJoystick_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetProductId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetProductId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetProductName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxJoystick_GetProductName(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRudderMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetRudderMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRudderMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetRudderMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRudderPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetRudderPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetUMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetUMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetUMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetUMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetUPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetUPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetVMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetVMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetVMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetVMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetVPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetVPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetXMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetXMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetXMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetXMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetYMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetYMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetYMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetYMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetZMax(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetZMax(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetZMin(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetZMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetZPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_GetZPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasPOV(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystick_HasPOV(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasPOV4Dir(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystick_HasPOV4Dir(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasPOVCTS(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystick_HasPOVCTS(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasRudder(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystick_HasRudder(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasU(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystick_HasU(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasV(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystick_HasV(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasZ(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystick_HasZ(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxJoystick_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ReleaseCapture(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxJoystick_ReleaseCapture(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCapture(_obj: *u8 /* void* */, win: *u8 /* void* */, pollingFreq: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxJoystick_SetCapture(_obj, win, pollingFreq)
        }
    }
    #[fixed_stack_segment]
    fn SetMovementThreshold(_obj: *u8 /* void* */, threshold: c_int /* int */) {
        unsafe {
            wxJoystick_SetMovementThreshold(_obj, threshold)
        }
    }
}
trait wxPrintDialogData {
    #[fixed_stack_segment]
    fn Assign(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_Assign(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn AssignData(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_AssignData(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxPrintDialogData_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromData(printData: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPrintDialogData_CreateFromData(printData)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn EnableHelp(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_EnableHelp(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn EnablePageNumbers(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_EnablePageNumbers(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn EnablePrintToFile(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_EnablePrintToFile(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn EnableSelection(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_EnableSelection(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn GetAllPages(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetAllPages(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCollate(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetCollate(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnableHelp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetEnableHelp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnablePageNumbers(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetEnablePageNumbers(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnablePrintToFile(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetEnablePrintToFile(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEnableSelection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetEnableSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetFromPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetFromPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaxPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetMaxPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMinPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetMinPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetNoCopies(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetNoCopies(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_GetPrintData(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetPrintToFile(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetPrintToFile(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSelection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPrintDialogData_GetSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetToPage(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxPrintDialogData_GetToPage(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetAllPages(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_SetAllPages(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn SetCollate(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_SetCollate(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn SetFromPage(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintDialogData_SetFromPage(_obj, v)
        }
    }
    #[fixed_stack_segment]
    fn SetMaxPage(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintDialogData_SetMaxPage(_obj, v)
        }
    }
    #[fixed_stack_segment]
    fn SetMinPage(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintDialogData_SetMinPage(_obj, v)
        }
    }
    #[fixed_stack_segment]
    fn SetNoCopies(_obj: *u8 /* void* */, v: c_int /* int */) {
        unsafe {
            wxPrintDialogData_SetNoCopies(_obj, v)
        }
    }
    #[fixed_stack_segment]
    fn SetPrintData(_obj: *u8 /* void* */, printData: *u8 /* void* */) {
        unsafe {
            wxPrintDialogData_SetPrintData(_obj, printData)
        }
    }
    #[fixed_stack_segment]
    fn SetPrintToFile(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_SetPrintToFile(_obj, flag)
        }
    }
    #[fixed_stack_segment]
    fn SetSelection(_obj: *u8 /* void* */, flag: bool /* bool */) {
        unsafe {
            wxPrintDialogData_SetSelection(_obj, flag)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn CanRead(name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_CanRead(name)
        }
    }
    #[fixed_stack_segment]
    fn ConvertToBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */) {
        unsafe {
            wxImage_ConvertToBitmap(_obj, bitmap)
        }
    }
    #[fixed_stack_segment]
    fn ConvertToByteString(_obj: *u8 /* void* */, type_: c_int /* int */, data: *char /* char* */) -> c_int /* int */ {
        unsafe {
            wxImage_ConvertToByteString(_obj, type_, data)
        }
    }
    #[fixed_stack_segment]
    fn ConvertToLazyByteString(_obj: *u8 /* void* */, type_: c_int /* int */, data: *char /* char* */) -> c_int /* int */ {
        unsafe {
            wxImage_ConvertToLazyByteString(_obj, type_, data)
        }
    }
    #[fixed_stack_segment]
    fn CountColours(_obj: *u8 /* void* */, stopafter: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxImage_CountColours(_obj, stopafter)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromBitmap(bitmap)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromByteString(arg0: *char /* char* */, arg1: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromByteString(arg0, arg1, type_)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromLazyByteString(arg0: *char /* char* */, arg1: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromLazyByteString(arg0, arg1, type_)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromData(arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromData(arg0, arg1, data)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromFile(name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateFromFile(name)
        }
    }
    #[fixed_stack_segment]
    fn CreateSized(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxImage_CreateSized(arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Destroy(_obj: *u8 /* void* */) {
        unsafe {
            wxImage_Destroy(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBlue(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetBlue(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetData(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_GetData(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetGreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetGreen(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImage_GetHeight(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaskBlue(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetMaskBlue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaskGreen(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetMaskGreen(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMaskRed(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetMaskRed(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRed(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */ {
        unsafe {
            wxImage_GetRed(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetSubImage(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, image: *u8 /* void* */) {
        unsafe {
            wxImage_GetSubImage(_obj, arg0, arg1, arg2, arg3, image)
        }
    }
    #[fixed_stack_segment]
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxImage_GetWidth(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasMask(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_HasMask(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOption(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxImage_GetOption(_obj, name)
        }
    }
    #[fixed_stack_segment]
    fn GetOptionInt(_obj: *u8 /* void* */, name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_GetOptionInt(_obj, name)
        }
    }
    #[fixed_stack_segment]
    fn HasOption(_obj: *u8 /* void* */, name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_HasOption(_obj, name)
        }
    }
    #[fixed_stack_segment]
    fn Initialize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxImage_Initialize(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn InitializeFromData(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */) {
        unsafe {
            wxImage_InitializeFromData(_obj, arg0, arg1, data)
        }
    }
    #[fixed_stack_segment]
    fn LoadFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxImage_LoadFile(_obj, name, type_)
        }
    }
    #[fixed_stack_segment]
    fn Mirror(_obj: *u8 /* void* */, horizontally: c_int /* int */, image: *u8 /* void* */) {
        unsafe {
            wxImage_Mirror(_obj, horizontally, image)
        }
    }
    #[fixed_stack_segment]
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxImage_IsOk(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Paste(_obj: *u8 /* void* */, image: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxImage_Paste(_obj, image, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Replace(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */, arg5: u8 /* u8 */) {
        unsafe {
            wxImage_Replace(_obj, arg0, arg1, arg2, arg3, arg4, arg5)
        }
    }
    #[fixed_stack_segment]
    fn Rescale(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxImage_Rescale(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn Rotate(_obj: *u8 /* void* */, angle: c_double /* double */, arg0: c_int /* int */, arg1: c_int /* int */, interpolating: c_int /* int */, offset_after_rotation: *u8 /* void* */, image: *u8 /* void* */) {
        unsafe {
            wxImage_Rotate(_obj, angle, arg0, arg1, interpolating, offset_after_rotation, image)
        }
    }
    #[fixed_stack_segment]
    fn Rotate90(_obj: *u8 /* void* */, clockwise: c_int /* int */, image: *u8 /* void* */) {
        unsafe {
            wxImage_Rotate90(_obj, clockwise, image)
        }
    }
    #[fixed_stack_segment]
    fn SaveFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxImage_SaveFile(_obj, name, type_)
        }
    }
    #[fixed_stack_segment]
    fn Scale(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, image: *u8 /* void* */) {
        unsafe {
            wxImage_Scale(_obj, arg0, arg1, image)
        }
    }
    #[fixed_stack_segment]
    fn SetData(_obj: *u8 /* void* */, data: *u8 /* void* */) {
        unsafe {
            wxImage_SetData(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn SetDataAndSize(_obj: *u8 /* void* */, data: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) {
        unsafe {
            wxImage_SetDataAndSize(_obj, data, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn SetMask(_obj: *u8 /* void* */, mask: c_int /* int */) {
        unsafe {
            wxImage_SetMask(_obj, mask)
        }
    }
    #[fixed_stack_segment]
    fn SetMaskColour(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) {
        unsafe {
            wxImage_SetMaskColour(_obj, arg0, arg1, arg2)
        }
    }
    #[fixed_stack_segment]
    fn SetOption(_obj: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) {
        unsafe {
            wxImage_SetOption(_obj, name, value)
        }
    }
    #[fixed_stack_segment]
    fn SetOptionInt(_obj: *u8 /* void* */, name: *u8 /* void* */, value: c_int /* int */) {
        unsafe {
            wxImage_SetOptionInt(_obj, name, value)
        }
    }
    #[fixed_stack_segment]
    fn SetRGB(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */) {
        unsafe {
            wxImage_SetRGB(_obj, arg0, arg1, arg2, arg3, arg4)
        }
    }
}
trait wxFileHistory {
    #[fixed_stack_segment]
    fn AddFileToHistory(_obj: *u8 /* void* */, file: *u8 /* void* */) {
        unsafe {
            wxFileHistory_AddFileToHistory(_obj, file)
        }
    }
    #[fixed_stack_segment]
    fn AddFilesToMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxFileHistory_AddFilesToMenu(_obj, menu)
        }
    }
    #[fixed_stack_segment]
    fn Create(maxFiles: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFileHistory_Create(maxFiles)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxFileHistory_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileHistory_GetCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetHistoryFile(_obj: *u8 /* void* */, i: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxFileHistory_GetHistoryFile(_obj, i)
        }
    }
    #[fixed_stack_segment]
    fn GetMaxFiles(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileHistory_GetMaxFiles(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetMenus(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxFileHistory_GetMenus(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn Load(_obj: *u8 /* void* */, config: *u8 /* void* */) {
        unsafe {
            wxFileHistory_Load(_obj, config)
        }
    }
    #[fixed_stack_segment]
    fn RemoveFileFromHistory(_obj: *u8 /* void* */, i: c_int /* int */) {
        unsafe {
            wxFileHistory_RemoveFileFromHistory(_obj, i)
        }
    }
    #[fixed_stack_segment]
    fn RemoveMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxFileHistory_RemoveMenu(_obj, menu)
        }
    }
    #[fixed_stack_segment]
    fn Save(_obj: *u8 /* void* */, config: *u8 /* void* */) {
        unsafe {
            wxFileHistory_Save(_obj, config)
        }
    }
    #[fixed_stack_segment]
    fn UseMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) {
        unsafe {
            wxFileHistory_UseMenu(_obj, menu)
        }
    }
}
trait wxGraphicsMatrix {
}
trait wxLogNull {
}
trait wxNewBitmapButton {
    #[fixed_stack_segment]
    fn Create(labelBitmap: *u8 /* void* */, labelText: *u8 /* void* */, alignText: c_int /* int */, isFlat: bool /* bool */, firedEventType: c_int /* int */, marginX: c_int /* int */, marginY: c_int /* int */, textToLabelGap: c_int /* int */, isSticky: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxNewBitmapButton_Create(labelBitmap, labelText, alignText, isFlat, firedEventType, marginX, marginY, textToLabelGap, isSticky)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromFile(bitmapFileName: *u8 /* void* */, bitmapFileType: c_int /* int */, labelText: *u8 /* void* */, alignText: c_int /* int */, isFlat: bool /* bool */, firedEventType: c_int /* int */, marginX: c_int /* int */, marginY: c_int /* int */, textToLabelGap: c_int /* int */, isSticky: bool /* bool */) -> *u8 /* void* */ {
        unsafe {
            wxNewBitmapButton_CreateFromFile(bitmapFileName, bitmapFileType, labelText, alignText, isFlat, firedEventType, marginX, marginY, textToLabelGap, isSticky)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxNewBitmapButton_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DrawDecorations(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxNewBitmapButton_DrawDecorations(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn DrawLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxNewBitmapButton_DrawLabel(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxNewBitmapButton_Enable(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn Realize(_obj: *u8 /* void* */, _prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) {
        unsafe {
            wxNewBitmapButton_Realize(_obj, _prt, _id, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn RenderAllLabelImages(_obj: *u8 /* void* */) {
        unsafe {
            wxNewBitmapButton_RenderAllLabelImages(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RenderLabelImage(_obj: *u8 /* void* */, destBmp: *u8 /* void* */, srcBmp: *u8 /* void* */, isEnabled: bool /* bool */, isPressed: bool /* bool */) {
        unsafe {
            wxNewBitmapButton_RenderLabelImage(_obj, destBmp, srcBmp, isEnabled, isPressed)
        }
    }
    #[fixed_stack_segment]
    fn RenderLabelImages(_obj: *u8 /* void* */) {
        unsafe {
            wxNewBitmapButton_RenderLabelImages(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Reshape(_obj: *u8 /* void* */) {
        unsafe {
            wxNewBitmapButton_Reshape(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetAlignments(_obj: *u8 /* void* */, alignText: c_int /* int */, marginX: c_int /* int */, marginY: c_int /* int */, textToLabelGap: c_int /* int */) {
        unsafe {
            wxNewBitmapButton_SetAlignments(_obj, alignText, marginX, marginY, textToLabelGap)
        }
    }
    #[fixed_stack_segment]
    fn SetLabel(_obj: *u8 /* void* */, labelBitmap: *u8 /* void* */, labelText: *u8 /* void* */) {
        unsafe {
            wxNewBitmapButton_SetLabel(_obj, labelBitmap, labelText)
        }
    }
}
trait wxEvent {
    #[fixed_stack_segment]
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */) {
        unsafe {
            wxEvent_CopyObject(_obj, object_dest)
        }
    }
    #[fixed_stack_segment]
    fn GetEventObject(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxEvent_GetEventObject(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEventType(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEvent_GetEventType(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEvent_GetId(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSkipped(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxEvent_GetSkipped(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTimestamp(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxEvent_GetTimestamp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsCommandEvent(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxEvent_IsCommandEvent(_obj)
        }
    }
    #[fixed_stack_segment]
    fn NewEventType() -> c_int /* int */ {
        unsafe {
            wxEvent_NewEventType()
        }
    }
    #[fixed_stack_segment]
    fn SetEventObject(_obj: *u8 /* void* */, obj: *u8 /* void* */) {
        unsafe {
            wxEvent_SetEventObject(_obj, obj)
        }
    }
    #[fixed_stack_segment]
    fn SetEventType(_obj: *u8 /* void* */, typ: c_int /* int */) {
        unsafe {
            wxEvent_SetEventType(_obj, typ)
        }
    }
    #[fixed_stack_segment]
    fn SetId(_obj: *u8 /* void* */, Id: c_int /* int */) {
        unsafe {
            wxEvent_SetId(_obj, Id)
        }
    }
    #[fixed_stack_segment]
    fn SetTimestamp(_obj: *u8 /* void* */, ts: c_int /* int */) {
        unsafe {
            wxEvent_SetTimestamp(_obj, ts)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_AltDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_ControlDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridEvent_GetCol(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGridEvent_GetPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGridEvent_GetRow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_MetaDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Selecting(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_Selecting(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGridEvent_ShiftDown(_obj)
        }
    }
}
trait wxFFileInputStream {
}
trait wxClipboard {
    #[fixed_stack_segment]
    fn AddData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_AddData(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn Clear(_obj: *u8 /* void* */) {
        unsafe {
            wxClipboard_Clear(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Close(_obj: *u8 /* void* */) {
        unsafe {
            wxClipboard_Close(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxClipboard_Create()
        }
    }
    #[fixed_stack_segment]
    fn Flush(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_Flush(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_GetData(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn IsOpened(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_IsOpened(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsSupported(_obj: *u8 /* void* */, format: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_IsSupported(_obj, format)
        }
    }
    #[fixed_stack_segment]
    fn Open(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_Open(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClipboard_SetData(_obj, data)
        }
    }
    #[fixed_stack_segment]
    fn UsePrimarySelection(_obj: *u8 /* void* */, primary: bool /* bool */) {
        unsafe {
            wxClipboard_UsePrimarySelection(_obj, primary)
        }
    }
}
trait wxStaticBox {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxStaticBox_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
}
trait wxMutex {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxMutex_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxMutex_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsLocked(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxMutex_IsLocked(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Lock(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMutex_Lock(_obj)
        }
    }
    #[fixed_stack_segment]
    fn TryLock(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMutex_TryLock(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Unlock(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxMutex_Unlock(_obj)
        }
    }
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
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxConfigBase_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DeleteAll(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_DeleteAll(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DeleteEntry(_obj: *u8 /* void* */, key: *u8 /* void* */, bDeleteGroupIfEmpty: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_DeleteEntry(_obj, key, bDeleteGroupIfEmpty)
        }
    }
    #[fixed_stack_segment]
    fn DeleteGroup(_obj: *u8 /* void* */, key: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_DeleteGroup(_obj, key)
        }
    }
    #[fixed_stack_segment]
    fn Exists(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_Exists(_obj, strName)
        }
    }
    #[fixed_stack_segment]
    fn ExpandEnvVars(_obj: *u8 /* void* */, str: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_ExpandEnvVars(_obj, str)
        }
    }
    #[fixed_stack_segment]
    fn Flush(_obj: *u8 /* void* */, bCurrentOnly: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_Flush(_obj, bCurrentOnly)
        }
    }
    #[fixed_stack_segment]
    fn GetAppName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetAppName(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetEntryType(_obj: *u8 /* void* */, name: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_GetEntryType(_obj, name)
        }
    }
    #[fixed_stack_segment]
    fn GetFirstEntry(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetFirstEntry(_obj, lIndex)
        }
    }
    #[fixed_stack_segment]
    fn GetFirstGroup(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetFirstGroup(_obj, lIndex)
        }
    }
    #[fixed_stack_segment]
    fn GetNextEntry(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetNextEntry(_obj, lIndex)
        }
    }
    #[fixed_stack_segment]
    fn GetNextGroup(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetNextGroup(_obj, lIndex)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberOfEntries(_obj: *u8 /* void* */, bRecursive: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_GetNumberOfEntries(_obj, bRecursive)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberOfGroups(_obj: *u8 /* void* */, bRecursive: bool /* bool */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_GetNumberOfGroups(_obj, bRecursive)
        }
    }
    #[fixed_stack_segment]
    fn GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetPath(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_GetStyle(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetVendorName(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_GetVendorName(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasEntry(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_HasEntry(_obj, strName)
        }
    }
    #[fixed_stack_segment]
    fn HasGroup(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_HasGroup(_obj, strName)
        }
    }
    #[fixed_stack_segment]
    fn IsExpandingEnvVars(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_IsExpandingEnvVars(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsRecordingDefaults(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_IsRecordingDefaults(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ReadBool(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_ReadBool(_obj, key, defVal)
        }
    }
    #[fixed_stack_segment]
    fn ReadDouble(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: c_double /* double */) -> c_double /* double */ {
        unsafe {
            wxConfigBase_ReadDouble(_obj, key, defVal)
        }
    }
    #[fixed_stack_segment]
    fn ReadInteger(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxConfigBase_ReadInteger(_obj, key, defVal)
        }
    }
    #[fixed_stack_segment]
    fn ReadString(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxConfigBase_ReadString(_obj, key, defVal)
        }
    }
    #[fixed_stack_segment]
    fn RenameEntry(_obj: *u8 /* void* */, oldName: *u8 /* void* */, newName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_RenameEntry(_obj, oldName, newName)
        }
    }
    #[fixed_stack_segment]
    fn RenameGroup(_obj: *u8 /* void* */, oldName: *u8 /* void* */, newName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_RenameGroup(_obj, oldName, newName)
        }
    }
    #[fixed_stack_segment]
    fn SetAppName(_obj: *u8 /* void* */, appName: *u8 /* void* */) {
        unsafe {
            wxConfigBase_SetAppName(_obj, appName)
        }
    }
    #[fixed_stack_segment]
    fn SetExpandEnvVars(_obj: *u8 /* void* */, bDoIt: bool /* bool */) {
        unsafe {
            wxConfigBase_SetExpandEnvVars(_obj, bDoIt)
        }
    }
    #[fixed_stack_segment]
    fn SetPath(_obj: *u8 /* void* */, strPath: *u8 /* void* */) {
        unsafe {
            wxConfigBase_SetPath(_obj, strPath)
        }
    }
    #[fixed_stack_segment]
    fn SetRecordDefaults(_obj: *u8 /* void* */, bDoIt: bool /* bool */) {
        unsafe {
            wxConfigBase_SetRecordDefaults(_obj, bDoIt)
        }
    }
    #[fixed_stack_segment]
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */) {
        unsafe {
            wxConfigBase_SetStyle(_obj, style)
        }
    }
    #[fixed_stack_segment]
    fn SetVendorName(_obj: *u8 /* void* */, vendorName: *u8 /* void* */) {
        unsafe {
            wxConfigBase_SetVendorName(_obj, vendorName)
        }
    }
    #[fixed_stack_segment]
    fn WriteBool(_obj: *u8 /* void* */, key: *u8 /* void* */, value: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteBool(_obj, key, value)
        }
    }
    #[fixed_stack_segment]
    fn WriteDouble(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_double /* double */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteDouble(_obj, key, value)
        }
    }
    #[fixed_stack_segment]
    fn WriteInteger(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteInteger(_obj, key, value)
        }
    }
    #[fixed_stack_segment]
    fn WriteLong(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_long /* long */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteLong(_obj, key, value)
        }
    }
    #[fixed_stack_segment]
    fn WriteString(_obj: *u8 /* void* */, key: *u8 /* void* */, value: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxConfigBase_WriteString(_obj, key, value)
        }
    }
}
trait wxDataInputStream {
}
trait wxGridCellFloatEditor {
    #[fixed_stack_segment]
    fn Ctor(width: c_int /* int */, precision: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGridCellFloatEditor_Ctor(width, precision)
        }
    }
}
trait wxServer {
}
trait wxScrollBar {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxScrollBar_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn GetPageSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollBar_GetPageSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRange(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollBar_GetRange(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetThumbPosition(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollBar_GetThumbPosition(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetThumbSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxScrollBar_GetThumbSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetScrollbar(_obj: *u8 /* void* */, position: c_int /* int */, thumbSize: c_int /* int */, range: c_int /* int */, pageSize: c_int /* int */, refresh: bool /* bool */) {
        unsafe {
            wxScrollBar_SetScrollbar(_obj, position, thumbSize, range, pageSize, refresh)
        }
    }
    #[fixed_stack_segment]
    fn SetThumbPosition(_obj: *u8 /* void* */, viewStart: c_int /* int */) {
        unsafe {
            wxScrollBar_SetThumbPosition(_obj, viewStart)
        }
    }
}
trait wxHtmlLinkInfo {
}
trait wxSimpleHelpProvider {
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            wxSimpleHelpProvider_Create()
        }
    }
}
trait wxGrid {
    #[fixed_stack_segment]
    fn AppendCols(_obj: *u8 /* void* */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_AppendCols(_obj, numCols, updateLabels)
        }
    }
    #[fixed_stack_segment]
    fn AppendRows(_obj: *u8 /* void* */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_AppendRows(_obj, numRows, updateLabels)
        }
    }
    #[fixed_stack_segment]
    fn AutoSize(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_AutoSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn AutoSizeColumn(_obj: *u8 /* void* */, col: c_int /* int */, setAsMin: c_int /* int */) {
        unsafe {
            wxGrid_AutoSizeColumn(_obj, col, setAsMin)
        }
    }
    #[fixed_stack_segment]
    fn AutoSizeColumns(_obj: *u8 /* void* */, setAsMin: c_int /* int */) {
        unsafe {
            wxGrid_AutoSizeColumns(_obj, setAsMin)
        }
    }
    #[fixed_stack_segment]
    fn AutoSizeRow(_obj: *u8 /* void* */, row: c_int /* int */, setAsMin: c_int /* int */) {
        unsafe {
            wxGrid_AutoSizeRow(_obj, row, setAsMin)
        }
    }
    #[fixed_stack_segment]
    fn AutoSizeRows(_obj: *u8 /* void* */, setAsMin: c_int /* int */) {
        unsafe {
            wxGrid_AutoSizeRows(_obj, setAsMin)
        }
    }
    #[fixed_stack_segment]
    fn BeginBatch(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_BeginBatch(_obj)
        }
    }
    #[fixed_stack_segment]
    fn BlockToDeviceRect(_obj: *u8 /* void* */, top: c_int /* int */, left: c_int /* int */, bottom: c_int /* int */, right: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_BlockToDeviceRect(_obj, top, left, bottom, right)
        }
    }
    #[fixed_stack_segment]
    fn CanDragColSize(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_CanDragColSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CanDragGridSize(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_CanDragGridSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CanDragRowSize(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_CanDragRowSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CanEnableCellControl(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_CanEnableCellControl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn CellToRect(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_CellToRect(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn ClearGrid(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_ClearGrid(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ClearSelection(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_ClearSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn CreateGrid(_obj: *u8 /* void* */, rows: c_int /* int */, cols: c_int /* int */, selmode: c_int /* int */) {
        unsafe {
            wxGrid_CreateGrid(_obj, rows, cols, selmode)
        }
    }
    #[fixed_stack_segment]
    fn DeleteCols(_obj: *u8 /* void* */, pos: c_int /* int */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_DeleteCols(_obj, pos, numCols, updateLabels)
        }
    }
    #[fixed_stack_segment]
    fn DeleteRows(_obj: *u8 /* void* */, pos: c_int /* int */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_DeleteRows(_obj, pos, numRows, updateLabels)
        }
    }
    #[fixed_stack_segment]
    fn DisableCellEditControl(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_DisableCellEditControl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DisableDragColSize(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_DisableDragColSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DisableDragGridSize(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_DisableDragGridSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DisableDragRowSize(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_DisableDragRowSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn DrawAllGridLines(_obj: *u8 /* void* */, dc: *u8 /* void* */, reg: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawAllGridLines(_obj, dc, reg)
        }
    }
    #[fixed_stack_segment]
    fn DrawCell(_obj: *u8 /* void* */, dc: *u8 /* void* */, _row: c_int /* int */, _col: c_int /* int */) {
        unsafe {
            wxGrid_DrawCell(_obj, dc, _row, _col)
        }
    }
    #[fixed_stack_segment]
    fn DrawCellBorder(_obj: *u8 /* void* */, dc: *u8 /* void* */, _row: c_int /* int */, _col: c_int /* int */) {
        unsafe {
            wxGrid_DrawCellBorder(_obj, dc, _row, _col)
        }
    }
    #[fixed_stack_segment]
    fn DrawCellHighlight(_obj: *u8 /* void* */, dc: *u8 /* void* */, attr: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawCellHighlight(_obj, dc, attr)
        }
    }
    #[fixed_stack_segment]
    fn DrawColLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxGrid_DrawColLabel(_obj, dc, col)
        }
    }
    #[fixed_stack_segment]
    fn DrawColLabels(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawColLabels(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn DrawGridSpace(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawGridSpace(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn DrawRowLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */, row: c_int /* int */) {
        unsafe {
            wxGrid_DrawRowLabel(_obj, dc, row)
        }
    }
    #[fixed_stack_segment]
    fn DrawRowLabels(_obj: *u8 /* void* */, dc: *u8 /* void* */) {
        unsafe {
            wxGrid_DrawRowLabels(_obj, dc)
        }
    }
    #[fixed_stack_segment]
    fn DrawTextRectangle(_obj: *u8 /* void* */, dc: *u8 /* void* */, txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, horizontalAlignment: c_int /* int */, verticalAlignment: c_int /* int */) {
        unsafe {
            wxGrid_DrawTextRectangle(_obj, dc, txt, arg0, arg1, arg2, arg3, horizontalAlignment, verticalAlignment)
        }
    }
    #[fixed_stack_segment]
    fn EnableCellEditControl(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableCellEditControl(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn EnableDragColSize(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableDragColSize(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn EnableDragGridSize(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableDragGridSize(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn EnableDragRowSize(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableDragRowSize(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn EnableEditing(_obj: *u8 /* void* */, edit: c_int /* int */) {
        unsafe {
            wxGrid_EnableEditing(_obj, edit)
        }
    }
    #[fixed_stack_segment]
    fn EnableGridLines(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxGrid_EnableGridLines(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn EndBatch(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_EndBatch(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBatchCount(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetBatchCount(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetCellAlignment(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetCellAlignment(_obj, row, col, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetCellBackgroundColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_GetCellBackgroundColour(_obj, row, col, colour)
        }
    }
    #[fixed_stack_segment]
    fn GetCellEditor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetCellEditor(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn GetCellFont(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, font: *u8 /* void* */) {
        unsafe {
            wxGrid_GetCellFont(_obj, row, col, font)
        }
    }
    #[fixed_stack_segment]
    fn GetCellHighlightColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetCellHighlightColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetCellRenderer(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetCellRenderer(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn GetCellTextColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_GetCellTextColour(_obj, row, col, colour)
        }
    }
    #[fixed_stack_segment]
    fn GetCellValue(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetCellValue(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn GetColLabelAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetColLabelAlignment(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetColLabelSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetColLabelSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetColLabelValue(_obj: *u8 /* void* */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetColLabelValue(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn GetColSize(_obj: *u8 /* void* */, col: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetColSize(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultCellAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetDefaultCellAlignment(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultCellBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetDefaultCellBackgroundColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultCellFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetDefaultCellFont(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultCellTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetDefaultCellTextColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultColLabelSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetDefaultColLabelSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultColSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetDefaultColSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultEditor(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultEditor(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultEditorForCell(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultEditorForCell(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultEditorForType(_obj: *u8 /* void* */, typeName: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultEditorForType(_obj, typeName)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultRenderer(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultRenderer(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultRendererForCell(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultRendererForCell(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultRendererForType(_obj: *u8 /* void* */, typeName: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetDefaultRendererForType(_obj, typeName)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultRowLabelSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetDefaultRowLabelSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetDefaultRowSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetDefaultRowSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetGridCursorCol(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetGridCursorCol(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetGridCursorRow(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetGridCursorRow(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetGridLineColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetGridLineColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetLabelBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetLabelBackgroundColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetLabelFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetLabelFont(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetLabelTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetLabelTextColour(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberCols(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetNumberCols(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetNumberRows(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetNumberRows(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRowLabelAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetRowLabelAlignment(_obj, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
    fn GetRowLabelSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetRowLabelSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetRowLabelValue(_obj: *u8 /* void* */, row: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetRowLabelValue(_obj, row)
        }
    }
    #[fixed_stack_segment]
    fn GetRowSize(_obj: *u8 /* void* */, row: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetRowSize(_obj, row)
        }
    }
    #[fixed_stack_segment]
    fn GetSelectionBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectionBackground(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetSelectionForeground(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectionForeground(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetTable(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxGrid_GetTable(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetTextBoxSize(_obj: *u8 /* void* */, dc: *u8 /* void* */, arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxGrid_GetTextBoxSize(_obj, dc, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn GridLinesEnabled(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GridLinesEnabled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HideCellEditControl(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_HideCellEditControl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn InsertCols(_obj: *u8 /* void* */, pos: c_int /* int */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_InsertCols(_obj, pos, numCols, updateLabels)
        }
    }
    #[fixed_stack_segment]
    fn InsertRows(_obj: *u8 /* void* */, pos: c_int /* int */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_InsertRows(_obj, pos, numRows, updateLabels)
        }
    }
    #[fixed_stack_segment]
    fn IsCellEditControlEnabled(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsCellEditControlEnabled(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsCellEditControlShown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsCellEditControlShown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsCurrentCellReadOnly(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsCurrentCellReadOnly(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsEditable(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsEditable(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsInSelection(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsInSelection(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn IsReadOnly(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsReadOnly(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn IsSelection(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsSelection(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsVisible(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, wholeCellVisible: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_IsVisible(_obj, row, col, wholeCellVisible)
        }
    }
    #[fixed_stack_segment]
    fn MakeCellVisible(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) {
        unsafe {
            wxGrid_MakeCellVisible(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn MoveCursorDown(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorDown(_obj, expandSelection)
        }
    }
    #[fixed_stack_segment]
    fn MoveCursorDownBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorDownBlock(_obj, expandSelection)
        }
    }
    #[fixed_stack_segment]
    fn MoveCursorLeft(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorLeft(_obj, expandSelection)
        }
    }
    #[fixed_stack_segment]
    fn MoveCursorLeftBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorLeftBlock(_obj, expandSelection)
        }
    }
    #[fixed_stack_segment]
    fn MoveCursorRight(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorRight(_obj, expandSelection)
        }
    }
    #[fixed_stack_segment]
    fn MoveCursorRightBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorRightBlock(_obj, expandSelection)
        }
    }
    #[fixed_stack_segment]
    fn MoveCursorUp(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorUp(_obj, expandSelection)
        }
    }
    #[fixed_stack_segment]
    fn MoveCursorUpBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */ {
        unsafe {
            wxGrid_MoveCursorUpBlock(_obj, expandSelection)
        }
    }
    #[fixed_stack_segment]
    fn MovePageDown(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_MovePageDown(_obj)
        }
    }
    #[fixed_stack_segment]
    fn MovePageUp(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_MovePageUp(_obj)
        }
    }
    #[fixed_stack_segment]
    fn ProcessTableMessage(_obj: *u8 /* void* */, evt: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxGrid_ProcessTableMessage(_obj, evt)
        }
    }
    #[fixed_stack_segment]
    fn RegisterDataType(_obj: *u8 /* void* */, typeName: *u8 /* void* */, renderer: *u8 /* void* */, editor: *u8 /* void* */) {
        unsafe {
            wxGrid_RegisterDataType(_obj, typeName, renderer, editor)
        }
    }
    #[fixed_stack_segment]
    fn SaveEditControlValue(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_SaveEditControlValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SelectAll(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_SelectAll(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SelectBlock(_obj: *u8 /* void* */, topRow: c_int /* int */, leftCol: c_int /* int */, bottomRow: c_int /* int */, rightCol: c_int /* int */, addToSelected: c_int /* int */) {
        unsafe {
            wxGrid_SelectBlock(_obj, topRow, leftCol, bottomRow, rightCol, addToSelected)
        }
    }
    #[fixed_stack_segment]
    fn SelectCol(_obj: *u8 /* void* */, col: c_int /* int */, addToSelected: c_int /* int */) {
        unsafe {
            wxGrid_SelectCol(_obj, col, addToSelected)
        }
    }
    #[fixed_stack_segment]
    fn SelectRow(_obj: *u8 /* void* */, row: c_int /* int */, addToSelected: c_int /* int */) {
        unsafe {
            wxGrid_SelectRow(_obj, row, addToSelected)
        }
    }
    #[fixed_stack_segment]
    fn SetCellAlignment(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, horiz: c_int /* int */, vert: c_int /* int */) {
        unsafe {
            wxGrid_SetCellAlignment(_obj, row, col, horiz, vert)
        }
    }
    #[fixed_stack_segment]
    fn SetCellBackgroundColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellBackgroundColour(_obj, row, col, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetCellEditor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, editor: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellEditor(_obj, row, col, editor)
        }
    }
    #[fixed_stack_segment]
    fn SetCellFont(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, font: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellFont(_obj, row, col, font)
        }
    }
    #[fixed_stack_segment]
    fn SetCellHighlightColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellHighlightColour(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetCellRenderer(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, renderer: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellRenderer(_obj, row, col, renderer)
        }
    }
    #[fixed_stack_segment]
    fn SetCellTextColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellTextColour(_obj, row, col, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetCellValue(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, s: *u8 /* void* */) {
        unsafe {
            wxGrid_SetCellValue(_obj, row, col, s)
        }
    }
    #[fixed_stack_segment]
    fn SetColAttr(_obj: *u8 /* void* */, col: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxGrid_SetColAttr(_obj, col, attr)
        }
    }
    #[fixed_stack_segment]
    fn SetColFormatBool(_obj: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxGrid_SetColFormatBool(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetColFormatCustom(_obj: *u8 /* void* */, col: c_int /* int */, typeName: *u8 /* void* */) {
        unsafe {
            wxGrid_SetColFormatCustom(_obj, col, typeName)
        }
    }
    #[fixed_stack_segment]
    fn SetColFormatFloat(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */, precision: c_int /* int */) {
        unsafe {
            wxGrid_SetColFormatFloat(_obj, col, width, precision)
        }
    }
    #[fixed_stack_segment]
    fn SetColFormatNumber(_obj: *u8 /* void* */, col: c_int /* int */) {
        unsafe {
            wxGrid_SetColFormatNumber(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetColLabelAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */) {
        unsafe {
            wxGrid_SetColLabelAlignment(_obj, horiz, vert)
        }
    }
    #[fixed_stack_segment]
    fn SetColLabelSize(_obj: *u8 /* void* */, height: c_int /* int */) {
        unsafe {
            wxGrid_SetColLabelSize(_obj, height)
        }
    }
    #[fixed_stack_segment]
    fn SetColLabelValue(_obj: *u8 /* void* */, col: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxGrid_SetColLabelValue(_obj, col, label)
        }
    }
    #[fixed_stack_segment]
    fn SetColMinimalWidth(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */) {
        unsafe {
            wxGrid_SetColMinimalWidth(_obj, col, width)
        }
    }
    #[fixed_stack_segment]
    fn SetColSize(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */) {
        unsafe {
            wxGrid_SetColSize(_obj, col, width)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultCellAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */) {
        unsafe {
            wxGrid_SetDefaultCellAlignment(_obj, horiz, vert)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultCellBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultCellBackgroundColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultCellFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultCellFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultCellTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultCellTextColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultColSize(_obj: *u8 /* void* */, width: c_int /* int */, resizeExistingCols: c_int /* int */) {
        unsafe {
            wxGrid_SetDefaultColSize(_obj, width, resizeExistingCols)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultEditor(_obj: *u8 /* void* */, editor: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultEditor(_obj, editor)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultRenderer(_obj: *u8 /* void* */, renderer: *u8 /* void* */) {
        unsafe {
            wxGrid_SetDefaultRenderer(_obj, renderer)
        }
    }
    #[fixed_stack_segment]
    fn SetDefaultRowSize(_obj: *u8 /* void* */, height: c_int /* int */, resizeExistingRows: c_int /* int */) {
        unsafe {
            wxGrid_SetDefaultRowSize(_obj, height, resizeExistingRows)
        }
    }
    #[fixed_stack_segment]
    fn SetGridCursor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) {
        unsafe {
            wxGrid_SetGridCursor(_obj, row, col)
        }
    }
    #[fixed_stack_segment]
    fn SetGridLineColour(_obj: *u8 /* void* */, col: *u8 /* void* */) {
        unsafe {
            wxGrid_SetGridLineColour(_obj, col)
        }
    }
    #[fixed_stack_segment]
    fn SetLabelBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetLabelBackgroundColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetLabelFont(_obj: *u8 /* void* */, font: *u8 /* void* */) {
        unsafe {
            wxGrid_SetLabelFont(_obj, font)
        }
    }
    #[fixed_stack_segment]
    fn SetLabelTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) {
        unsafe {
            wxGrid_SetLabelTextColour(_obj, colour)
        }
    }
    #[fixed_stack_segment]
    fn SetMargins(_obj: *u8 /* void* */, extraWidth: c_int /* int */, extraHeight: c_int /* int */) {
        unsafe {
            wxGrid_SetMargins(_obj, extraWidth, extraHeight)
        }
    }
    #[fixed_stack_segment]
    fn SetReadOnly(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, isReadOnly: bool /* bool */) {
        unsafe {
            wxGrid_SetReadOnly(_obj, row, col, isReadOnly)
        }
    }
    #[fixed_stack_segment]
    fn SetRowAttr(_obj: *u8 /* void* */, row: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxGrid_SetRowAttr(_obj, row, attr)
        }
    }
    #[fixed_stack_segment]
    fn SetRowLabelAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */) {
        unsafe {
            wxGrid_SetRowLabelAlignment(_obj, horiz, vert)
        }
    }
    #[fixed_stack_segment]
    fn SetRowLabelSize(_obj: *u8 /* void* */, width: c_int /* int */) {
        unsafe {
            wxGrid_SetRowLabelSize(_obj, width)
        }
    }
    #[fixed_stack_segment]
    fn SetRowLabelValue(_obj: *u8 /* void* */, row: c_int /* int */, label: *u8 /* void* */) {
        unsafe {
            wxGrid_SetRowLabelValue(_obj, row, label)
        }
    }
    #[fixed_stack_segment]
    fn SetRowMinimalHeight(_obj: *u8 /* void* */, row: c_int /* int */, width: c_int /* int */) {
        unsafe {
            wxGrid_SetRowMinimalHeight(_obj, row, width)
        }
    }
    #[fixed_stack_segment]
    fn SetRowSize(_obj: *u8 /* void* */, row: c_int /* int */, height: c_int /* int */) {
        unsafe {
            wxGrid_SetRowSize(_obj, row, height)
        }
    }
    #[fixed_stack_segment]
    fn SetSelectionBackground(_obj: *u8 /* void* */, c: *u8 /* void* */) {
        unsafe {
            wxGrid_SetSelectionBackground(_obj, c)
        }
    }
    #[fixed_stack_segment]
    fn SetSelectionForeground(_obj: *u8 /* void* */, c: *u8 /* void* */) {
        unsafe {
            wxGrid_SetSelectionForeground(_obj, c)
        }
    }
    #[fixed_stack_segment]
    fn SetSelectionMode(_obj: *u8 /* void* */, selmode: c_int /* int */) {
        unsafe {
            wxGrid_SetSelectionMode(_obj, selmode)
        }
    }
    #[fixed_stack_segment]
    fn SetTable(_obj: *u8 /* void* */, table: *u8 /* void* */, takeOwnership: bool /* bool */, selmode: c_int /* int */) -> bool /* bool */ {
        unsafe {
            wxGrid_SetTable(_obj, table, takeOwnership, selmode)
        }
    }
    #[fixed_stack_segment]
    fn ShowCellEditControl(_obj: *u8 /* void* */) {
        unsafe {
            wxGrid_ShowCellEditControl(_obj)
        }
    }
    #[fixed_stack_segment]
    fn StringToLines(_obj: *u8 /* void* */, value: *u8 /* void* */, lines: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxGrid_StringToLines(_obj, value, lines)
        }
    }
    #[fixed_stack_segment]
    fn XToCol(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_XToCol(_obj, x)
        }
    }
    #[fixed_stack_segment]
    fn XToEdgeOfCol(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_XToEdgeOfCol(_obj, x)
        }
    }
    #[fixed_stack_segment]
    fn XYToCell(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */) {
        unsafe {
            wxGrid_XYToCell(_obj, arg0, arg1, arg2, arg3)
        }
    }
    #[fixed_stack_segment]
    fn YToEdgeOfRow(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_YToEdgeOfRow(_obj, y)
        }
    }
    #[fixed_stack_segment]
    fn YToRow(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */ {
        unsafe {
            wxGrid_YToRow(_obj, y)
        }
    }
    #[fixed_stack_segment]
    fn GetSelectedCells(_obj: *u8 /* void* */, _arr: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectedCells(_obj, _arr)
        }
    }
    #[fixed_stack_segment]
    fn GetSelectionBlockTopLeft(_obj: *u8 /* void* */, _arr: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectionBlockTopLeft(_obj, _arr)
        }
    }
    #[fixed_stack_segment]
    fn GetSelectionBlockBottomRight(_obj: *u8 /* void* */, _arr: *u8 /* void* */) {
        unsafe {
            wxGrid_GetSelectionBlockBottomRight(_obj, _arr)
        }
    }
    #[fixed_stack_segment]
    fn GetSelectedRows(_obj: *u8 /* void* */, _arr: *intptr_t /* intptr_t* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetSelectedRows(_obj, _arr)
        }
    }
    #[fixed_stack_segment]
    fn GetSelectedCols(_obj: *u8 /* void* */, _arr: *intptr_t /* intptr_t* */) -> c_int /* int */ {
        unsafe {
            wxGrid_GetSelectedCols(_obj, _arr)
        }
    }
    #[fixed_stack_segment]
    fn GetCellSize(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */) {
        unsafe {
            wxGrid_GetCellSize(_obj, row, col, arg0, arg1)
        }
    }
    #[fixed_stack_segment]
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
    #[fixed_stack_segment]
    fn Create() -> *u8 /* void* */ {
        unsafe {
            cbBarInfo_Create()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            cbBarInfo_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsExpanded(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            cbBarInfo_IsExpanded(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsFixed(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            cbBarInfo_IsFixed(_obj)
        }
    }
}
trait wxPropertyGrid {
    #[fixed_stack_segment]
    fn Append(_obj: *u8 /* void* */, prop: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxPropertyGrid_Append(_obj, prop)
        }
    }
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxPropertyGrid_Create(_prt, _id, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn DisableProperty(_obj: *u8 /* void* */, propName: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPropertyGrid_DisableProperty(_obj, propName)
        }
    }
}
trait wxGridCellBoolEditor {
    #[fixed_stack_segment]
    fn Ctor() -> *u8 /* void* */ {
        unsafe {
            wxGridCellBoolEditor_Ctor()
        }
    }
}
trait wxCalendarCtrl {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _dat: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCalendarCtrl_Create(_prt, _id, _dat, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn EnableHolidayDisplay(_obj: *u8 /* void* */, display: c_int /* int */) {
        unsafe {
            wxCalendarCtrl_EnableHolidayDisplay(_obj, display)
        }
    }
    #[fixed_stack_segment]
    fn EnableMonthChange(_obj: *u8 /* void* */, enable: bool /* bool */) {
        unsafe {
            wxCalendarCtrl_EnableMonthChange(_obj, enable)
        }
    }
    #[fixed_stack_segment]
    fn GetAttr(_obj: *u8 /* void* */, day: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCalendarCtrl_GetAttr(_obj, day)
        }
    }
    #[fixed_stack_segment]
    fn GetDate(_obj: *u8 /* void* */, date: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetDate(_obj, date)
        }
    }
    #[fixed_stack_segment]
    fn GetHeaderColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHeaderColourBg(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetHeaderColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHeaderColourFg(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetHighlightColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHighlightColourBg(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetHighlightColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHighlightColourFg(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetHolidayColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHolidayColourBg(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetHolidayColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_GetHolidayColourFg(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, date: *u8 /* void* */, wd: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxCalendarCtrl_HitTest(_obj, arg0, arg1, date, wd)
        }
    }
    #[fixed_stack_segment]
    fn ResetAttr(_obj: *u8 /* void* */, day: c_int /* int */) {
        unsafe {
            wxCalendarCtrl_ResetAttr(_obj, day)
        }
    }
    #[fixed_stack_segment]
    fn SetAttr(_obj: *u8 /* void* */, day: c_int /* int */, attr: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetAttr(_obj, day, attr)
        }
    }
    #[fixed_stack_segment]
    fn SetDate(_obj: *u8 /* void* */, date: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetDate(_obj, date)
        }
    }
    #[fixed_stack_segment]
    fn SetHeaderColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetHeaderColours(_obj, colFg, colBg)
        }
    }
    #[fixed_stack_segment]
    fn SetHighlightColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetHighlightColours(_obj, colFg, colBg)
        }
    }
    #[fixed_stack_segment]
    fn SetHoliday(_obj: *u8 /* void* */, day: c_int /* int */) {
        unsafe {
            wxCalendarCtrl_SetHoliday(_obj, day)
        }
    }
    #[fixed_stack_segment]
    fn SetHolidayColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */) {
        unsafe {
            wxCalendarCtrl_SetHolidayColours(_obj, colFg, colBg)
        }
    }
}
trait wxSingleInstanceChecker {
    #[fixed_stack_segment]
    fn Create(_obj: *u8 /* void* */, name: *u8 /* void* */, path: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSingleInstanceChecker_Create(_obj, name, path)
        }
    }
    #[fixed_stack_segment]
    fn CreateDefault() -> *u8 /* void* */ {
        unsafe {
            wxSingleInstanceChecker_CreateDefault()
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxSingleInstanceChecker_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsAnotherRunning(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSingleInstanceChecker_IsAnotherRunning(_obj)
        }
    }
}
trait wxTreeEvent {
    #[fixed_stack_segment]
    fn GetCode(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxTreeEvent_GetCode(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxTreeEvent_GetItem(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeEvent_GetLabel(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetOldItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */) {
        unsafe {
            wxTreeEvent_GetOldItem(_obj, _ref)
        }
    }
    #[fixed_stack_segment]
    fn GetPoint(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxTreeEvent_GetPoint(_obj)
        }
    }
}
trait wxClassInfo {
    #[fixed_stack_segment]
    fn CreateClassByName(_inf: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_CreateClassByName(_inf)
        }
    }
    #[fixed_stack_segment]
    fn GetClassName(_inf: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_GetClassName(_inf)
        }
    }
    #[fixed_stack_segment]
    fn IsKindOf(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClassInfo_IsKindOf(_obj, _name)
        }
    }
    #[fixed_stack_segment]
    fn FindClass(_txt: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_FindClass(_txt)
        }
    }
    #[fixed_stack_segment]
    fn GetBaseClassName1(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_GetBaseClassName1(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetBaseClassName2(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_GetBaseClassName2(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetClassNameEx(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxClassInfo_GetClassNameEx(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetSize(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxClassInfo_GetSize(_obj)
        }
    }
    #[fixed_stack_segment]
    fn IsKindOfEx(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxClassInfo_IsKindOfEx(_obj, classInfo)
        }
    }
    #[fixed_stack_segment]
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
trait wxManagedPtr {
    #[fixed_stack_segment]
    fn GetPtr(self_: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_GetPtr(self_)
        }
    }
    #[fixed_stack_segment]
    fn NoFinalize(self_: *u8 /* void* */) {
        unsafe {
            wxManagedPtr_NoFinalize(self_)
        }
    }
    #[fixed_stack_segment]
    fn Finalize(self_: *u8 /* void* */) {
        unsafe {
            wxManagedPtr_Finalize(self_)
        }
    }
    #[fixed_stack_segment]
    fn Delete(self_: *u8 /* void* */) {
        unsafe {
            wxManagedPtr_Delete(self_)
        }
    }
    #[fixed_stack_segment]
    fn GetDeleteFunction() -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_GetDeleteFunction()
        }
    }
    #[fixed_stack_segment]
    fn CreateFromObject(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromObject(obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromDateTime(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromDateTime(obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromGridCellCoordsArray(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromGridCellCoordsArray(obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromBitmap(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromBitmap(obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromIcon(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromIcon(obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromBrush(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromBrush(obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromColour(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromColour(obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromCursor(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromCursor(obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromFont(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromFont(obj)
        }
    }
    #[fixed_stack_segment]
    fn CreateFromPen(obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxManagedPtr_CreateFromPen(obj)
        }
    }
    #[fixed_stack_segment]
    fn wxObject_SafeDelete(self_: *u8 /* void* */) {
        unsafe {
            wxObject_SafeDelete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxBitmap_SafeDelete(self_: *u8 /* void* */) {
        unsafe {
            wxBitmap_SafeDelete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxIcon_SafeDelete(self_: *u8 /* void* */) {
        unsafe {
            wxIcon_SafeDelete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxBrush_SafeDelete(self_: *u8 /* void* */) {
        unsafe {
            wxBrush_SafeDelete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxColour_SafeDelete(self_: *u8 /* void* */) {
        unsafe {
            wxColour_SafeDelete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxCursor_SafeDelete(self_: *u8 /* void* */) {
        unsafe {
            wxCursor_SafeDelete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxFont_SafeDelete(self_: *u8 /* void* */) {
        unsafe {
            wxFont_SafeDelete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxPen_SafeDelete(self_: *u8 /* void* */) {
        unsafe {
            wxPen_SafeDelete(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxBitmap_IsStatic(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBitmap_IsStatic(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxIcon_IsStatic(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxIcon_IsStatic(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxBrush_IsStatic(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxBrush_IsStatic(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxColour_IsStatic(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxColour_IsStatic(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxCursor_IsStatic(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCursor_IsStatic(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxFont_IsStatic(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxFont_IsStatic(self_)
        }
    }
    #[fixed_stack_segment]
    fn wxPen_IsStatic(self_: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxPen_IsStatic(self_)
        }
    }
}
trait wxCheckBox {
    #[fixed_stack_segment]
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxCheckBox_Create(_prt, _id, _txt, arg0, arg1, arg2, arg3, _stl)
        }
    }
    #[fixed_stack_segment]
    fn Delete(_obj: *u8 /* void* */) {
        unsafe {
            wxCheckBox_Delete(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxCheckBox_GetValue(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetValue(_obj: *u8 /* void* */, value: c_int /* int */) {
        unsafe {
            wxCheckBox_SetValue(_obj, value)
        }
    }
}
trait cbLayoutRowEvent {
    #[fixed_stack_segment]
    fn Row(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbLayoutRowEvent_Row(_obj)
        }
    }
}
trait wxTextEntryDialog {
}
trait wxBoxSizer {
    #[fixed_stack_segment]
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxBoxSizer_CalcMin(_obj)
        }
    }
    #[fixed_stack_segment]
    fn Create(orient: c_int /* int */) -> *u8 /* void* */ {
        unsafe {
            wxBoxSizer_Create(orient)
        }
    }
    #[fixed_stack_segment]
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxBoxSizer_GetOrientation(_obj)
        }
    }
    #[fixed_stack_segment]
    fn RecalcSizes(_obj: *u8 /* void* */) {
        unsafe {
            wxBoxSizer_RecalcSizes(_obj)
        }
    }
}
trait wxSetCursorEvent {
    #[fixed_stack_segment]
    fn GetCursor(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            wxSetCursorEvent_GetCursor(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSetCursorEvent_GetX(_obj)
        }
    }
    #[fixed_stack_segment]
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */ {
        unsafe {
            wxSetCursorEvent_GetY(_obj)
        }
    }
    #[fixed_stack_segment]
    fn HasCursor(_obj: *u8 /* void* */) -> bool /* bool */ {
        unsafe {
            wxSetCursorEvent_HasCursor(_obj)
        }
    }
    #[fixed_stack_segment]
    fn SetCursor(_obj: *u8 /* void* */, cursor: *u8 /* void* */) {
        unsafe {
            wxSetCursorEvent_SetCursor(_obj, cursor)
        }
    }
}
trait cbDrawPaneDecorEvent {
    #[fixed_stack_segment]
    fn Dc(_obj: *u8 /* void* */) -> *u8 /* void* */ {
        unsafe {
            cbDrawPaneDecorEvent_Dc(_obj)
        }
    }
}
trait wxFileName {
}
