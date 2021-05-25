#!/bin/bash

#settings
#---------------------------------------------------------------
SUBS=("art,pics,earthporn,oldschoolcool,images")
SORTBY="hot" #available ->new/
#---------------------------------------------------------------

SUBS=${SUBS//,/ }" ${SORTBY}"

WALLPAPER_LINK=`node ./getwallpaper.js $SUBS`


case ${XDG_CURRENT_DESKTOP} in

  KDE)
      qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript "
    var screens = desktops();
    for (i=0;i<screens.length;i++) {
        screens[i].wallpaperPlugin = 'org.kde.image';
        screens[i].currentConfigGroup = Array('Wallpaper',
                                    'org.kde.image',
                                    'General');
        screens[i].writeConfig('Image', '${WALLPAPER_LINK}')
    }"
    ;;

  GNOME)
    `gsettings set org.gnome.desktop.background picture-uri "${WALLPAPER_LINK}"`
    ;;

  UNITY)
    `gsettings set org.gnome.desktop.background picture-uri "${WALLPAPER_LINK}"`
    ;;

  CINNAMON)
    `gsettings set org.cinnamon.desktop.background picture-uri  "${WALLPAPER_LINK}"`
    ;;
esac



#uncomment if you want to save the pic
curl $WALLPAPER_LINK --output pics/$RANDOM.jpg 

#setting wallpaper



