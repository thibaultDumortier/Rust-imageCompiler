# Rust-imageCompiler

## Description

As you may guess this is a small image compiler fully made with using the Rust programming language. 
It is faster than its java counterpart taking around 8sec, while java takes around 13sec.

## How to use

Simply open main and replace `"YOUR PATH HERE"` by whatever your path is: for example "./dir_name/".<br/>
Then use the command `cargo run` and the program will do the rest.

## Future ideas

- Make it so that a user can simply use a egui GUI to select a directory to compile.
- Add a filterable compiler, that way one can simply get the compiled images they want.

## How it works

The program is quite simple to understand.<br/>
The main file simply starts the compiler by giving it a path, the compiler then checks if the path that was given is in fact a directory, 
like asked.<br/>
If so then the compiler simply reads the files inside of the directory to determine the size of the compiled image.<br/>
Afterwards the program simply reads and copies images onto the canvas.
