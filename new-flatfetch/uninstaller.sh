#!/usr/bin/sh

uninstall () {
echo "shreding flatfetch binary..."
echo "enter password: "
sudo shred -u /usr/local/bin/flatfetch
echo "shreding man page..."
sudo shred -u /usr/local/man/man1/flatfetch.roff
echo "refreshing mandb..."
mandb
echo "flatfetch has sucsessfully been deleted."
}

while true; do
    read -p "Do you wish to uninstall flatfetch? [Y/n] " yn
    case $yn in
        [Yy]* ) uninstall; break;;
        [Nn]* ) echo "cancled" && exit;;
        * ) echo "Please answer yes or no.";;
    esac
done
