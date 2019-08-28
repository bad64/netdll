// String manip
use std::os::raw::c_char;
use std::ffi::CStr;
use std::str;

// Net libs
use std::net::{SocketAddrV4, Ipv4Addr, ToSocketAddrs};
use std::net::TcpStream;

#[no_mangle]
pub extern "C" fn request(method: *const c_char, hostent: *const c_char, content: *const c_char) {
    let r_method = unsafe { CStr::from_ptr(method).to_str().unwrap() };
    let mut r_hostent = unsafe { CStr::from_ptr(hostent).to_str().unwrap() };
    let r_content = unsafe { CStr::from_ptr(content).to_str().unwrap() };

    // Is http:// prefix present ?
    let mut offset = 0;
    if r_hostent.contains("http://") {
        offset = 7;
    }
    r_hostent = r_hostent.split_at_mut(offset).1;


    // Split host, port, and resource
    let mut host = "".to_string();
    let mut port = 80;
    let mut resource = "/".to_string();
    
    if r_hostent.contains(":") {
        let sep: Vec<_> = r_hostent.clone().split(":").collect();
        host = sep[0].to_string();
        port = sep[1].parse().unwrap();
    }
    else {
        let sep: Vec<_> = r_hostent.clone().split("/").collect();
        host = sep[0].to_string();
    }

    let sep: Vec<_> = r_hostent.clone().splitn(2, "/").collect();
    if sep.len() == 2 {
        resource.push_str(&sep[1]);
    }
}
