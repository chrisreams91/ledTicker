# VM setup

### Vagrant setup


vagrant init debian/stretch64

vagrant up

vagrant ssh

sudo apt update
sudo apt upgrade

### Install Rust 

sudo apt install gcc

sudo apt install curl

curl https://sh.rustup.rs -sSf | sh

source $HOME/.cargo/env

rustup default nightly

sudo apt-get install -qq gcc-arm-linux-gnueabihf

rustup target add armv7-unknown-linux-gnueabihf

### Config cargo for cross compile 

mkdir -p ~/.cargo

cat >>~/.cargo/config <<EOF

[target.armv7-unknown-linux-gnueabihf]

linker = "arm-linux-gnueabihf-gcc"

EOF


# Sending files

### Mac to vagrant 

vagrant scp /Users/chris/Documents/Code/ledTicker :/home/vagrant/ledTicker

cargo build --target=armv7-unknown-linux-gnueabihf —release


### Binary from VM to pi 

sftp pi@192.168.1.133 ( from VM )

put /home/vagrant/ledTicker/target/armv7-unknown-linux-gnueabihf/release/ledTicker /home/pi/ledTickerAndLights


### - images / gifs / fonts

sftp pi@192.168.1.133

put /Your/path/to/file/ /home/pi/(gifs/images/fonts)

