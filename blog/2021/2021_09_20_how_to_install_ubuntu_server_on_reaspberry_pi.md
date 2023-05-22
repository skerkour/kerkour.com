+++
date = 2021-09-20T13:00:00Z
title = "How to install Ubuntu Server on a Raspberry Pi 4"
type = "post"
tags = [ "linux", "programming", "tutorial", "raspberry-pi"]
authors = ["Sylvain Kerkour"]
url = "/how-to-install-ubuntu-server-64-bit-on-a-raspberry-pi"

[extra]
lang = "en"

comment ="""
"""
+++

I just got my hands on Raspberry Pi 4. As I wanted to install a 64 bit Operating System, Ubuntu seemed to be the best choice. Unfortunately, [the official tutorial](https://ubuntu.com/tutorials/how-to-install-ubuntu-on-your-raspberry-pi) on installing Ubuntu Server 21.04 on a Raspberry Pi didn't work for me, so here is how I proceeded.

## Flashing the OS onto the micro SD card

### From Linux

```shell
lsblk -p
```

You should see the list of the attached disks. Note the device of the micro SD card (for example: `/dev/sdb`) and replace `X` in the following command:

```shell
$ sudo dd if=your_image.img of=/dev/sdX bs=4M conv=fsync
```

Finally, you can eject the card.


### From macOS


```shell
$ diskutil list
```

You should see the list of the attached disks. Note the number of the micro SD card (for example: `/dev/disk2` is `2`) and replace `N` in the following commands by it:
```
$ diskutil unmountDisk /dev/diskN
$ sudo dd bs=1m if=your_image.img of=/dev/rdiskN; sync
$ sudo diskutil eject /dev/rdiskN
```


## Setting up the OS

Then, SSH into the Raspberry Pi. Username: `ubuntu`, Password: `ubuntu`

```shell
$ ssh ubuntu@[IP_ADDRESS]
```

And follow the instructions.

### Updating the software and firmware

This step is meant to ensure that your Raspberry Pi runs the latest software and firmware:
```shell
$ sudo apt update && sudo apt upgrade -y
$ sudo apt install rpi-eeprom-update
$ sudo rpi-eeprom-update
```

## Setting up Wi-Fi

This is where the official tutorial stopped working for me. Indeed, instead of editing the `network-config` file,
you need to edit `/etc/netplan/50-cloud-init.yaml`, such as follow:

```yaml
network:
    ethernets:
        eth0:
            dhcp4: true
            optional: true
    version: 2
    wifis:
        wlan0:
            optional: true
            dhcp4: true
            access-points:
                "{{SSID}}":
                    password: "{{PASSWORD}}"
```

And, that's all :)


Reboot your Raspberry Pi with `sudo shutdown now -h` and you are ready to rock 🚀
