#!/bin/bash

REDDIT_LINK=`node ./getwallpaper.js`
PICTURE_NAME=$RANDOM.jpg

curl $REDDIT_LINK --output pics/${PICTURE_NAME}

IMAGE=`pwd`/pics/${PICTURE_NAME}


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




