#install requirements

#check node.js
if ! command -v node &> /dev/null
then
packagesNeeded='nodejs'
if [ -x "$(command -v apk)" ];       then sudo apk add --no-cache $packagesNeeded
elif [ -x "$(command -v apt-get)" ]; then sudo apt-get install $packagesNeeded
elif [ -x "$(command -v dnf)" ];     then sudo dnf install $packagesNeeded
elif [ -x "$(command -v zypper)" ];  then sudo zypper install $packagesNeeded
else echo "FAILED TO INSTALL PACKAGE: Package manager not found. You must manually install: $packagesNeeded">&2; fi
fi

#check curl
if ! command -v curl &> /dev/null
then
packagesNeeded='curl'
if [ -x "$(command -v apk)" ];       then sudo apk add --no-cache $packagesNeeded
elif [ -x "$(command -v apt-get)" ]; then sudo apt-get install $packagesNeeded
elif [ -x "$(command -v dnf)" ];     then sudo dnf install $packagesNeeded
elif [ -x "$(command -v zypper)" ];  then sudo zypper install $packagesNeeded
else echo "FAILED TO INSTALL PACKAGE: Package manager not found. You must manually install: $packagesNeeded">&2; fi
fi



INSTALL_DIR=/home/`whoami`/.RedditWallPaper

rm -rf ${INSTALL_DIR}
mkdir  ${INSTALL_DIR}

chmod +x src/*

cp -r src/* ${INSTALL_DIR}
chmod +x /home/`whoami`/.RedditWallPaper/*
mkdir /home/`whoami`/.RedditWallPaper/pics
# add to path
echo "PATH=""${INSTALL_DIR}:"\$"PATH">> /home/`whoami`/.bashrc
export PATH=${INSTALL_DIR}:$PATH
echo "installation complete"
echo "1)to select which subreddit you want type 'redwall config'"
echo "2)to get a random wallpaper from the chosen subs type 'redwall set'"
echo "3)to change backround every x seconds type 'redwall set <seconds>'"
exec bash -l
