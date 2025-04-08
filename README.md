## Running blinky on Milk-V Duo 256M
The LED GPIO pin (PWR_GPIO[2]) is held by the kernel. So we need it to free it first. 
</br>
</br>
The following command frees the pin and disables automatic LED blinking. `mv /mnt/system/blink.sh /mnt/system/blink.sh_backup && sync`
(blinking can be brought back by: `mv /mnt/system/blink.sh_backup /mnt/system/blink.sh && sync`)
</br>
</br>
Now run the program on this branch