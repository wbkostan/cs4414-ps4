#[cfg(target_os = "win32")]
use std::libc::{c_void, c_ulong, size_t, malloc, free};

#[cfg(target_os = "win32")]
use std::ptr::null;

#[cfg(target_os = "win32", target_arch = "x86")]
#[link_name = "crypt32"]
extern "stdcall" {
	fn CryptAcquireContextW(x: *c_void, c: *c_void, p: *c_void, t: c_ulong, f: c_ulong) -> bool;
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
fn getOSEntropy() -> Option<uint>{
	unsafe {
		let context : *c_void = malloc(8 as size_t) as *c_void; //size of context
		let n : *c_void = null();
		let res1 = CryptAcquireContextW(context, n, n, 1 as c_ulong, 0);
		if(!res1){println("Context bailed");return Some(GetLastError() as uint);}
		
		let ptr: *c_void = malloc(8 as size_t) as *c_void;
		let res2 = CryptGenRandom(context, 8 as c_ulong, ptr);
		if(!res2){
			free(ptr);
			CryptReleaseContext(context, 0);
			free(context);
			println("Couldn't Gen");
			return Some(ptr as uint);
		}
		else{
			let package = Some((ptr as uint));
			free(ptr);
			CryptReleaseContext(context, 0);
			free(context);
			return package;
		}
	}
}


/*Returns a truly random integer value between 0 and RAND_MAX*/
pub fn rand() -> Option<uint>{
	let retVal = getOSEntropy();
	return retVal;
}

fn main() {
	let num = rand();
	println(num.unwrap().to_str());
}
