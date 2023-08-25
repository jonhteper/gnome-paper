#!/bin/bash

# WARNING: para que el script funcione deben existir los 
# archivos:
# ~/.dyn-wallpapers/dyn-wallpapers.sh
# ~/.dyn-wallpapers/d.jpg
# ~/.dyn-wallpapers/n.jpg

add_cron_job() {
    local time=$1
    local command=$2
    (crontab -l; echo "$time $command") | crontab -
}

path=".dyn-wallpapers/dyn-wallpapers.sh"
command="bash /home/${USER}/${path}"

add_cron_job "0 7 * * *" "$command"

add_cron_job "0 19 * * *" "$command"

add_cron_job "/30 * * * *" "$command"

add_cron_job "@reboot" "$command"