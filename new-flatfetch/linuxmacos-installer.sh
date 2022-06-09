#!/usr/bin/sh

noteify () {
echo "IMPORTANT NOTICE: THE FOLLOWING DEPENDENCIES ARE REQUIRED: "
echo "Rust (Cargo, rustc, rustup, etc.) and xz"
echo "if there is an error, you probably dont have one of these installed."
}

install () {
echo "Building..."
cargo build
echo "enter password:"
sudo mkdir /usr/local/bin
sudo mv target/debug/flatfetch /usr/local/bin
echo "moved flatfetch into local bin directory."
echo "installing man page..."
sudo mkdir /usr/local/man
sudo mkdir /usr/local/man/man1
sudo mv flatfetch.roff /usr/local/man/man1/
sudo xz --compress /usr/local/man/man1/flatfetch.roff
echo "man page installed and compressed to '/usr/local/man/man1/flatfetch.roff"
echo "Refreshing man database..."
mandb
echo Finshed.
echo "Thank you for installing flatfetch! You can now delete this file." 
}

noteify

while true; do
    read -p "Do you wish to install flatfetch? [Y/n] " yn
    case $yn in
        [Yy]* ) install; break;;
        [Nn]* ) echo "cancled" && exit;;
        * ) echo "Please answer yes or no.";;
    esac
done
