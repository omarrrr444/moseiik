use image::io::Reader as ImageReader;
use std::error::Error;
use image::RgbImage;


#[cfg(test)]
mod tests {
    use super::*;

    fn open(path: &str) -> Result<RgbImage, Box<dyn Error>> {
    	
    	let mut image = path;
    	Ok(ImageReader::open(&image)?.decode()?.into_rgb8())
    } 
    
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // test avx2 or sse2 if available
        
        use moseiik::main::compute_mosaic;
        use moseiik::main::Options;
        
        let args = Options {
        	image: "./assets/kit.jpeg".to_string(),
        	output: "./assets/out_test.png".to_string(),
        	tiles: "./assets/images/".to_string(),
        	scaling: 1,
        	tile_size: 25,
        	remove_used: true,
        	verbose: false,
        	simd: false,
        	num_thread: 1,
        };
        
        compute_mosaic(args);
        
        let target_path = "./assets/x86-ground-truth.png";
        let real_path = "./assets/out_test.png";
        
        let target = open(&target_path);
        let real = open(&real_path);
        
        match target {
        	Ok(vec) => {
        			match real {
        				Ok(vec2) => {
        					assert_eq!(vec, vec2);
        				}
        				Err(e) => {
    						assert!(false);
    					}
        			}
        	}
        	Err(e) => {
    			assert!(false);
    		}
        }
    }
    
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86_simd() {
        // test avx2 or sse2 if available
        
        use moseiik::main::compute_mosaic;
        use moseiik::main::Options;
        
        let args = Options {
        	image: "./assets/kit.jpeg".to_string(),
        	output: "./assets/out_test_simd.png".to_string(),
        	tiles: "./assets/images/".to_string(),
        	scaling: 1,
        	tile_size: 25,
        	remove_used: true,
        	verbose: false,
        	simd: true,
        	num_thread: 1,
        };
        
        compute_mosaic(args);
        
        let target_path = "./assets/x86-ground-truth.png";
        let real_path = "./assets/out_test.png";
        
        let target = open(&target_path);
        let real = open(&real_path);
        
        match target {
        	Ok(vec) => {
        			match real {
        				Ok(vec2) => {
        					assert_eq!(vec, vec2);
        				}
        				Err(e) => {
    						assert!(false);
    					}
        			}
        	}
        	Err(e) => {
    			assert!(false);
    		}
        }
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
    
    	use moseiik::main::compute_mosaic;
        use moseiik::main::Options;
        
        let args = Options {
        	image: "./assets/kit.jpeg".to_string(),
        	output: "./assets/out.png".to_string(),
        	tiles: "./assets/images/".to_string(),
        	scaling: 1,
        	tile_size: 25,
        	remove_used: true,
        	verbose: false,
        	simd: false,
        	num_thread: 1,
        };
        
        compute_mosaic(args);
        
        let target_path = "./assets/ground-truth-kit.png";
        let real_path = "./assets/out.png";
        
        let target = open(&target_path);
        let real = open(&real_path);
        
        match target {
        	Ok(vec) => {

        			match real {
        				Ok(vec2) => {
        					assert_eq!(vec, vec2);
        				}
        				Err(e) => {
    						assert!(false);
    					}
        			}
        	}
        	Err(e) => {
    			assert!(false);
    		}
        }
    }
    
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64_simd() {
    
    	use moseiik::main::compute_mosaic;
    	use moseiik::main::Options;

        let args = Options {
        	image: "./assets/kit.jpeg".to_string(),
        	output: "./assets/out-simd.png".to_string(),
        	tiles: "./assets/images/".to_string(),
        	scaling: 1,
        	tile_size: 25,
        	remove_used: true,
        	verbose: false,
        	simd: true,
        	num_thread: 1,
        };
        
        compute_mosaic(args);
        
        let target_path = "./assets/ground-truth-kit.png";
        let real_path = "./assets/out-simd.png";
        
        let target = open(&target_path);
        let real = open(&real_path);
        
        match target {
        	Ok(vec) => {
        			match real {
        				Ok(vec2) => {
        					assert_eq!(vec, vec2);
        				}
        				Err(e) => {
    						assert!(false);
    					}
        			}
        	}
        	Err(e) => {
    			assert!(false);
    		}
        }
    }
}
