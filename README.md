# tema
A theme manager for \*nix systems respecting the FHS spec  

## configuration
As of now all the configuration is done via `$XDG_CONFIG_HOME/tema/tema.yaml`, consisting of
1. a list of modules, [What is a module ?](#what-is-a-module-?);
2. a list of commands, [What is a command ?](#what-is-a-command-?).

### what is a module ?
Modules are found at `$XDG_CONFIG_HOME/tema/themes/{theme_name}/{module_name}` and contain the relevant files for the theme.

eg. Module {
	name: name of module, generally the application it will affect			(eg. wallpaper) 
	path: the directory the respective file·s should be copied to 			(eg. $XDG_PICTURES_DIR/wallpapers)
	command: a complementary command to be run when the module is changed	(eg. swagbg -i $XDG_PICTURES_DIR/wallpapers/{name_of_wallpaper}.jpg)
}

### what is a command ?
A command is well, a command; but, it may be run on specific events. 

Currently implemented events: 
1. on_change 		(eg. on_change: "notify-send theme ${current_theme}")

There are also variables as seen above, currently implemented variables:
1. current_theme 	returns the currently enabled theme, stored at `$XDG_DATA_HOME/current_theme.tema`

## notes
- Currently command arguments containing whitespaces cannot be handled, even if they include quotation marks (")