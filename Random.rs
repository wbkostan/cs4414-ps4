static RAND_MAX:uint = 4294967295; //max unsigned 32 bit integer

#[cfg(target_os = "win32")]
fn getOSEntropy() -> Option<uint>{
	let mut retVal = None;
	retVal = Some(5);
	return retVal;
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
fn getOSEntropy() -> Option<uint>{
	let mut retVal = None;
	retVal = Some(3);
	return retVal;
}

/*Returns a truly random integer value between 0 and RAND_MAX*/
pub fn rand() -> Option<uint>{
	let mut retVal = None;
	retVal = getOSEntropy();
	return retVal;
}

fn main() {
	let num = rand();
	println(num.unwrap().to_str());
}
