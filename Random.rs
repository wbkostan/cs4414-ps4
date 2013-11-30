#[cfg(target_os = "win32")]
use std::libc::{c_void, c_ulong, size_t, malloc, free};

static RAND_MAX:uint = 4294967295; //max unsigned 32 bit integer

#[cfg(target_os = "win32")]
#[link_name = "advapi32"]
extern "stdcall" {
	fn SystemFunction036(buffer: *c_void, size: c_ulong) -> bool;
}

#[cfg(target_os = "win32")]
fn getOSEntropy() -> Option<uint>{
	unsafe {
		let mut ptr: *c_void = malloc(32 as size_t) as *c_void;
		let result = SystemFunction036(ptr, 32 as c_ulong);
		if(!result){
			free(ptr);
			return None;
		}
		else{
			let package = Some((*ptr as uint));
			free(ptr);
			return package;
		}
	}
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
fn getOSEntropy() -> Option<uint>{
	let retVal = Some(3);
	return retVal;
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
