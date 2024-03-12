# Image to ASCII Converter

This is a simple command-line tool written in Rust for converting images to ASCII art.

## Prerequisites

- Rust compiler (https://www.rust-lang.org/tools/install)
- Cargo package manager (included with Rust)

## Installation

1. Clone the repository:
   git clone https://github.com/your-username/your-repository.git
2. Navigate to the project directory:
   cd image-to-ascii
3. Build the project using Cargo:
   cargo build --release

## Usage

run the following command:
  cargo run --release -- <input_image_path> <resolution>
NOTE: Replace <input_image_path> with the path to your input image and <resolution> with the desired resolution for the ASCII art. 
IMPORTANT: The resolution parameter represents the divisor of the original image's dimensions. So, when you input a smaller value, such as 1, you effectively divide the original dimensions by 1, resulting in the original size. I recommend choosing a value between 1 and 10(depending on your image's resolution).

The ASCII art will be saved to a file named output.txt in the current directory.
