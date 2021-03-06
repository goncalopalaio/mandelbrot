use std::str::FromStr;
use std::fs::File;

extern crate num;
use num::Complex;
extern crate image;
use image::ColorType;
use image::png::PNGEncoder;

/**
Transcribed from the Programming Rust book by Jim Bandly and Jason Orendorff
**/

fn main() {
    println!("Hello, world!");
    
    /*
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
    	writeln!(std::io::stderr(), "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT").unwrap();

    	std::process::exit(1);
    }*/

    let file = "../mandel.png";
    let pixels = "1000x750";
    let un_upper_left = "-1.20,0.35";
    let un_lower_right = "-1.0,0.20";


   	let bounds = parse_pair::<usize>(pixels, 'x').expect("Error parsing pixels");
   	let upper_left = parse_complex(un_upper_left).expect("Error parsing upper left");
   	let lower_right = parse_complex(un_lower_right).expect("Error parsing lower right");

   	let mut pixels = vec![0; bounds.0 * bounds.1];

   	render(&mut pixels, bounds, upper_left, lower_right);

   	write_image(&file, &pixels, bounds).expect("Error writing PNG file");

}

fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>){
	assert!(pixels.len() == bounds.0 * bounds.1);

	for row in 0..bounds.1 {
		for column in 0..bounds.0 {
			let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
			pixels[row * bounds.0 + column] = match escape_time(point, 255) {
				None => 0,
				Some(count) => 255 - count as u8
			}
		}
	}
}

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64>{
	let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
	Complex {
		re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
		im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
	}
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
	let mut z = Complex {re: 0.0, im: 0.0};

	for i in 0..limit {
		z = z * z + c;
		if z.norm_sqr() > 4.0 {
			return Some(i);
		}
	}

	None
}

fn parse_complex(s: &str) -> Option<Complex<f64>>{
	match parse_pair(s, ',') {
		Some((re, im)) => Some(Complex {re, im}),
		_ => None
	}
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
	match s.find(separator) {
		None => None,
		Some(index) => {
			match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
				(Ok(l), Ok(r)) => Some((l,r)),
				_ => None
			}
		}
	}
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize,usize)) -> Result<(), std::io::Error> {
	let output = File::create(filename)?;

	let encoder = PNGEncoder::new(output);

	encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

	Ok(())
}

#[test]
fn test_parse_pair() {
	assert_eq!(parse_pair::<i32>("", ','), None);
	assert_eq!(parse_pair::<i32>("10", ','), None);
	assert_eq!(parse_pair::<i32>("10x10", 'x'), Some((10,10)));
	assert_eq!(parse_pair::<f64>("10.1x10.2", 'x'), Some((10.1,10.2)));
}
