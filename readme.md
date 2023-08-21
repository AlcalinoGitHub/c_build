## C_build

c_build is a package manager and build system for c/c++, that aims to simplify and automate the process
of managing and building c/c++ applications, similar to *cargo* for rust or *npm* for Javascript. It also makes the projects self-contained, by installing all dependencies locally.

commands:  
```bash
c_build init`  
```
This command creates a c_build_config.json file and a c_build_modules directory. The config file has sdl2 installed by default as a placeholder for other libraries. It also links math and std

<br>

```bash
sudo c_build install
```
This command install all dependencies to the project to c_build_modules. Note that make, cmake and git are required for this command to work
<br><br>

```bash
sudo c_build compile
```
This command compiles the code, linking the installed libraries to c_build_modules
<br><br>

```bash
sudo c_build run
```
This command compiles and runs the code


The default config file looks like this

```json
{
  "input": "main.cpp",
  "output": "main",
  "libs": [
    {
      "name": "SDL2",
      "git_url": "https://github.com/libsdl-org/SDL.git",
      "build_command": "cmake -S . -B build && cmake --build build && cmake --install build"
    }
  ],
  "std": [
    "lstdc++",
    "lm"
  ]
}
```
1. *input* specifies the main c/c++ file
2. *output* specifies the name of the exported binary
3. *libs* specifies all the external dependencies
   1. *name* is the name of the dependency, as will be used by the compiler
   2. *git_url* expects a valid url for `git clone`
   3. *build_command* will be used to compile the binary. It is usually specified within the library's repo. If it isnt, it is usually `make`
4. *std* specifies the std libraries to link. Altough not what it is intented for, you can also add any compiler flag here. They will be used in the same order as you input them

# Notes
1. c_build_modules folder is not meant for maual manipulation. A `remove` command will be added to remove broken dependencies
2. `c_build init` does NOT create a .gitignore, I recommend adding c_build_modules to gitignore.