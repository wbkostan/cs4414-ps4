extern mod std;
extern mod extra;

#[cfg(target_os = "win32")]
use std::libc::{c_void, c_ulong, size_t, malloc, free};

#[cfg(target_os = "win32")]
use std::ptr::null;

#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
use std::io::file_reader;

#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
use std::path;

use extra::bigint;


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
	
	let filepath: ~str = ~"/dev/random";
	
	let read_result: Result<@Reader, ~str>;
	read_result = file_reader(~path::Path(filepath.to_owned()));
	
	match read_result {
		Ok(file) => {
			file.read(mem, 4);
		},
		Err(e) => {
			println(fmt!("Error reading file: %?", e));
		}
	}
	
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

pub fn wrandN(n: uint) -> bigint::BigUint{
	let mut iterations = (n/32) as uint;
	let mut big_digs: ~[bigint::BigDigit] = ~[];
	while (iterations != 0){
		let num = wrand();
		let digs = bigint::BigDigit::from_uint(num);
		big_digs.push(digs.first());
		big_digs.push(digs.second());
		iterations -= 1;
	}
	return bigint::BigUint::new(big_digs)
}

pub fn srandN(n: uint) -> Option<bigint::BigUint>{
	let mut iterations = (n/32) as uint;
	let mut big_digs: ~[bigint::BigDigit] = ~[];
	while (iterations != 0){
		let num = srand();
		match num{
			Some(x) => {
					let digs = bigint::BigDigit::from_uint(x);
					big_digs.push(digs.first());
					big_digs.push(digs.second());
				   },
			None => {return None;},
		}
		iterations -= 1;
	}
	return Some(bigint::BigUint::new(big_digs))
}

fn main() {
	let wnum = wrand();
	let snum = srand();
	let wnumn = wrandN(64 as uint);
	let snumn = srandN(64 as uint);
	println(wnum.to_str());
	match snum{
		Some(x) => {println(x.to_str());},
		None => {println("None");},
	}
	println(wnumn.to_str());
	match snumn{
		Some(x) => {println(x.to_str());},
		None => {println("None");},
	}
}
