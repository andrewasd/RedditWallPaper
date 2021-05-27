#!/bin/bash

DIRECTORY=/home/`whoami`/.RedditWallPaper
REDDIT_LINK=`node $DIRECTORY/getwallpaper.js $DIRECTORY`
PICTURE_NAME=$RANDOM.jpg


curl $REDDIT_LINK --output $DIRECTORY/pics/${PICTURE_NAME}

IMAGE=$DIRECTORY/pics/${PICTURE_NAME}

echo $IMAGE > /home/`whoami`/Desktop/file.txt
case ${XDG_CURRENT_DESKTOP} in

  KDE)
      qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript "
    var screens = desktops();
    for (i=0;i<screens.length;i++) {
        screens[i].wallpaperPlugin = 'org.kde.image';
        screens[i].currentConfigGroup = Array('Wallpaper',
                                    'org.kde.image',
                                    'General');
        screens[i].writeConfig('Image', '${IMAGE}')
    }"
    ;;

  GNOME)
    `gsettings set org.gnome.desktop.background picture-uri "${IMAGE}"`
    ;;

  UNITY)
    `gsettings set org.gnome.desktop.background picture-uri "${IMAGE}"`
    ;;

  CINNAMON)
    `gsettings set org.cinnamon.desktop.background picture-uri  "${IMAGE}"`
    ;;
esac




