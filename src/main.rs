use std::sync::{Arc,Mutex};
use std::thread;

use std::sync::atomic::{AtomicI16, Ordering};

const DIGITS_SUM_UPPER_BOUNDARY:i16 = 25;
const START_COORD:(i16, i16) = (1000, 1000);
const THREAD_LIMIT:i16 = 1000;

static GLOBAL_THREAD_COUNT: AtomicI16 = AtomicI16::new(0);

fn main() {
    let visited_coords: Arc<Mutex<Vec<(i16, i16)>>> =
	Arc::new(Mutex::new(vec!(START_COORD)));

    threads_in_all_directions(START_COORD, &visited_coords);

    let clone = Arc::clone(&visited_coords);
    let v = clone.lock().unwrap();
    println!("An ant can visit {} coords", v.len());
}

/// Using threading, send an ant to check coordinates in all 4 directions:
/// N, E, S, W
///
/// # Arguments
///
/// * `x` - A coordinate tuple (x0, x1) of an ant.  * `visited_coords`
/// - A reference to an ARC with a mutex with a list of visited
/// coordinates.
fn threads_in_all_directions(x: (i16, i16), visited_coords: &Arc<Mutex<Vec<(i16, i16)>>>)
{
    thread::scope(|s| {

	if GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) < THREAD_LIMIT {
	    s.spawn(move || {
		GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
		step_in_one_direction(go_n(x), visited_coords);
		GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
	    });
	} else {
	    step_in_one_direction(go_n(x), visited_coords);
	}

	if GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) < THREAD_LIMIT {
	    s.spawn(move || {
		GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
		step_in_one_direction(go_e(x), visited_coords);
		GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
	    });
	} else {
	    step_in_one_direction(go_e(x), visited_coords);
	}    

	if GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) < THREAD_LIMIT {
	    s.spawn(move || {
		GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
		step_in_one_direction(go_s(x), visited_coords);
		GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
	    });
	} else {
	    step_in_one_direction(go_s(x), visited_coords);
	}    

	if GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) < THREAD_LIMIT {
	    s.spawn(move || {
		GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
		step_in_one_direction(go_w(x), visited_coords);
		GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
	    });
	} else {
	    step_in_one_direction(go_w(x), visited_coords);
	}    

    });
}

/// Send an ant to check coordinates of one point in one direction.
///
/// # Arguments
///
/// * `x` - A coordinate tuple (x0, x1) of an ant.  * `visited_coords`
/// - A reference to an ARC with a mutex with a list of visited
/// coordinates.
fn step_in_one_direction(x: (i16, i16),
			 visited_coords: &Arc<Mutex<Vec<(i16, i16)>>>) {
    if is_coords_below_upper_boundary(x) {	
	let clone = Arc::clone(visited_coords);
	let mut v = clone.lock().unwrap();

	if !v.contains(&x) {
	    println!("Thread count:{}", GLOBAL_THREAD_COUNT.load(Ordering::SeqCst));
	    println!("Ant moves to: ({},{})", x.0, x.1);
	    v.push(x);
	    std::mem::drop(v); 
	    threads_in_all_directions(x, visited_coords);
	 }
     }
}

fn go_n(x: (i16, i16)) -> (i16, i16) {
    (x.0, x.1 + 1)
}

fn go_e(x: (i16, i16)) -> (i16, i16) {
    (x.0 + 1, x.1)
}

fn go_s(x: (i16, i16)) -> (i16, i16) {
    (x.0, x.1 - 1)
}

fn go_w(x: (i16, i16)) -> (i16, i16) {
    (x.0 - 1, x.1)
}

/// Returns a sum of digits of a number.
///
/// # Arguments
///
/// * `number` - A number constituent digits of which to add up.
fn calc_digits_sum(number: i16) -> i16 { 
    let mut sum = 0;
    let mut number = number;
    
    while number != 0 {
	sum += number % 10;
	number /= 10;
    }
    sum
}

/// Checks if a sum of coords x0 and x1 of an any is below the upper
/// boundary.
///
/// # Arguments
///
/// * `x` - A coordinate tuple (x0, x1) of an ant.
fn is_coords_below_upper_boundary(x: (i16,i16)) -> bool {
    let sum_x_0 = calc_digits_sum(x.0);
    let sum_x_1 = calc_digits_sum(x.1);
    if sum_x_0 + sum_x_1 > DIGITS_SUM_UPPER_BOUNDARY {
	return false;
    }
    true
}
