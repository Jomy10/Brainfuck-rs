<p align="center">
  <img src="assets/logo.png" alt="BRAINFUCK"/>
</p>

# Rust Brainfuck interpreter

A brainfuck interpreter written in Rust. Fast and simple.

## Usage

```bash
bf file.bf
```

## Download
Download the interpreter [here](https://github.com/Jomy10/Brainfuck-rs/releases/tag/v0.1.0), or compile the compiler yourself 
for your own system using `cargo build --release`.

Then, either run it using `./bf`, or copy it to `/usr/local/bin` folder (or make a symlink).

<details>
  <summary>Click for full instructions in making a symlink to /usr/local/bin</summary>
    
**Linux & MacOS**
```bash
# Go to /usr/local/bin folder
cd /usr/local/bin

# Make a symlink
ln -s /path/to/bf bf

# Or, alternatively, move the binary to the folder
mv /path/to/bf bf
```

**Windows**
```PowerShell
CD \Windows\System32
mklink bf \path\to\bf
```
*NOTE: I have no experiene with Windows PowerShell, so do correct me if I'm wrong.*
</details>

## Contributing
Feel free to open an issue if you have found a bug. Open a pull request if you can fix it as well. 
Contributions are very welcome.

Also, if you have any questions, feel free to open an issue.

## License
[MIT license](LICENSE.txt)
