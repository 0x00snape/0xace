# 0xace
0xace is a simple file obfuscater/deobfuscater with simple technique but with different approach in rustlang.

## How obfuscation works in 0xace?
0xace uses two different techinuque:<br> 
Firstly encoding the data using two different types of characters, spaces(\x20) and newlines(\x0A), secondly encrypted data is XORed by randomly generated key. 
<br><img src="https://github.com/0x00snape/0xace/blob/main/src/oxace.png" style="max-width: 100%;" width="240" align="center">

## Usage:
```bash
$ git clone https://github.com/0x00snape/0xace.git
$ cd 0xace
$ cargo build --release
```
## License
This project is licensed under [MIT](https://github.com/0x00snape/0xace/blob/main/LICENSE)

