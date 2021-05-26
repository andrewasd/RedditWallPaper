# RedditWallPaper
A program to automatically set your wallpaper from random reddit subs

This is a simple app that fetches random wallpaper from reddit and set it as background

**Requirments**  
curl ,node.js  
Ubuntu/Debian  
`sudo apt install nodejs`  
`sudo apt install curl`  
arch  
`pacman -S nodejs npm`  
`pacman -S curl`  
fedora/centOS  
`sudo dnf install nodejs`  
`sudo dnf install curl`  

 
**Installation**:  
`git clone https://github.com/andrewasd/RedditWallPaper.git`  
`cd RedditWallpaper`  
`chmod +x ./setwallpaper.sh`   
`chmod +x ./changeWallPaperInterval`


you can select the subreddits you want to chose in RedditConfig.json

**set background once** :  
`./setwallpaper.sh`

**change background every x seconds**  
`./changeWallPaperInterval <seconds>`  
 if you want to close the terminal while continuing to running in the background   
 `nohup `./changeWallPaperInterval `<seconds> & disown`  

if you want to add to the path so you can can call the script everywhere  
``echo "PATH=""`pwd`:"$"PATH">> /home/`whoami`/.bashrc``




