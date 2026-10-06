# HERD
## What is Herd?
Herd is a build system for C and C++, it aims to make building and managing my C and C++ projects and their dependencies easy

## Background
Herd was born from my frustration with existing C and C++ build systems, I am a make user normally but even with this supposedly simple
system I forget the syntax most times, the syntax is very quirky and weird with its tab laws, Integrating projects that use different 
build systems is a pain there are many other pain points, Then one day I used Rust and experienced cargo and I was star struck it was so 
coherent and well designed and it made using the language all that easier unlike C and C++ where you must learn a complex system and most times
have to manage dependencies manually 
Now I know herd cant be like cargo well because unlike Rust with its centralized registry C and C++ dont behave like this they are very 
decentralized  however I plan to use this to my advantage using package managers per distro

## What it's not?
+ Not a package manager, it resolves dependencies via existing tools like pkg-config and distro package managers, it doesnt host or manage said 
dependencies
+ Not cross platform, right now it only aims for linux(Debian for now since thats what I have) extending this isnt a priority and might never happen
+ Not a general replacement for make or Cmake so dont expect it to do what all these systems and many others do

## Who is it for?
Well its for me and anyone who wants to use it
