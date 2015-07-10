extern crate libc;
use std::ptr;
use std::ffi::CString;
use std::ffi::CStr;
use std::mem;
use std::boxed;
use std::cmp::Ordering;
use std::num;

pub type Lv2handle = *mut libc::c_void;

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
    amp_uri: *const  libc::c_char,
    instantiate: extern fn(descriptor: *const LV2Descriptor, rate:  &mut f64,
                            bundle_path: *const u8, features: *const LV2Feature)
                                -> Lv2handle,
    connect_port: extern fn(handle: Lv2handle, port: PortIndex, data: *mut libc::c_void),
    activate: extern fn(instance: Lv2handle),
    run: extern fn(instance: Lv2handle, n_samples: u32),
    deactivate: extern fn(instance: Lv2handle),
    cleanup: extern fn(instance: Lv2handle),
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
                                -> Lv2handle {
                                let ptr: *mut libc::c_void;
                                unsafe{
                                    ptr = libc::malloc(mem::size_of::<Amp>() as libc::size_t) as *mut libc::c_void;
                                }
                                return ptr;
    }
    pub extern fn connect_port(handle: Lv2handle, port: PortIndex, data: *mut libc::c_void) {
        let amp: *mut Amp = handle as *mut Amp;
        match port {
            PortIndex::AmpGain => {println!("Connecting gain");
                unsafe{ (*amp).gain = data  as *const f32 };}, // data may be NULL pointer, so don't dereference!
            PortIndex::AmpInput => {println!("Connecting input");
                unsafe{ (*amp).input = data as *const f32 };},
            PortIndex::AmpOutput => {println!("Connecting output");
                unsafe{ (*amp).output = data as *mut f32 };},
        }
    }
    pub extern fn activate(instance: Lv2handle) {}
    pub extern fn run(instance: Lv2handle, n_samples: u32) {
        let amp = instance as *const Amp;
        let gain = unsafe{ *((*amp).gain) };
        let input: *const f32 = unsafe{  (*amp).input };
        let output: *mut f32 = unsafe{ (*amp).output };

        println!("gain: {}", gain);
        println!("input: {}", unsafe{  *input } );

        let mut coef:  f32;
        match gain > -90.0 {
            true    => {println!("setting coef"); coef =(10.0 as f32).powf(gain*0.05);},
            false => {println!("setting coef to zero"); coef = 0.0;}
        }
        println!("coef: {}", coef );

        unsafe{
            for x in 0..n_samples-1 {
                *output.offset(x as isize) = *input.offset(x as isize) * coef;
            }
        }
        println!("output: {}", unsafe{  *output } );
    }

    pub extern fn deactivate(instance: Lv2handle) {}
    pub extern fn cleanup(instance: Lv2handle) {

        unsafe{
            //ptr::read(instance as *mut Amp); // no need for this?
            libc::free(instance  as Lv2handle)
        }
    }
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
        //static ff: *const libc::c_char = (b"http://example.org/eg-amp_rust\n\0").as_ptr() as *const libc::c_char;
        //static asa: *const libc::c_char = std::ffi::CStr::from_ptr(ff);

        let s = "http://example.org/eg-amp_rust";
        let cstr = CString::new(s).unwrap();
        let ptr = cstr.as_ptr();
        mem::forget(cstr);
        let ff = "hoit";

        //static gr: &'static LV2Descriptor = &LV2Descriptor{amp_uri: "hoit",
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
