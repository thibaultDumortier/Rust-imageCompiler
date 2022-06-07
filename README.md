# Rust-imageCompiler

## Description

As you may guess this is a small image compiler fully made using the Rust programming language. 
It is faster than its java counterpart taking around 8sec, while java takes around 13sec [java still needs to be optimised on some aspects].

## How to use

Put a directory or the files you want compiled in the "to_compile" folder.</br>
If you don't want to use the "to_compile" directory you can instead change the path in main (see comments)</br>
Then run `./target/release/rust_ic` this will simply run the program without having to compile it again.

## Future ideas

- Make it so that a user can simply use a egui GUI to select a directory to compile.
- Add a filterable compiler, that way one can simply get the compiled images they want.
- Add a decompiler with user given instructions using a rust trait

## How it works

The program is quite simple to understand.<br/>
\- The main file simply starts the compiler by giving it a path, the compiler then checks if the path that was given is in fact a directory, 
like asked.<br/>
\- If so then the compiler simply reads the files inside of the directory to determine the size of the compiled image.<br/>
\- Afterwards the program simply reads and copies images onto a canvas and saves the compiled image under the name of the directory.
