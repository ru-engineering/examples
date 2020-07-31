#! /bin/sh

cargo build
scp target/arm-unknown-linux-gnueabihf/debug/screen_demo pi@192.168.137.12:/home/pi
ssh pi@192.168.137.12 /home/pi/screen_demo

