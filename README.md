#Tostiloco

_Another_ chip8 emulator in Rust. Currently compiles to WebAssembly.

Graphics are rendered with webGL and eventually perhaps a desktop
opengl or metal version if I don't get bored. 
(UPDATE: I'm working on an NES emulator so will not continue to update this)

##How to Run

##What works

Most chip8 roms seem to work. So does user input and sound via WebAudio API using an oscillator for a single tone.
Most everything (including **DRW**!) is unit tested.

##What doesn't work

Superchip8 rom support has not been implemented. I also wanted to write a compiler and memory dumps during tick-by-tick execution but probably
won't get around to it. Didn't unit test the web code due to not wanting to deal with
writing `wasm_bindgen` tests; CPU at least has full test coverage.

##Thanks

The following resources were immensely helpful in putting together this project:

###Cowgod's guide

http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

Absolutely indispensable! Most other tutorials droned on a bit too long, not this one; just give me the instruction list
and tell me exactly what they do. I really only used other guides if something was unclear (looking at you **DRW**)

###Tobias Langhoff's guide

https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

The perfect guide to fill in all the details, helped me get the **DRW** instruction to really click. Also fantastic
explanations on fonts and input.

###WebGL2 Fundamentals

https://webgl2fundamentals.org

Helped me begin to learn WebGL. I had to  take a step back and really try and understand this. I regret not starting with
a simpler 2D canvas API first instead of diving into WebGL. But taking the time to follow this first helped me get my
poor WebGL implementation for drawing pixels working. 