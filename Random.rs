mod Random;

const RAND_MAX:uint = 4294967295 //max unsigned 32 bit integer

/*Returns a truly random integer value between 0 and RAND_MAX*/
pub fn rand() -> uint{
	/*Read some values from /dev/random*/
	/*XOR some values with results of RdRand or similar*/
		/*Use some inline assembly to call RdRand*/
		/*Make sure we try to read until we get success*/
	/*Return our random value*/
}
