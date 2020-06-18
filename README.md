# Low Battery Alert
![Rust](https://github.com/athulp01/battery-alert/workflows/Rust/badge.svg)\
Send notification when battery falls below critical level.

## Installation Instructions
1. Make sure you have a notification daemon running.
2. `$ cargo build -release` or download the binary from release.
3. Modify the .service file to the binary path.
4. Copy the .service file to ~/.config/systemd/user
5. `$ systemctl enable bat-notify && systemctl start bat-notify`

## Screenshots
![](screenshot/10per.png)\
![](screenshot/20per.png)
