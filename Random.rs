#[cfg(target_os = "win32")]
use std::libc::{c_void, c_ulong, size_t, malloc, free};

#[cfg(target_os = "win32")]
use std::ptr::null;

#[cfg(target_os = "win32", target_arch = "x86")]
#[link_name = "crypt32"]
extern "stdcall" {
	fn CryptAcquireContextA(x: *c_void, c: *c_void, p: *c_void, t: c_ulong, f: c_ulong) -> bool;
	fn CryptReleaseContext(x: *c_void, f: c_ulong) -> bool;
	fn CryptGenRandom(x: *c_void, l: c_ulong, d: *c_void) -> bool;
}

#[cfg(target_os = "win32", target_arch = "x86")]
#[link_name = "kernel32"]
extern "stdcall" {
	fn GetLastError() -> c_ulong;
}

#[cfg(target_os = "win32")]
#[fixed_stack_segment]
fn getOSEntropy() -> uint{
	unsafe {
		let context : *c_void = malloc(8 as size_t) as *c_void; //size of context
		let n : *c_void = null();
		let res1 = CryptAcquireContextA(context, n, n, 1 as c_ulong, 4026531848 as c_ulong);
		if(!res1){
			let error = (GetLastError() as uint);
			println("Context bailed");
			return error;
		}
		
		let ptr: *c_void = malloc(8 as size_t) as *c_void;
		let res2 = CryptGenRandom(context, 8 as c_ulong, ptr);
		if(!res2){
			let error = (GetLastError() as uint);
			free(ptr);
			CryptReleaseContext(context, 0);
			free(context);
			println("Couldn't Gen");
			return error;
		}
		else{
			let package = (ptr as uint);
			free(ptr);
			CryptReleaseContext(context, 0);
			free(context);
			return package;
		}
	}
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
pub fn getOSEntropy() -> uint{
	let mut mem : ~[u8] = ~[1,1,1,1];
	
	/* Read from file to mem */
	
	let mut res : u32 = 0;
	for val in mem.iter(){
		res = res << 8;
		res = res | (*val as u32);
	}
	
	return (res as uint);
}

/*Returns a truly random integer value between 0 and RAND_MAX*/
pub fn srand() -> Option<uint>{
	return None;
}

pub fn wrand() -> uint{
	return getOSEntropy();
}

fn main() {
	let num = wrand();
	println(num.to_str());
}
