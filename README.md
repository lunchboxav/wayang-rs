# wayang-rs
Wayang is an engine for interactive storytelling, currently it focuses on enabling its user to prototype interactive story published in the form of website. Under the hood, Wayang uses Rust to parse story into sets of web pages, ready to be deployed anywhere a static website is available. [Wayang](https://en.wikipedia.org/wiki/Wayang) is traditional Indonesian puppet show, it's a well known artform.

## Mechanic
Story in Wayang is written using DSL (Domain Specific Language) also called Wayang that takes the `.wyg` file extension. Wayang engine will then parse this file and generates a website that consists of multiple interconnected web pages.

## Wayang DSL
Wayang DSL is deesigned to enable rapid development of story. I omit usage of JSON, to reduce the probablity of syntax mistype. Currently, the syntax is as follows:

```
Scene
  [scene number a]
  [scene description]
  Event
    [event description]
  Branch
    [choice 1] go to Scene[number x]
    [choice 2] go to Scene[number y] 
```

This file should generate an HTML file roughly as follows

```
<!-- a.html -->
<html>
  <head></head>
  <body>
    <h1>scene number a</h1>
    <p>scene description</p>
    <p>Event descriotion</p>
    <a href="x.html>choice 1</a>
    <a href="y.html>choice 1</a>
  <body>
</html>
```

How this HTML is presented is then left to the implementation phase of the file, as it's just a matter of styling HTML and CSS.

## Roadmap
At the moment the biggest milestone is getting the DSL properly parsed into said HTML format. 
