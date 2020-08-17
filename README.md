# wayang-rs
Wayang is an engine for interactive storytelling, currently it focuses on enabling its user to prototype interactive story published in the form of website. Under the hood, Wayang uses Rust to parse story into sets of web pages, ready to be deployed anywhere a static website is available. [Wayang](https://en.wikipedia.org/wiki/Wayang) is traditional Indonesian puppet show, it's a well known artform.

## Mechanic
Story in Wayang is written using DSL (Domain Specific Language) also called Wayang that takes the `.wyg` file extension. Wayang engine will then parse this file and generates a website that consists of multiple interconnected web pages.

## Wayang DSL
Wayang DSL is deesigned to enable rapid development of story. I omit usage of JSON, to reduce the probablity of syntax mistype. Currently, the syntax is as follows:

```
[scene 1]

Scene:
What, do you think? The water is clear. The sky is blue. The air is fresh.

Event: 
The pool to your left and door to your right. Which one will you choose?

Choice:
- Go to left. -> [scene 2]
- Go to right. -> [scene 3]
```

This file should generate an HTML file roughly as follows

```
<html>
  <head>
    <title>scene 1</title>
  </head>
  <body>
    <div class="scene">
      <div class="sceneContent">
        What, do you think? The water is clear. The sky is blue. The air is fresh.
      </div>
    </div>
    <div class="event">
      <div class="eventContent">
        The pool to your left and door to your right. Which one will you choose?
      </div>
    </div>
    <div class="choice">
      <div class="choiceContent">        
        <a href="scene 2.html">Go to left.</a>
        <a href="scene 3.html">Go to right.</a>
      </div>
    </div>
  </body>
</html>
```

How this HTML is presented is then left to the implementation phase of the file, as it's just a matter of styling HTML and CSS.

## Roadmap
At the moment the biggest milestone is getting the DSL properly parsed into said HTML format. After managed to parse it by hand, I realize that using parser Crate such as [pest](https://www.pest.rs) is a better option. 

## Parsing .wyg file
As an effort to parse `.wyg` file using pest, a grammar needs to be established first. The [`grammar.pest`](https://github.com/lunchboxav/wayang-rs/blob/master/grammar.pest) file is an effort to do so. The current grammar file is enough to parse the specified `.wyg` file to grab required data. If you're testing the grammar, [pest.rs](https://pest.rs/) homepage has a nice parser playground for experimenting.
