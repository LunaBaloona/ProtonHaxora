![Build Status](https://github.com/LunaBaloona/protonhaxora/actions/workflows/release.yml/badge.svg)

# Protonhax GUI for Cheathappens Aurora

## The Problem
Cheathappens Aurora has its own launcher, but it limits what Proton versions can be selected. Some Linux distros have custom Proton varients, which may run your games better, but you'll not be able to select them from Aurora.  There are work arounds but they are very terminal heavy, and we want to make things easy for Linux newcomers. 

## The solution
ProtonHaxora. It can run your game with ProtonHax which lets you run any app (in this case Aurora.exe) inside your preferred Proton. This runs in terminal, but ProtonHaxora combines the powers of ProtonHax, and Aurora in one-ish GUI. 

## Features
* **Auto-Discovery:** Finds then lists your installed Steam games. Tell it what game you need to attach Aurora to for your session. 
* **Easy Launching:** If you installed Aurora in the default directory, it will launch as long as the game is running. 

## Installation

Our installer installs Protonhax + ProtonHaxora. 

* Download the install zip
* Unzip
* Make the `install.sh` executable. Either right click it and enable "execute" or "Allow executing" in permissions - or run `chmod +x install.sh`
* double-click the install.sh or `./install.sh`

Protonhaxora should now be in your application menu. 

If you haven't yet, you should now install Cheathappens Aurora available at https://www.cheathappens.com/steamdecktool.asp

## Operation

* In steam, right click your game, select Properties and in the Launch Options section, paste: `protonhax init %COMMAND%` (you only need to do this once for any game you intend to play with Aurora/Protonhaxora)
* Run Protonhaxora
* Select a game or type Steam's App ID for the game
* Press Launch
* Once the game's fully loaded and at the menu, press F1 to launh Aurora. 

## Thanks
Thanks to jcnils for protonhax https://github.com/jcnils
