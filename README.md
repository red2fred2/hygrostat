# This contains the code and design files for a very small hygrostat system

This system is being built to keep a constant humidity in a very small cheese cave, but should be usable for other small areas.

The electronic design files are in the electronics folder, mechanical in the
mechanical folder, and software in the src folder.

To load new code to the device:

0. [Install](https://www.rust-lang.org/tools/install) Rust.
1. Run ```cargo run --release``` to generate a .uf2 file for flashing. This
can be found in the ```target > thumbv6m-none-eabi > release``` folder.
2. Short the test pads on the board on the edge opposite the big capacitor.
3. Plug the board into your computer with a USB cord.
4. Stop shorting the test pads.
5. The board should show up as a flash drive. Drag and drop the .uf2 file in.
6. It will disconnect from the computer and reboot with the new code.

## Hygrostat controller
![probe render](https://raw.githubusercontent.com/red2fred2/hygrostat/master/electronics/hygrostat-controller/render.png)

## Hygrometer probe
![probe render](https://raw.githubusercontent.com/red2fred2/hygrostat/master/electronics/hygrometer-probe/render.png)
