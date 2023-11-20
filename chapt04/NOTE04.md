# Head Aches

## How head Works

## Getting Started

The primitive **usize** is the pointer\-sized unsigned integer type, and its size varies from 4 bytes on a 32-bit operating system to 8 bytes on a 64-bit system. Rust also has an **isize** type, which is a pointer-sized *signed* integer, which you would need to represent negative numbers as the GNU version does. Since you only want to store positive numbers à la the BSD version, you can stick with an unsigned type. Note that Rust also has the types u32/i32 (unsigned/signed 32-bit integer)
and u64/i64(unsigned/signed 64-bit integer) if you want finer control over how large these values can be.


### Writing a Unit Test to Parse a Strings

### Converting Strings into Errors

### Defining the Arguments

### Processing the Input Files

### Reading Bytes Versus Characters

Before continuing, you should understand the difference between reading *bytes* and *characters* from a file. In the early 1960s, the American Standard Code for Information Interchange (ASCII, pronounced *as\-key*) table of 128 characters represented all possible text elements in computing. It takes only seven bits (2^7^ = 128) to represent this many characters. Usually a byte consists of eight bits, so the notion of byte and character were interchangeable.

Since the creation of Unicode (Universal Coded Character Set) to represent all the writing systems of the world (and even emojis),  some characters may require up to four bytes. The Unicode standard defines several ways to encode characters, including UTF\-8 (Unicode Transformation Format using eight bits).  

## Solution

### Reading a File Line by Line

### Preserving Line Endings While Reading a File

### Reading Bytes from a File

>The **take** method from the **std::io::Read** trait expects its argument to be the type **u64**, but I have a **usize**. I *cast* or convert the value using the **as keyword**.

As you saw in the case of selecting only part of a multibyte character, converting bytes to characters could fail because strings in Rust must be valid UTF-8. The **string::from_utf8 function** will return an **Ok** only if the string is valid, but **string::from_utf8_lossy** will convert invalid UTF-8 sequences to the *unknown* or *replacement* character.

### Printing the File Separators



## Going Further

Consider implementing how the GNU head handles numeric values with suffixes and negative values.
You could also add an option for selecting characters in addition to bytes.


## Summary
