# wrapper_extender
An attempt to start yggdrasil wrapper from Godot


## 👾 Godot Integration (Workaround)
Due to permission elevation we need this to launch the yggdrasil_wrapper.exe:

- The extender is a small native helper that Godot can launch.
- It starts this wrapper with admin permissions.
- Make sure you check the file names before you build. By default the wrappr name is set to yggdrasil_wrapper.exe
- Will also print the pid of the wrapper, you can use that to close the process. 

⚠️ yggdrasil does not stop, you will have to close it manually from task manager (will have to fix this sooner or later )