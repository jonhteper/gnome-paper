#!/bin/bash

# WARNING: para que el script funcione, es necesario que existan los archivos:
# ~/.dyn-wallpapers/d.jpg
# ~/.dyn-wallpapers/n.jpg

# Por ahora este script solo funciona cambiar en modo oscuro: 
now=$(date +%-H)
file="n.jpg"

if ((now >= 7 && now < 19)); then
	file="d.jpg"
	echo "¡wallpaper de día activado!"
else
	echo "¡wallpaper de noche activado!"
fi


gsettings set org.gnome.desktop.background picture-uri-dark ~/.dyn-wallpapers/"$file"
