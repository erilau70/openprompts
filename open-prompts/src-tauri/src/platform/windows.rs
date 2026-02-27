#![cfg(target_os = "windows")]

use std::mem::size_of;
use std::thread;
use std::time::{Duration, Instant};

use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::Graphics::Gdi::{
	GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::System::Threading::{
	AttachThreadInput, GetCurrentProcessId, GetCurrentThreadId,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
	GetAsyncKeyState, SendInput, VIRTUAL_KEY, INPUT, INPUT_0, INPUT_KEYBOARD,
	KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_MENU, VK_V,
};
use windows::Win32::UI::WindowsAndMessaging::{
	GetForegroundWindow, GetWindowThreadProcessId, IsIconic, IsWindow, SetForegroundWindow,
	ShowWindow, SW_RESTORE,
};

fn hwnd_from_isize(raw: isize) -> HWND {
	HWND(raw as *mut std::ffi::c_void)
}

fn hwnd_to_isize(hwnd: HWND) -> isize {
	hwnd.0 as isize
}

pub fn get_current_process_id() -> u32 {
	// SAFETY: GetCurrentProcessId has no preconditions and does not dereference pointers.
	unsafe { GetCurrentProcessId() }
}

pub fn capture_foreground_hwnd(own_pid: u32) -> Option<isize> {
	// SAFETY: GetForegroundWindow has no preconditions and returns a possibly-null HWND.
	let foreground = unsafe { GetForegroundWindow() };
	if foreground.0.is_null() {
		return None;
	}

	let mut pid = 0u32;
	// SAFETY: foreground is an HWND received from the OS; pid points to valid writable memory.
	unsafe {
		GetWindowThreadProcessId(foreground, Some(&mut pid));
	}

	if pid == own_pid {
		return None;
	}

	Some(hwnd_to_isize(foreground))
}

pub fn is_valid_window(hwnd: isize) -> bool {
	let hwnd = hwnd_from_isize(hwnd);
	// SAFETY: IsWindow accepts any HWND value and reports validity.
	unsafe { IsWindow(hwnd).as_bool() }
}

pub fn force_foreground(target_hwnd: isize) -> bool {
	let hwnd = hwnd_from_isize(target_hwnd);
	if !is_valid_window(target_hwnd) {
		eprintln!("force_foreground: target window is invalid: {target_hwnd}");
		return false;
	}

	// SAFETY: hwnd is validated above; null process-id pointer is allowed to query thread id only.
	let target_thread = unsafe { GetWindowThreadProcessId(hwnd, None) };
	// SAFETY: GetCurrentThreadId has no preconditions.
	let our_thread = unsafe { GetCurrentThreadId() };

	let mut attached = false;
	if our_thread != target_thread {
		// SAFETY: Thread IDs are obtained from the OS; BOOL(1) requests attach.
		let attach_ok = unsafe { AttachThreadInput(our_thread, target_thread, BOOL(1)).as_bool() };
		if !attach_ok {
			eprintln!(
				"force_foreground: AttachThreadInput attach failed (our_thread={our_thread}, target_thread={target_thread})"
			);
			return false;
		}
		attached = true;
	}

	// SAFETY: IsIconic/ShowWindow operate on HWND values; hwnd was validated as a window.
	unsafe {
		if IsIconic(hwnd).as_bool() {
			let _ = ShowWindow(hwnd, SW_RESTORE);
		}
	}

	// SAFETY: SetForegroundWindow operates on an HWND; hwnd is validated.
	let set_ok = unsafe { SetForegroundWindow(hwnd).as_bool() };

	if attached {
		// SAFETY: Mirrors successful attach call above; BOOL(0) requests detach.
		let detach_ok = unsafe { AttachThreadInput(our_thread, target_thread, BOOL(0)).as_bool() };
		if !detach_ok {
			eprintln!(
				"force_foreground: AttachThreadInput detach failed (our_thread={our_thread}, target_thread={target_thread})"
			);
		}
	}

	if !set_ok {
		eprintln!("force_foreground: SetForegroundWindow failed for hwnd={target_hwnd}");
	}

	set_ok
}

pub fn wait_for_focus(target_hwnd: isize, timeout_ms: u64) -> bool {
	let hwnd = hwnd_from_isize(target_hwnd);
	let deadline = Instant::now() + Duration::from_millis(timeout_ms);

	while Instant::now() < deadline {
		// SAFETY: GetForegroundWindow has no preconditions.
		let current = unsafe { GetForegroundWindow() };
		if current == hwnd {
			return true;
		}
		thread::sleep(Duration::from_millis(10));
	}

	false
}

pub fn wait_for_modifier_release(timeout_ms: u64) {
	let deadline = Instant::now() + Duration::from_millis(timeout_ms);

	while Instant::now() < deadline {
		// SAFETY: GetAsyncKeyState is safe to call for virtual key constants.
		let ctrl_state = unsafe { GetAsyncKeyState(VK_CONTROL.0 as i32) };
		// SAFETY: GetAsyncKeyState is safe to call for virtual key constants.
		let alt_state = unsafe { GetAsyncKeyState(VK_MENU.0 as i32) };

		let ctrl_down = (ctrl_state as u16 & 0x8000) != 0;
		let alt_down = (alt_state as u16 & 0x8000) != 0;

		if !ctrl_down && !alt_down {
			return;
		}

		thread::sleep(Duration::from_millis(10));
	}
}

pub fn send_ctrl_v() -> bool {
	let inputs = [
		INPUT {
			r#type: INPUT_KEYBOARD,
			Anonymous: INPUT_0 {
				ki: KEYBDINPUT {
					wVk: VK_CONTROL,
					wScan: 0,
					dwFlags: Default::default(),
					time: 0,
					dwExtraInfo: 0,
				},
			},
		},
		INPUT {
			r#type: INPUT_KEYBOARD,
			Anonymous: INPUT_0 {
				ki: KEYBDINPUT {
					wVk: VIRTUAL_KEY(VK_V.0),
					wScan: 0,
					dwFlags: Default::default(),
					time: 0,
					dwExtraInfo: 0,
				},
			},
		},
		INPUT {
			r#type: INPUT_KEYBOARD,
			Anonymous: INPUT_0 {
				ki: KEYBDINPUT {
					wVk: VIRTUAL_KEY(VK_V.0),
					wScan: 0,
					dwFlags: KEYEVENTF_KEYUP,
					time: 0,
					dwExtraInfo: 0,
				},
			},
		},
		INPUT {
			r#type: INPUT_KEYBOARD,
			Anonymous: INPUT_0 {
				ki: KEYBDINPUT {
					wVk: VK_CONTROL,
					wScan: 0,
					dwFlags: KEYEVENTF_KEYUP,
					time: 0,
					dwExtraInfo: 0,
				},
			},
		},
	];

	// SAFETY: `inputs` is a valid contiguous array of INPUT events; cbSize matches INPUT size.
	let sent = unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) };
	if sent != inputs.len() as u32 {
		eprintln!(
			"send_ctrl_v: SendInput sent {sent} of {} events (possible UIPI blocking)",
			inputs.len()
		);
		return false;
	}

	true
}

pub fn get_launcher_position(
	foreground_hwnd: isize,
	launcher_width: i32,
	_launcher_height: i32,
) -> Option<(i32, i32)> {
	let hwnd = hwnd_from_isize(foreground_hwnd);

	// SAFETY: hwnd can be any top-level window handle; MONITOR_DEFAULTTONEAREST guarantees a monitor.
	let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
	if monitor.0.is_null() {
		eprintln!("get_launcher_position: MonitorFromWindow returned null monitor");
		return None;
	}

	let mut info = MONITORINFO {
		cbSize: size_of::<MONITORINFO>() as u32,
		..Default::default()
	};

	// SAFETY: `monitor` is from MonitorFromWindow; `info` points to initialized writable MONITORINFO.
	let monitor_ok = unsafe { GetMonitorInfoW(monitor, &mut info as *mut MONITORINFO as *mut _).as_bool() };
	if !monitor_ok {
		eprintln!("get_launcher_position: GetMonitorInfoW failed");
		return None;
	}

	let work = info.rcWork;
	let work_width = work.right - work.left;
	let work_height = work.bottom - work.top;

	let x = work.left + (work_width - launcher_width) / 2;
	let y = work.top + work_height / 4;

	Some((x, y))
}
