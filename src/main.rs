// Port to Rust by Joao Nuno Carvalho of the
//  
// SYSTEM BUS RADIO - Using counter and threads
// https://github.com/fulldecent/system-bus-radio
// Copyright 2016 William Entriken
// C++11 port by Ryou Ezoe
//
// License: MIT Open Source License 
// Date: 2021.01.11

// Current status:
//     The original code in C++, I tested both version (counters and threads and
//     SIMD instructions) on a Linux Machine Lenovo T430 and they worked correctly
//     They both emitted at 1,033 MHz tested with a small digital AM Radio
//     with a range of 2 to 3 meters around the laptop computer. 
//
//     This version is a work in progress, it doesn't currently and I post it here
//     with the objective that other Rust programmers can use it to make a final
//     correct version that emittes in AM in the 1 MHz frequency.
//
//     I think that the current problem is that when writing to memory it is not
//     bypassed all the cache levels in real time. I say this because I have total
//     radio silence from the program from 550 KHz to 1600 KHz.
//     I also developed this program on Windows 10 on a HP 11 years old computer
//     and not on Linux, on a Lenovo T430.
//     I have to recompile this in Linux and test it there to see if the result
//     changes.   
//
//     There are three other methods that I would like to test.
//
//     The first is the SIMD instruction method, to test if I can make the intrinsics
//     work on Rust. This is from the same repository. <br>
//
//     The second is the dithering, or digital density AM emissions, in which one
//     can transmit a WAV file like voice or music instead of a single tone in
//     each moment.
//
//     The third method is the simple memcpy() of an array from memory position A
//     to memory position B. The array as to be big and it has to be prepared for
//     the specific  DDR memory Bus speed of the computer. This last method can be
//     performed with any programming language.


use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering}; // To write to memory bypassing cache.
use std::sync::mpsc;                            // Channels inter-process communication.
use std::sync::mpsc::{Sender, Receiver};
use volatile::Volatile;

static _NTHREADS: i32 = 1;

#[derive(Clone, Copy)]
struct TimePair
{
    mid:    u64,
    period: u64,   // reset
}

/// The first version uses Atomics to write directly to memory
/// bypassing all cache levels.
fn boost_song_1(rx: Receiver<TimePair>)
{
    loop
    {
        // Get's from the main thread square_am_signal() and synchronizes this thread. 
        let time_pair: TimePair = rx.recv().unwrap(); 
        let mid:    u64 = time_pair.mid;
        let period: u64 = time_pair.period;

        // It uses a atomic variable (counter) so it ensures that the value 
        // is written to memory bypassing the cache of the processor.
        // Remember that it is the writing to the memory using the memory BUS
        // that we want. 
        
        let x = AtomicUsize::new(0);

        // while core::arch::x86_64::_rdtsc() < mid     
        loop
        {
            unsafe{
                if  core::arch::x86_64::_rdtsc() > mid 
                {
                    break;
                }
            }
            // Increments the atomic counter.
            x.fetch_add(1, Ordering::SeqCst);
        }
        thread::sleep(Duration::from_nanos(period));
    }
}

/// The second version uses Volatile crate to write directly to memory
/// bypassing all cache levels. [I don't know if this is really the case
/// but it should be the case].
fn boost_song_2(rx: Receiver<TimePair>)
{
    loop
    {
        // Get's from the main thread square_am_signal() and synchronizes this thread. 
        let time_pair: TimePair = rx.recv().unwrap(); 
        let mid:    u64 = time_pair.mid;
        let period: u64 = time_pair.period;

        // It uses a atomic variable (counter) so it ensures that the value 
        // is written to memory bypassing the cache of the processor.
        // Remember that it is the writing to the memory using the memory BUS
        // that we want. 
        
        let mut value = 0u64;
        let mut volatile = Volatile::new(&mut value);

        // while core::arch::x86_64::_rdtsc() < mid     
        loop
        {
            unsafe{
                if  core::arch::x86_64::_rdtsc() > mid 
                {
                    break;
                }
            }
            // Increments the volatile variable.
            volatile.write(volatile.read() + 1);
        }
        thread::sleep(Duration::from_nanos(period));
    }
}


