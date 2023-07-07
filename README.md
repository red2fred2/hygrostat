# This contains the code and design files for a very small hygrostat system

The electronic design files are in the electronics folder, mechanical in the
mechanical folder, and software in the src folder.

To build, make sure rust is [installed](https://www.rust-lang.org/tools/install)
then run ```cargo run --release``` and find the .uf2 file in the
```target > thumbv6m-none-eabi > release``` folder.

To deploy, plug in the controller board to the computer with the USB cord with
the boot pads shorted. The board should show up as an external drive if this was
done correctly. Just put the .uf2 file on the drive and it will disconnect after
it loads.

## Hygrometer probe
![probe render](https://raw.githubusercontent.com/red2fred2/hygrostat/master/electonics/hygrometer-probe/render.png)