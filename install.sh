
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
