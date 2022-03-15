# Description
`image-utility` is a light and easy-to-use cli tool to do some basic image-processing like 
resize, rotate, blur without the need to open a heavy software like GIMP, I decided
to make the program the more intuitive possible so unlike some other cli tools.
This one will ask the parameters of the action you want to do after you have selected.

What does it mean? It means that you don't have to type a long line like `image-utility path path2 resize dim1 dim2`
It will ask for more information once you have selected an action so you don't have to remember a lot of things to use this program
There is also a link at the end when the processing is done to rapidly view the result of the operation
Can generate two types of histogram (rgb or gray).

*To do the processing part I use (except histogram): [image](https://github.com/image-rs/image)*

*For the cli part : [clap](https://github.com/clap-rs/clap), [anyhom](https://github.com/dtolnay/anyhow), [indicatif](https://github.com/console-rs/indicatif)*


## Possible Actions
Here the list of all the action currently implemented:

| **Name**        | **Description**                                                                                                        | 
|-----------------|------------------------------------------------------------------------------------------------------------------------|
| **blur**        | perform a Gaussian blur with a sigma value who determined how much to blur it                                          |
| **resize**      | resize a image without preserving the ratio at the new width and height                                                |
| **resizeratio** | resize a image and preserve the ratio at the new width and height                                                      |
| **grayscale**   | return the grayscale of the image (only gray use)                                                                      |
| **contrast**    | adjust the contrast by taking a value. Negative reduces the contrast positive increase it                              |
| **brighten**    | take a value it will be the value added to every color of the pixel (positive increase brightness / negative decrease) |
| **rotate90**    | rotate 90° clockwise                                                                                                   |
| **rotate180**   | rotate 180° clockwise                                                                                                  |
| **rotate270**   | rotate 270° clockwise                                                                                                  |
| **flipv**       | flip the image vertically                                                                                              |
| **fliph**       | flip the image horizontally                                                                                            |
| **histogram**   | create the histogram of the image, `gray` parameter does the average of the RGB, `rgb` do 3 curves for each color      |

# Installation
## With Rust
install [rust]("https://www.rust-lang.org/learn/get-started")
run `cargo run -- --help` on the root of the project

## As a binary

You have a binary file who contains the program so you don't need to
have rust or anything else
### Linux
You can also use it as a binary with the file `image-utility` in `install`
Run it by doing `./install/image-utility --help` at the root of the project

### Window
You have the file `image-utility.exe` in `install`
at the root of the project run from cmd `start install/image-utility.exe %help%`

### Debian
You can install it like a Debian package double click the file `image-utility_0.1.0_amd64.deb` in `install` and follow the step or
run in the root of the project `sudo dpkg -i install/image-utility_0.1.0_amd64.deb`
to verify that the installation work correctly, do `image-utility --help`

# Example result
These will use Lenna

![lena](display.png) 

