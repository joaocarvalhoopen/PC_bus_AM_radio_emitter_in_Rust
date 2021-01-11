# PC bus AM radio emitter in Rust
A very cool hack :-D

## Description
This program when finished, will allow you to generate monophonic music tones of a tune (Mary had a little lamb) on a small AM radio, from the precise timings of the increment of a counter in the memory of your PC. Through the PCB traces of your motherboard that connect the CPU to the DDR RAM slots, the electromagnetic waves will radiate and escapes the shield of your computer, going 2 to 3 meters in the air until they finds the small AM Radio antenna, tunned to the frequency of ?????. <br> 
This is code is a port to Rust, of the System Bus Radio (Using counter and threads) [https://github.com/fulldecent/system-bus-radio](https://github.com/fulldecent/system-bus-radio) developed by William Entriken in 2016 and ported to C++11 by Ryou Ezoe.
See it's github repository and the papers for full details.

## Current status: Not working on Windows 10, on a old HP Laptop.
The original code in C++, I tested both version (counters and threads and SIMD instructions) on a Linux Machine Lenovo T430 and they both worked correctly. They both emitted at 1,033 MHz, tested with a small digital AM Radio with a range of 2 to 3 meters around the laptop computer. <br>

This version is a work in progress, it doesn't currently work and I post it here with the objective that other Rust programmers can use it to make a final correct version that emittes in AM in the 1 MHz frequency. <br>

I think that the current problem is that when writing to memory it isn't bypassing all the cache levels in real time. I say this because I have total radio silence from the program from 550 KHz to 1600 KHz. I also developed this program on Windows 10 on a HP 11 years old computer and not on Linux, on a Lenovo T430. I have to recompile this in Linux and test it there to see if the result changes. <br>

There are three other methods that I would like to test. <br>

The first is the SIMD instruction method, to test if I can make the intrinsics work on Rust. This is from the same repository. <br>

The second is the dithering, or digital density AM emissions, in wich one can transmit a WAV file like voice or music instead of a single tone in each moment. <br>

The third method is the simple memcpy() of an array from memory position A to memory position B. The array as to be big and it has to be prepared for the specific  DDR memory Bus speed of the computer. <br>

## References
* System Bus Radio - Using counter and threads <br>
  [https://github.com/fulldecent/system-bus-radio](https://github.com/fulldecent/system-bus-radio)

* Rust - High accuracy timer - Timer with the resolution of nanoseconds. <br>
  [https://users.rust-lang.org/t/high-accuracy-timer/29019](https://users.rust-lang.org/t/high-accuracy-timer/29019)

* The Rust Programming Language Book <br>
  Fearless Concurrency <br>
  [https://doc.rust-lang.org/book/ch16-00-concurrency.html](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

* System BUS AM Radio - WAV musicplayer <br>
  [https://github.com/anfractuosity/musicplayer](https://github.com/anfractuosity/musicplayer) 

* Academics turn RAM into Wi-Fi cards to steal data from air-gapped systems <br>
  [https://www.zdnet.com/article/academics-turn-ram-into-wifi-cards-to-steal-data-from-air-gapped-systems/](https://www.zdnet.com/article/academics-turn-ram-into-wifi-cards-to-steal-data-from-air-gapped-systems/)

* AIR-FI: Generating Covert Wi-Fi Signals from Air-Gapped Computers <br>
  [https://arxiv.org/pdf/2012.06884.pdf](https://arxiv.org/pdf/2012.06884.pdf)

## License
MIT Open Source License.

## Have fun!
Best regards, <br>
Joao Nuno Carvalho <br>
