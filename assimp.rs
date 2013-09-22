#[link(name = "assimp",
	vers = "0.1",
	uuid = "9fd3d600-20b0-11e3-8224-0800200c9a66",
	author = "Tomasz Stachowiak",
	url = "https://github.com/h3r2tic/assimp-rs")];

#[comment = "Bindings and wrapper functions for AssImp."];
#[crate_type = "lib"];

// TODO: Document differences between GLFW and glfw-rs

//use std::libc::*;
use std::ptr;
//use std::str;
//use std::vec;

// re-export constants
pub use consts::*;

pub mod ffi;
pub mod consts;
//mod private;


#[deriving(Eq, IterBytes)]
pub struct Scene {
    ptr: *ffi::aiScene,
}


impl Scene {
	#[fixed_stack_segment] #[inline(never)]
	pub fn load( filename: &str, flags: u32 ) -> Result<Scene, ~str> {
		unsafe {
			do filename.with_c_str |fname| {
				ffi::aiImportFile( fname, flags )
					.to_option().map_default(Err( ~"aiImportFile returned null" ),
						| &ptr | Ok(
							Scene { ptr: ptr::to_unsafe_ptr( ptr ) }
						)
					)
			}
		}
	}
}