/// The second version uses Volatile crate to write directly to memory
/// bypassing all cache levels. [I don't know if this is really the case
/// but it should be the case].

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), 
target_feature = "sse2"))]
fn boost_song_3(rx: Receiver<TimePair>)
{
    loop
    {
        // Get's from the main thread square_am_signal() and synchronizes this thread. 
        let time_pair: TimePair = rx.recv().unwrap(); 
        let mid:    u64 = time_pair.mid;
        let period: u64 = time_pair.period;

        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::*;
    
        // It uses the SIMD _mm_stream_si128 instruction to send data directly
        // to memory bypassing the cache of the processor.
        // Remember that it is the writing to the memory using the memory BUS
        // that we want. 
        unsafe{
            let mut target_mem_m128i: __m128i = _mm_set1_epi32(0);
            let all_bytes_zero_m128i: __m128i = _mm_setzero_si128();
            // Creating all FFFF's or all ones in binary.
            let all_bytes_ffff_m128i: __m128i = _mm_set1_epi32(-1);

            // Pointer.
            let reg_ptr = & mut target_mem_m128i as *mut __m128i;

            // while core::arch::x86_64::_rdtsc() < mid     
            loop
            {
                if  core::arch::x86_64::_rdtsc() > mid 
                {
                    break;
                }
                // Set memory of SSE2 register pointer.
                _mm_stream_si128(reg_ptr, all_bytes_zero_m128i);
                _mm_stream_si128(reg_ptr, all_bytes_ffff_m128i);
            }
        
        // Thread sleep in Rust isn't very accurate, not even in the micro seconds region.    
        // thread::sleep(Duration::from_nanos(period));

            // Doing active wait, spinning, to get more accuracy.
            loop
            {
                if  core::arch::x86_64::_rdtsc() > mid + period / 2
                {
                    break;
                }
            }
        }

    }
}

fn square_am_signal(tx: & Sender<TimePair>, time: f64, in_frequency: i32)
{
    let frequency: f64 = in_frequency as f64; 

    println!("Playing {} seconds at {} Hz", time, frequency);
    
    let nsec_per_sec:f64 = 1_000_000_000.0;
    let period: f64  = nsec_per_sec / frequency;
    // println!("  period: {}", &period);

    // Nano second timer in Rust.    
    let mut start: u64;
    unsafe{
         start = core::arch::x86_64::_rdtsc();
     };
    // println!("  start: {}", start);
    
    let end: u64 = start + (time * nsec_per_sec) as u64;
    // println!("  end:   {}", end);
    // println!("  delta:        {}", end - start);

    // while core::arch::x86_64::_rdtsc() < end 
    
    loop
    {
        let val: u64;
        unsafe{
            val = core::arch::x86_64::_rdtsc();
        }
        if  val > end 
        {
                break;
        }
        // println!("  val:   {}", val);
    
        let mid: u64   = start + (period / 2.0) as u64 ;
        let reset: u64 = start + period as u64 ;

        let period_to_send: u64 = period as u64;
        let period_as_u64: u64 = period as u64;


        // Send to the other working threads.
        let time_pair = TimePair{
            mid: mid.clone(),
            period: period_to_send.clone(), // reset
        };
        tx.send(time_pair).unwrap();

        // Thread sleep in Rust isn't very accurate, not even in the micro seconds region.    
        // thread::sleep(Duration::from_nanos(period_as_u64));

        unsafe 
        {
            // Doing active wait, spinning, to get more accuracy.
            loop
            {
                if  core::arch::x86_64::_rdtsc() > mid + period_as_u64 / 2
                {
                    break;
                }
            }
        }

        start = reset;
    }
}

fn main() {
    println!("PC bus AM Radio emitter!");

    let mut handles = vec![];

    let (tx, rx): (Sender<TimePair>, Receiver<TimePair>) = mpsc::channel::<TimePair>();
//    for _ in 0..NTHREADS // 1 2
//    {
        // Currently only boosts with one thread.
        let handle = thread::spawn(move || {
            boost_song_3(rx);
        });
        handles.push(handle);
//    }

    loop
    {
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.400, 2349);
        square_am_signal(&tx, 0.400, 2093);
        square_am_signal(&tx, 0.400, 2349);
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.790, 2673);
        square_am_signal(&tx, 0.400, 2349);
        square_am_signal(&tx, 0.400, 2349);
        square_am_signal(&tx, 0.790, 2349);
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.400, 3136);
        square_am_signal(&tx, 0.790, 3136);
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.400, 2349);
        square_am_signal(&tx, 0.400, 2093);
        square_am_signal(&tx, 0.400, 2349);
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.400, 2349);
        square_am_signal(&tx, 0.400, 2349);
        square_am_signal(&tx, 0.400, 2673);
        square_am_signal(&tx, 0.400, 2349);
        square_am_signal(&tx, 0.790, 2093);
    }

}

