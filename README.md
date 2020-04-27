# codetour_export_to_md

[comment]: # (lmake_readme cargo.toml data start)
version: 0.1.1  date: 2020-04-27 authors: Luciano  
**proof-of-concept of export to md from CodeTour**

[comment]: # (lmake_readme cargo.toml data end)

CodeTour is a fantastic extension for VSCode. Kudos to the authors.  
<https://marketplace.visualstudio.com/items?itemName=vsls-contrib.codetour>  
CodeTours contain steps.  
Each step has a description and a link to the file and line number in the source code.  
No more out of sync examples in the documentation.  
This type of documentation is meant to be used for code flow explanation.  
To show other programmers the important information step by step in a logical order for humans.  
This extension for now works only inside VSCode. That is a problem for coders with other editors.  

## markdown

In the present version (2020-04-27) the extension has no functionality to export to a markdown file.  
I prepared this project as a proof of concept how the export to md could look like.  
I don't have enough knowledge in vs code extensions and Typescript to make a PR contribution.  
So I make a rust CLI tiny small program.
The resulting md is very nice. It is a file and therefore it can be committed to Github.  
In the md there are links to the source code on Github.  
That way all coders can follow the code flow on the actual code.  

## example

I copied to /example/ a few files from my other project where I use CodeTour.  
The filename of the *.tour file to be exported is hardcoded for that one example.  
I don't mean to use this program in real life. This is just a proof of concept for
the authors of CodeTour to add this functionality.  

## GitHub and working example

In my other project I tried to write documentation about the code flow.  
It was awful. I avoided copy/paste the source code because in no time it is obsolete and misleading.  
<https://github.com/LucianoBestia/mem6_game/blob/master/CodeFlow.md>  
Now I created a md from CodeTour and it is amazing:  
<https://github.com/LucianoBestia/mem6_game/blob/master/codetour_start_route_template_render.md>  
The step by step approach jumping from module to module is great.  
It just hides all the other non-important code for basic human understanding of the code flow.  
And the links are "alive", they go to the actual code.  

[comment]: # (lmake_readme exclude start A)  

## development

Clone the repo  
`git clone git@github.com:LucianoBestia/codetour_export_to_md.git`  
Use prepared cargo make scripts:  
`clear; cargo make release`  
`clear; cargo make run_rel1`  
The result is :  
`example/codetour_start_route_template_render.md`  

[comment]: # (lmake_readme exclude end A)  
