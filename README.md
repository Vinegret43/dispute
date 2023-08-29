
<div align="Center" display="inline-block">
<img src="assets/dispute.png" width="128px"/>
<div/>

# DiSPuTe

**The Damn Stylish Pomodoro Timer**

Designed to get better productivity while staying healthy and motivated

# Features
🎯 Simple and strict: there is no pause button or ability to skip a round.
Dispute contains only what you really need to actually get work done

✨ Clean, yet beautiful and unique UI design with animations and sound effects

🦀 <s>Written in Ru</s> Is this even a feature in the modern day?


# Installation
Note that Dispute will compile and work properly only on *NIX platforms
supporting XDG specification. Dispute also currently requires QT to be
installed on your system

To install dispute, run these commands in the following order:
```sh
git clone "https://github.com/Vinegret43/dispute"
cd dispute
# May take a while, so be patient
cargo build --release
# You don't have to copy it to ~/.local/bin, any directory which is in
# your $PATH should be fine
cp target/release/dispute ~/.local/bin
# Installs desktop files so you can launch Dispute from your app launcher
dispute --install
```
