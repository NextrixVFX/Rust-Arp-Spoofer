# Rust-Arp-Spoofer
Simple ARP Spoofer written in Rust.
Supports multiple targets.
### Requirements
- Npcap [https://npcap.com/dist/npcap-1.79.exe]
- Npcap SDK [https://npcap.com/dist/npcap-sdk-1.13.zip]
- Windows
### Usage:
- Put the ```/Lib/``` and ```/Include/``` Directories from the Npcap SDK in your Npcap install directory. (C:/Program Files/Npcap/)
![image](https://github.com/user-attachments/assets/68822b2d-6d90-4235-be96-10a5702dada5)
- Add the path ```C:/Program Files/Npcap/Lib/``` (x86) or ```C:/Program Files/Npcap/Lib/x64``` (x64) to your envoirment variables.
![image](https://github.com/user-attachments/assets/bc545f97-2c1b-4b84-8a8a-bd6201feaf67)
- Set your targets and put your Gateway + Mac Address
![image](https://github.com/user-attachments/assets/690cc8b7-3221-45b3-bfd4-edea049ac129)
- Set your NIC
![image](https://github.com/user-attachments/assets/6c98c08d-9539-4e80-ad4b-3f97a7140571)
- Run with ```cargo run```
