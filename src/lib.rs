// IO
use std::io::prelude::*;

// String manip
use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;

// Chrono
use std::time::Duration;

// Net libs
use std::net::SocketAddr;
use std::net::TcpStream;

// DNS
use dns_lookup::lookup_host;

#[no_mangle]
#[repr(C)]
pub struct Request {
    request: *mut i8,
    response: *mut i8,
}

#[no_mangle]
pub extern "C" fn ffi_request(method: *const c_char, hostent: *const c_char, content: *const c_char, agent: *const c_char) -> Request {
    let r_method = unsafe { CStr::from_ptr(method).to_str().unwrap() };
    let r_hostent = unsafe { CStr::from_ptr(hostent).to_str().unwrap() };
    let r_content = unsafe { CStr::from_ptr(content).to_str().unwrap() };
    let r_agent = unsafe { CStr::from_ptr(agent).to_str().unwrap() };

    let mut r = Request {
        request: CString::new("").unwrap().into_raw(),
        response: CString::new("").unwrap().into_raw()
    };

    // Is http:// prefix present ?
    let mut offset = 0;
    if r_hostent.contains("http://") {
        offset = 7;
    }
    else if r_hostent.contains("https://") {
        offset = 8;
    }

    // Split host, port, and resource
    let r_host: String;
    let mut r_port = 80;
    let mut r_resource = "/".to_string();
    
    if r_hostent.split_at(offset).1.contains(":") {
        let sep: Vec<_> = r_hostent.split_at(offset).1.split(":").collect();
        r_host = sep[0].to_string();
        r_port = sep[1].parse().unwrap();
    }
    else {
        if r_hostent.contains("/") {
            let sep: Vec<_> = r_hostent.split_at(offset).1.split("/").collect();
            r_host = sep[0].to_string();
        }
        else {
            r_host = r_hostent.split_at(offset).1.to_string();
        }
    }

    let sep: Vec<_> = r_hostent.split_at(offset).1.splitn(2, "/").collect();
    if sep.len() == 2 {
        r_resource.push_str(&sep[1]);
    }

    // Build the request string
    let mut req = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", r_method, r_resource, r_host);
    if r_agent != "" {
        req.push_str("User-Agent: ");
        req.push_str(&r_agent);
        req.push_str("\r\n");
    }
    if r_method == "POST" {
        req.push_str("Content-type: application/json\r\nContent-length: ");
        req.push_str(&r_content.len().to_string());
        req.push_str("\r\n");
    }
    req.push_str("Connection: close\r\n\r\n");

    if r_method == "POST" {
        req.push_str(r_content);
    }

    r.request = CString::new(req.clone()).unwrap().into_raw();

    // Establish connection
    let ips: Vec<std::net::IpAddr> = lookup_host(&r_host).unwrap();

    let addr = SocketAddr::new(ips[0], r_port);
    let sock = TcpStream::connect_timeout(&addr, Duration::new(30, 0));

    match sock {
        Ok(mut s) => {
            // Send data
            s.write(req.as_bytes()).expect("Could not write to socket");

            // Receive data
            let mut resp = String::new();
            s.read_to_string(&mut resp).expect("Could not read from socket");

            r.response = CString::new(resp).unwrap().into_raw();

            return r;
        },
        Err(e) => {
            let mut error = String::new();
            error.push_str("{");
            error.push_str("\"method\": \"");
            error.push_str(&r_method);
            error.push_str("\",");
            error.push_str("\"hostent\": \"");
            error.push_str(&r_host);
            error.push_str(":");
            error.push_str(&r_port.to_string());
            error.push_str("\",");
            error.push_str("\"error\": \"");
            error.push_str(&e.to_string());
            error.push_str("\"}");
            
            r.response = CString::new(error).unwrap().into_raw();

            return r;
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn freeresp (resp: *mut c_char) {
    if resp.is_null() { return };
    let _ = CString::from_raw(resp);
}
