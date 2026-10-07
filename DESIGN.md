## Version 0.1
These are the objectives of version 0.1
+ Support the init command
+ Create the build.toml file with the standard config
+ Create the two standard dirs i.e include, src and build (For executables now only)
+ Create the main.c stub inside the src/main.c file
+ Parse the toml file to guide the basic build system
+ Come up with a final executable

# Why only C for now
Supporting C++ isnt a priority right now and most especially isnt an aim of v0.1, I want the syste, basics to prove themselves on C first
Support for C++ is deffered till the toml parsing and build pipelines are stable enough


# Standard toml file config
This dicatates the basic standard config that is created when the init command created the build.toml file
For v0.1 only the project key and the name are enough

# The workspace manager
This is like a representation of the workspace we are dealing with, it is owned by the build ochestrator which I will define later
For v0.1 it keeps an array of all the source files, the layout and the config 
Reason being it needs to have the overall info in order to properly ochestrate the build
I had stupidly tried creating it whenever init runs but yeah thats was dumb no need for it, anyways the resolution
is that it will get built whenever the build command runs since that is when we want to build after that it can die 
It however needs to load the build.toml from disk since it needs to know the name of the project 
