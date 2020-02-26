[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple HTTP-speaking Rust DLL. Pretty straightforward. Use it in other languages that support Foreign Function Interfaces.

Don't get alarmed by the strange commits, this repo was a testing ground for another project.

# How do I shot request ?

`request(char* method, char* hostent, char* content)` behaves pretty much the way you expect it to: `method` is the HTTP verb to use (it *theoretically* supports them all, but the frontend I made for it only allowed for GET and POST), `hostent` is the intended recipient of the request, and `content` is the request body, if there is any to be had.

This method returns a **c-string**, not a `string`-like object ! It's the caller's (aka: *you*, most likely) responsibility to create a `string` object out of it !

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
	printf("%s", request("GET", "http://www.example.com", ""));
	return 0;
}
```

Assuming you have the cargo-generated library (librsnet.so or rsnet.dll depending on your OS) located in the same folder as your experimental C program, build it thusly:

* Linux (or pretty much any \*NIX out there): `gcc -o nettest -L. -I. main.c -lrsnet`
* Windows: `C:\path\to\gcc.exe -o nettest.exe -L. -I. main.c -lrsnet`

To run it, on Linux you need to tell `ld` that the library is located in the same folder as the executable (Windows does this by default, if I remember correctly): `LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH ./nettest`

In other languages, I have no idea how you accomplish the same. Up to you to find out. 
