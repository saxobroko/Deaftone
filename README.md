<p align="center">
  <img src="https://raw.githubusercontent.com/Ortygia/Deaftone/master/resources/deaftone.png" alt="Deaftione logo" title="navidrome" align="center" height="340" width="340" />
</p>


<div align="center">

[![license](https://img.shields.io/github/license/Ortygia/Deaftone)](https://github.com/Ortygia/Deaftone/blob/master/LICENSE)
[![CI](https://github.com/Ortygia/Deaftone/actions/workflows/build.yml/badge.svg)](https://github.com/Ortygia/Deaftone/actions/workflows/build.yml)
![codecov](https://codecov.io/gh/Ortygia/Deaftone/branch/main/graph/badge.svg?token=NWS6Q3W4FP)
![GitHub repo size](https://img.shields.io/github/repo-size/Ortygia/Deaftone)
![Lines of Code](https://aschey.tech/tokei/github/Ortygia/Deaftone)
![Version](https://img.shields.io/github/v/release/Ortygia/Deaftone)
</div>

# Overview
A cross-platform open source music collection server and streamer. Leaving Subsonic in the past and starting fresh with a new design and new API
Currently in active development

# Perfomance
Currently scans 34,000 songs in ~14 mins. My testing is done on a USB Desktop HDD 3TB seagate over USB3.

# Features
* Ability to handle the largest of music collections
* Very low system resource usage
* Multi-platform. Currently building for macOS, Windows, Linux, Arm and Armhf 

# Clients
* Tauri Desktop client for macOS, Windows and Linux
https://github.com/Ortygia/Orpheus currently in early stages of development but usable
* Android based application possibly native or using Tauri Mobile

# Roadmap
* Built-in metadata scrapping of sources such as MusicBrainz, LastFM and AllMusic
* SlimProto Support
* Playlist Curation
* Recommendation engine
* ReplayGain support 
* Radio mode

# Installation
Currently to setup and install Deaftone you need to download the binarie in release for you platform or clone and build the repo.
After you have your binary in the same folder you need to place a ``settings.toml`` with the following inside it
```
log_level="deaftone=info,tower_http=info"
db_path="./deaftone.sqlite"
media_path="H:\\aa"
```
Where media_path is the location of your media
db_path is where to save the database
and logging is for change the log level of the application


