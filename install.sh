
INSTALL_DIR=/home/`whoami`/.RedditWallPaper

rm -rf ${INSTALL_DIR}
mkdir  ${INSTALL_DIR}
cp -r src/* ${INSTALL_DIR}
# add to path
echo "PATH=""${INSTALL_DIR}:"\$"PATH">> /home/`whoami`/.bashrc
export PATH=${INSTALL_DIR}:$PATH
exec bash -l