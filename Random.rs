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

use extra::{bigint, time};
use std::{rand};


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
                        return 0;
                }
                
                let ptr: *c_void = malloc(8 as size_t) as *c_void;
                let res2 = CryptGenRandom(context, 8 as c_ulong, ptr);
                if(!res2){
                        let error = (GetLastError() as uint);
                        free(ptr);
                        CryptReleaseContext(context, 0);
                        free(context);
                        return 0;
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
/*Returns 0 if rdrand fails to supply a number*/
pub fn srand() -> uint{
        let mut rand = 0;
            unsafe {
                asm!("rdrand %eax"
                     : "=r"(rand)
                     );
            }
    return rand;
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
                let mut res = Some(num);
                if(num == 0){res = None;}
                match res{
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

pub fn randN(n: uint) -> bigint::BigUint {
	let mut iterations = (n/32) as uint;
        let mut big_digs: ~[bigint::BigDigit] = ~[];
        while (iterations != 0){
                let num: uint = rand::random();
                let mut res = Some(num);
               	let digs = bigint::BigDigit::from_uint(num);
              	big_digs.push(digs.first());
              	big_digs.push(digs.second());
                iterations -= 1;
        }
        return bigint::BigUint::new(big_digs);
}

static THRESHHOLD: uint = 2;
static BITCOUNT: uint = 64;
static CONV_FACTOR: f64 = 1.0/1000000.0;

fn main() {
        let mut i = THRESHHOLD;
        let mut avg_times: ~[f64] = ~[0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        let mut fail = false;

        while (i != 0){
                /* take timings */
                let t1 = time::precise_time_ns();
                let wnum = wrand();
                let t2 = time::precise_time_ns();
                let snum = srand();
                let t3 = time::precise_time_ns();
                let wnumn = wrandN(BITCOUNT);
                let t4 = time::precise_time_ns();
                let snumn = srandN(BITCOUNT);
                let t5 = time::precise_time_ns();
                let bench1: uint = rand::random();
                let t6 = time::precise_time_ns();
		let bench2 = randN(BITCOUNT);
		let t7 = time::precise_time_ns();
                
                if(snum == 0 || snumn.is_none()){fail = true;}

                /*add to running sums*/
                avg_times[0] = avg_times[0] + (((t2 - t1) as f64) * CONV_FACTOR);
                avg_times[1] = avg_times[1] + (((t3 - t2) as f64) * CONV_FACTOR);
                avg_times[2] = avg_times[2] + (((t4 - t3) as f64) * CONV_FACTOR);
                avg_times[3] = avg_times[3] + (((t5 - t4) as f64) * CONV_FACTOR);
                avg_times[4] = avg_times[4] + (((t6 - t5) as f64) * CONV_FACTOR);
		avg_times[5] = avg_times[5] + (((t7 - t6) as f64) * CONV_FACTOR);

		if(THRESHHOLD < 10){
			println("");
			println("wrand: " + wnum.to_str());
			println("wrandN: " + wnumn.to_str());
			println("snum: " + snum.to_str());
			println("snumn: " + snumn.to_str());
			println("rand: " + bench1.to_str());
			println("randN: " + bench2.to_str());
		}
                
                i -= 1;
        }
	println("");

        for sum in avg_times.mut_iter(){
                *sum = (*sum/(THRESHHOLD as f64));
        }
        
        println("Averages over " + THRESHHOLD.to_str() + " iterations:");
        println("Average time for 32-bit weak random: " + std::f64::to_str_digits(avg_times[0], 6));
        if(!fail){println("Average time for 32-bit strong random: " + std::f64::to_str_digits(avg_times[1], 6));}
        else{println("Average time for 32-bit strong random: N/A");}
        println("Average time for " + BITCOUNT.to_str() + "-bit weak random: " + std::f64::to_str_digits(avg_times[2], 6));
        if(!fail){println("Average time for " + BITCOUNT.to_str() + "-bit strong random: " + std::f64::to_str_digits(avg_times[3], 6));}
        else{println("Average time for " + BITCOUNT.to_str() + "-bit strong random: N/A");}
        println("Benchmark for 32-bit random: " + std::f64::to_str_digits(avg_times[4], 6));
        println("Benchmark for " + BITCOUNT.to_str() + "-bit random: " + std::f64::to_str_digits(avg_times[5], 6));
}
