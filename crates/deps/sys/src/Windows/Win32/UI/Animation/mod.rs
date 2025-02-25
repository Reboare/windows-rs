#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type IUIAnimationInterpolator = *mut ::core::ffi::c_void;
pub type IUIAnimationInterpolator2 = *mut ::core::ffi::c_void;
pub type IUIAnimationLoopIterationChangeHandler2 = *mut ::core::ffi::c_void;
pub type IUIAnimationManager = *mut ::core::ffi::c_void;
pub type IUIAnimationManager2 = *mut ::core::ffi::c_void;
pub type IUIAnimationManagerEventHandler = *mut ::core::ffi::c_void;
pub type IUIAnimationManagerEventHandler2 = *mut ::core::ffi::c_void;
pub type IUIAnimationPrimitiveInterpolation = *mut ::core::ffi::c_void;
pub type IUIAnimationPriorityComparison = *mut ::core::ffi::c_void;
pub type IUIAnimationPriorityComparison2 = *mut ::core::ffi::c_void;
pub type IUIAnimationStoryboard = *mut ::core::ffi::c_void;
pub type IUIAnimationStoryboard2 = *mut ::core::ffi::c_void;
pub type IUIAnimationStoryboardEventHandler = *mut ::core::ffi::c_void;
pub type IUIAnimationStoryboardEventHandler2 = *mut ::core::ffi::c_void;
pub type IUIAnimationTimer = *mut ::core::ffi::c_void;
pub type IUIAnimationTimerClientEventHandler = *mut ::core::ffi::c_void;
pub type IUIAnimationTimerEventHandler = *mut ::core::ffi::c_void;
pub type IUIAnimationTimerUpdateHandler = *mut ::core::ffi::c_void;
pub type IUIAnimationTransition = *mut ::core::ffi::c_void;
pub type IUIAnimationTransition2 = *mut ::core::ffi::c_void;
pub type IUIAnimationTransitionFactory = *mut ::core::ffi::c_void;
pub type IUIAnimationTransitionFactory2 = *mut ::core::ffi::c_void;
pub type IUIAnimationTransitionLibrary = *mut ::core::ffi::c_void;
pub type IUIAnimationTransitionLibrary2 = *mut ::core::ffi::c_void;
pub type IUIAnimationVariable = *mut ::core::ffi::c_void;
pub type IUIAnimationVariable2 = *mut ::core::ffi::c_void;
pub type IUIAnimationVariableChangeHandler = *mut ::core::ffi::c_void;
pub type IUIAnimationVariableChangeHandler2 = *mut ::core::ffi::c_void;
pub type IUIAnimationVariableCurveChangeHandler2 = *mut ::core::ffi::c_void;
pub type IUIAnimationVariableIntegerChangeHandler = *mut ::core::ffi::c_void;
pub type IUIAnimationVariableIntegerChangeHandler2 = *mut ::core::ffi::c_void;
pub const UIAnimationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1277150778, data2: 26972, data3: 18408, data4: [163, 57, 26, 25, 75, 227, 208, 184] };
pub const UIAnimationManager2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3529345090, data2: 34948, data3: 19018, data4: [179, 33, 9, 19, 20, 55, 155, 221] };
pub const UIAnimationTimer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3217902092, data2: 1718, data3: 17284, data4: [183, 104, 13, 170, 121, 44, 56, 14] };
pub const UIAnimationTransitionFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2325421277, data2: 64727, data3: 16796, data4: [139, 68, 66, 253, 23, 219, 24, 135] };
pub const UIAnimationTransitionFactory2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2217750423,
    data2: 32635,
    data3: 16448,
    data4: [177, 144, 114, 172, 157, 24, 228, 32],
};
pub const UIAnimationTransitionLibrary: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 493036205, data2: 43653, data3: 20213, data4: [168, 40, 134, 215, 16, 103, 209, 69] };
pub const UIAnimationTransitionLibrary2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2167379018,
    data2: 50632,
    data3: 19673,
    data4: [176, 166, 179, 218, 128, 47, 34, 141],
};
pub type UI_ANIMATION_DEPENDENCIES = u32;
pub const UI_ANIMATION_DEPENDENCY_NONE: UI_ANIMATION_DEPENDENCIES = 0u32;
pub const UI_ANIMATION_DEPENDENCY_INTERMEDIATE_VALUES: UI_ANIMATION_DEPENDENCIES = 1u32;
pub const UI_ANIMATION_DEPENDENCY_FINAL_VALUE: UI_ANIMATION_DEPENDENCIES = 2u32;
pub const UI_ANIMATION_DEPENDENCY_FINAL_VELOCITY: UI_ANIMATION_DEPENDENCIES = 4u32;
pub const UI_ANIMATION_DEPENDENCY_DURATION: UI_ANIMATION_DEPENDENCIES = 8u32;
pub type UI_ANIMATION_IDLE_BEHAVIOR = i32;
pub const UI_ANIMATION_IDLE_BEHAVIOR_CONTINUE: UI_ANIMATION_IDLE_BEHAVIOR = 0i32;
pub const UI_ANIMATION_IDLE_BEHAVIOR_DISABLE: UI_ANIMATION_IDLE_BEHAVIOR = 1i32;
pub type UI_ANIMATION_KEYFRAME = isize;
pub type UI_ANIMATION_MANAGER_STATUS = i32;
pub const UI_ANIMATION_MANAGER_IDLE: UI_ANIMATION_MANAGER_STATUS = 0i32;
pub const UI_ANIMATION_MANAGER_BUSY: UI_ANIMATION_MANAGER_STATUS = 1i32;
pub type UI_ANIMATION_MODE = i32;
pub const UI_ANIMATION_MODE_DISABLED: UI_ANIMATION_MODE = 0i32;
pub const UI_ANIMATION_MODE_SYSTEM_DEFAULT: UI_ANIMATION_MODE = 1i32;
pub const UI_ANIMATION_MODE_ENABLED: UI_ANIMATION_MODE = 2i32;
pub type UI_ANIMATION_PRIORITY_EFFECT = i32;
pub const UI_ANIMATION_PRIORITY_EFFECT_FAILURE: UI_ANIMATION_PRIORITY_EFFECT = 0i32;
pub const UI_ANIMATION_PRIORITY_EFFECT_DELAY: UI_ANIMATION_PRIORITY_EFFECT = 1i32;
pub const UI_ANIMATION_REPEAT_INDEFINITELY: i32 = -1i32;
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END: i32 = -1i32;
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START: i32 = -2i32;
pub type UI_ANIMATION_REPEAT_MODE = i32;
pub const UI_ANIMATION_REPEAT_MODE_NORMAL: UI_ANIMATION_REPEAT_MODE = 0i32;
pub const UI_ANIMATION_REPEAT_MODE_ALTERNATE: UI_ANIMATION_REPEAT_MODE = 1i32;
pub type UI_ANIMATION_ROUNDING_MODE = i32;
pub const UI_ANIMATION_ROUNDING_NEAREST: UI_ANIMATION_ROUNDING_MODE = 0i32;
pub const UI_ANIMATION_ROUNDING_FLOOR: UI_ANIMATION_ROUNDING_MODE = 1i32;
pub const UI_ANIMATION_ROUNDING_CEILING: UI_ANIMATION_ROUNDING_MODE = 2i32;
pub type UI_ANIMATION_SCHEDULING_RESULT = i32;
pub const UI_ANIMATION_SCHEDULING_UNEXPECTED_FAILURE: UI_ANIMATION_SCHEDULING_RESULT = 0i32;
pub const UI_ANIMATION_SCHEDULING_INSUFFICIENT_PRIORITY: UI_ANIMATION_SCHEDULING_RESULT = 1i32;
pub const UI_ANIMATION_SCHEDULING_ALREADY_SCHEDULED: UI_ANIMATION_SCHEDULING_RESULT = 2i32;
pub const UI_ANIMATION_SCHEDULING_SUCCEEDED: UI_ANIMATION_SCHEDULING_RESULT = 3i32;
pub const UI_ANIMATION_SCHEDULING_DEFERRED: UI_ANIMATION_SCHEDULING_RESULT = 4i32;
pub const UI_ANIMATION_SECONDS_EVENTUALLY: i32 = -1i32;
pub const UI_ANIMATION_SECONDS_INFINITE: i32 = -1i32;
pub type UI_ANIMATION_SLOPE = i32;
pub const UI_ANIMATION_SLOPE_INCREASING: UI_ANIMATION_SLOPE = 0i32;
pub const UI_ANIMATION_SLOPE_DECREASING: UI_ANIMATION_SLOPE = 1i32;
pub type UI_ANIMATION_STORYBOARD_STATUS = i32;
pub const UI_ANIMATION_STORYBOARD_BUILDING: UI_ANIMATION_STORYBOARD_STATUS = 0i32;
pub const UI_ANIMATION_STORYBOARD_SCHEDULED: UI_ANIMATION_STORYBOARD_STATUS = 1i32;
pub const UI_ANIMATION_STORYBOARD_CANCELLED: UI_ANIMATION_STORYBOARD_STATUS = 2i32;
pub const UI_ANIMATION_STORYBOARD_PLAYING: UI_ANIMATION_STORYBOARD_STATUS = 3i32;
pub const UI_ANIMATION_STORYBOARD_TRUNCATED: UI_ANIMATION_STORYBOARD_STATUS = 4i32;
pub const UI_ANIMATION_STORYBOARD_FINISHED: UI_ANIMATION_STORYBOARD_STATUS = 5i32;
pub const UI_ANIMATION_STORYBOARD_READY: UI_ANIMATION_STORYBOARD_STATUS = 6i32;
pub const UI_ANIMATION_STORYBOARD_INSUFFICIENT_PRIORITY: UI_ANIMATION_STORYBOARD_STATUS = 7i32;
pub type UI_ANIMATION_TIMER_CLIENT_STATUS = i32;
pub const UI_ANIMATION_TIMER_CLIENT_IDLE: UI_ANIMATION_TIMER_CLIENT_STATUS = 0i32;
pub const UI_ANIMATION_TIMER_CLIENT_BUSY: UI_ANIMATION_TIMER_CLIENT_STATUS = 1i32;
pub type UI_ANIMATION_UPDATE_RESULT = i32;
pub const UI_ANIMATION_UPDATE_NO_CHANGE: UI_ANIMATION_UPDATE_RESULT = 0i32;
pub const UI_ANIMATION_UPDATE_VARIABLES_CHANGED: UI_ANIMATION_UPDATE_RESULT = 1i32;
