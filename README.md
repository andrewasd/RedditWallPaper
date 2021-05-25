# RedditWallPaper
A program to automatically set your wallpaper from random reddit subs

This is a simple app that fetches random wallpaper from reddit and set it as background

**Requirments**  
 node.js    
 Ubuntu/Debian based:
 `sudo apt install nodejs`  
 arch:
 `pacman -S nodejs npm`  
 fedora/centOS:
 `sudo dnf install nodejs`  
 
**Installation**:  
`git clone https://github.com/andrewasd/RedditWallPaper.git`  
`cd RedditWallpaper`  
`chmod +x ./setwallpaper.sh`   


you can select the subreddits you want to chose in setwallpaper.sh

**set background once** :  
`./setwallpaper.sh`

**change background in an interval of time**  
`watch -n <seconds> ./setwallpaper.sh`

if you want to add to the path so you can can call the script everywhere  
``echo "PATH=""`pwd`:"$"PATH">> /home/`whoami`/.bashrc``




