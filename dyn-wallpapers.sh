#!/bin/bash

# WARNING: para que el script funcione, es necesario que existan los archivos:
# ~/.dyn-wallpapers/d.jpg
# ~/.dyn-wallpapers/n.jpg

# Por ahora este script solo funciona cambiar en modo oscuro: 
now=$(date +%-H)
file="n.jpg"

day_msg="¡wallpaper de día activado!"
nigth_msg="¡wallpaper de noche activado!"


if ((now >= 7 && now < 19)); then
	file="d.jpg"

	echo "$day_msg"
	echo "Hora:${now} -- ${day_msg}" >> "/home/$USER/.dyn-wallpapers/dyn-wallpapers.log"
else
	echo "$nigth_msg"
	echo "Hora:${now} -- ${nigth_msg}" >> "/home/$USER/.dyn-wallpapers/dyn-wallpapers.log"
fi

path="/home/${USER}/.dyn-wallpapers/${file}"
gsettings set org.gnome.desktop.background picture-uri-dark "$path"
echo "ejecutando... $path"
