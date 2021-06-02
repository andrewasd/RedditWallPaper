# RedditWallPaper
A program to automatically set your wallpaper from random reddit subs

This is a simple app that fetches random wallpaper from reddit and set it as background  
you can select the subreddits you want to chose with `redwall config`

it supports GNOME,KDE,CINNAMON,UNITY  

![alt text](https://pbs.twimg.com/media/EAF9qbVXkAAo1io.jpg)


**Requirments**  
curl
 
**Installation**:  
`git clone https://github.com/andrewasd/RedditWallPaper.git &&
cd RedditWallPaper &&  
chmod +x install.sh &&  
./install.sh  
`

**Chose desired subreddits and configurations:**
`redwall config`

**set random background once: **  
`redwall change`

**change random background every x seconds**  
`redwall set <seconds>`  
    to stop: `redwall stop`  

**override configs with:**
`redwall change -r <subreddit> -s <sort> -l <limit>`   
all parameters are optional  
example:  `redwall change -r earthPorn -s hot -l 30`  

you can also select to match something example   
`redwall change -r earthPorn -m norway`  
returns a pic from /r/earthPorn related to norway  

you can call the match also without the sub  
`redwall change -m norway`  
return a pic related to norway regardless of the subreddit  





