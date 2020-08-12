# Modpacker
launcher and data format for the distribution and installation of minecraft

------------
## Goals
1. To create a launcher that allows for the easy installation of:
		- Any version of Minecraft
		- Any modloader
		- Any mod or modpack
2. To create a data format that allows for:
	- A decentralized approach to mod hosting
	- Security in knowing that a mod is the correct version
	- Extendability in what it can provide
--------
## Roadmap
- [x] Download and parse Minecraft assets from Mojang's servers
- [ ] Launch said Minecraft installation
- [ ] Download and parse modloaders (plus extensibility for others)
	- [ ] Fabric
	- [ ] Forge
- [ ] Download mods by searching a repository
	- [ ] Care taken to follow the general ideas of the mc-cip/spec project
- [ ] Download modpacks through the same repository
- [ ] Create GUI for all of the above
