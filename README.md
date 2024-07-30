Leptosite

Leptosite is a web assembly application to quickly build web sites. It provides an infinitely customizable template for a menu-navigated web site. It is written in Rust and Leptos (ui toolkit) to optimize performance. The template includes four example pages and a top navigation menu. You edit the HTML layout in the page files and styles in the CSS files to customize the web site. Send questions to leptosite@overthinker.ai.

No Rust knowlege necessary. Almost all editing is done in HTML and CSS.

Once edits to page files are done, you use wasm-pack to build the app, then upload it to your web server as static files. That's it. More details are provided in the example Slint files in the "ui" directory.

Directory Structure:
src <-- contains the Leptos app source code.
  main.rs (main source file for app, no need to edit)
  web_site.rs (contains top level web site component list)
  nav_menu.rs (contains navigation menu layout, edit to customize menu)
  footer.rs (contains footer layout, edit to customize footer)
src/pages <-- this is where all the pages files are located, typically you only need to edit and/or add files to this folder.
images <-- put your images in here; currently it contains examples.
dist <-- each time you build with trunk it will copy the output files to here.
web_site.css (styles for body, nav menu and footer)
pages.css (styles for all pages)

How to Build:
If you don't have it already, install:
  - Rust (https://www.rust-lang.org/learn/get-started)
  - Trunk ("cargo install trunk")
In the root folder of this project, run:
trunk build --open

How to Deploy:
Copy the contents of the dist folder to the web root directory of your web server.