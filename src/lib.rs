extern crate libc;
use std::ptr;
use std::ffi::CString;
use std::mem;
use std::boxed;
use std::cmp::Ordering;
use std::num;

pub enum PortIndex {
        AmpGain = 0,
        AmpInput= 1,
        AmpOutput = 2
}

#[repr(C)]
pub struct LV2Feature {
    uri: *const u8,
    data: *mut libc::c_void
}


#[repr(C)]
pub struct LV2Descriptor {
    amp_uri: *const libc::c_char,
    instantiate: extern fn(descriptor: *const LV2Descriptor, rate:  &mut f64,
                            bundle_path: *const u8, features: *const LV2Feature)
                                -> *mut libc::c_void,
    connect_port: extern fn(lv2handle: *mut libc::c_void, port: PortIndex, data: *mut libc::c_void),
    activate: extern fn(instance: *const libc::c_void),
    run: extern fn(instance: *const libc::c_void, n_samples: u32),
    deactivate: extern fn(),
    cleanup: extern fn(),
    extension_data: extern fn(uri: *const u8)-> (*const libc::c_void),
}

#[repr(C)]
struct Amp {
    gain: *const f32,
    input: *const f32,
    output: *mut f32
}


impl LV2Descriptor {
    pub extern fn instantiate(descriptor: *const LV2Descriptor, rate: &mut f64, bundle_path: *const u8, features: *const LV2Feature)
                                -> *mut libc::c_void {
//                                 let mut x1 = Box::new(0.5 as f32);
//                                 let mut x2 = Box::new(0.5 as f32);
//                                 let mut x3 = Box::new(0.5 as f32);
//                                 let x1p= &mut *x1 as *const f32;
//                                 let x2p= &mut *x2 as *const f32;
//                                 let x3p= &mut *x3 as *mut f32;
//                                 let mut myamp = Box::new(Amp{gain: x1p, input: x2p ,output:  x3p});
// //                                 mem::forget(x1p);
// //                                 mem::forget(x2p);
// //                                 mem::forget(x3p);
                                let mut myamp = Box::new(Amp{gain: &(0.5 as f32), input: &(0.0 as f32),output:  &mut (0.0 as f32)});

                                let ptr = (&mut *myamp as *mut Amp) as *mut libc::c_void;
                                mem::forget(myamp);
                                //let mut amp = Amp{gain: &0.0, input: &0.0,
                                 //output:  &mut 0.0};
                                //let ptr= (&mut amp as *mut Amp) as *mut libc::c_void;
                                return ptr;
    }
    pub extern fn connect_port(lv2handle: *mut libc::c_void, port: PortIndex, data: *mut libc::c_void) {
        let amp: *mut Amp = lv2handle as *mut Amp;
        match port {
            PortIndex::AmpGain => {println!("gain");
                unsafe{ (*amp).gain = data  as *const f32 };}, // data may be NULL pointer, so don't dereference!
            PortIndex::AmpInput => {println!("inp");
                unsafe{ (*amp).input = data as *const f32 };},
            PortIndex::AmpOutput => {println!("outp");
                unsafe{ (*amp).output = data as *mut f32 };},
        }
    }
    pub extern fn activate(instance: *const libc::c_void) {}
    pub extern fn run(instance: *const libc::c_void, n_samples: u32) {
        let amp = instance as *const Amp;
        let gain = unsafe{ *((*amp).gain) };
        println!("gain: {}", gain);
        let input: *const f32 = unsafe{  (*amp).input };
        let output: *mut f32 = unsafe{ (*amp).output };
        println!("input: {}", unsafe{  *input } );
        println!("input_pointer: {}", unsafe{ *input.offset(0) * 3.0});
        println!("output: {}", unsafe{  *output } );
        let mut coef:  f32;
        match gain > -90.0 {
            true    => {println!("setting coef"); coef =(10.0 as f32).powf(gain*0.05);},
            false => {println!("setting coef to zero"); coef = 0.0;}
        }
        println!("coef: {}", coef );
        // let vec: Vec<isize> = Vec::with_capacity((n_samples-1 as usize));
        // for i in 0..n_samples-1 {
        //     vec.push(i);
        // }
        unsafe{
            // for x in vec.iter() {
            for x in 0..n_samples-1 {
                //println!("ptr index {}", x); // x: i32
                *output.offset(x as isize) = *input.offset(x as isize) * coef;
            }
        }
        println!("output_new: {}", unsafe{  *output } );
    }

    pub extern fn deactivate() {}
    pub extern fn cleanup() {}
    pub extern fn extension_data(uri: *const u8)-> (*const libc::c_void) {
                            ptr::null()
    }
}

#[no_mangle]
pub extern fn lv2_descriptor(index:i32) -> *const LV2Descriptor {
    if index != 0 {
        return ptr::null();
    } else {
        println!("called lv2_descriptor");
        let s = "http://example.org/eg-amp_rust";
        let cstr = CString::new(s).unwrap();
        let ptr = cstr.as_ptr();
        mem::forget(cstr);

        let gr = Box::new(LV2Descriptor{amp_uri: ptr,
                                  instantiate: LV2Descriptor::instantiate,
                                  connect_port: LV2Descriptor::connect_port,
                                  activate: LV2Descriptor::activate,
                                  run: LV2Descriptor::run,
                                  deactivate: LV2Descriptor::deactivate,
                                  cleanup: LV2Descriptor::cleanup,
                                  extension_data: LV2Descriptor::extension_data
                                  });


        let hoit = &*gr as *const LV2Descriptor; // see https://doc.rust-lang.org/std/ptr/
        mem::forget(gr);
        return  hoit
    }
}
