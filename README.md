[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

 A simple HTTP-speaking Rust DLL. Pretty straightforward. Use it in other languages that support Foreign Function Interfaces.

# How do I shot request ?

`request(char* method, char* hostent, char* content, char* agent)` behaves pretty much the way you expect it to: `method` is the HTTP verb to use, `hostent` is the intended recipient of the request, `content` is the request body, if there is any to be had, and `agent` is the user-agent string if you want or need to impersonate a browser in particular.

This method returns a `struct` containing two `char` arrays: one for the outgoing request, and one for the incoming response. Both should normally include their respective HTTP headers.

# How do I start with FFI ?

Depends on your language really. In C/C++ it's fairly simple: create a header file that defines an extern function looking like the one supposed to be in the DLL (in this case, just look above for the prototype of `request`), include it in your main program, and call it. So in our case, in C:

```
/* netdll.h */
#ifndef NETDLL_H
#define NETDLL_H
extern char* request(char* method, char* hostent, char* content);
#endif
/* main.c or whatever */
#include <stdio.h>
#include "netdll.h"
int main (int argc, char** argv)
{
	printf("%s", request("GET", "http://www.example.com", "", "definitely-legit-user-agent 4.20"));
	return 0;
}
```

Assuming you have the cargo-generated library (librsnet.so or rsnet.dll depending on your OS) located in the same folder as your experimental C program, build it thusly:

* Linux (or pretty much any \*NIX out there): `gcc -o nettest -L. -I. main.c -lrsnet`
* Windows: `C:\path\to\gcc.exe -o nettest.exe -L. -I. main.c -lrsnet`

To run it, on Linux you need to tell `ld` that the library is located in the same folder as the executable (Windows does this by default, if I remember correctly): `LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH ./nettest`

In other languages, I have no idea how you accomplish the same. Up to you to find out. 
