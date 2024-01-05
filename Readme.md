# Json parser
This project is a simple json parser I wrote in Rust. It can parse a 5MB json file in 69 milliseconds. At the moment, you can read, print, use, and modify the json data. Reading and modifying do require the user to check the type of the json value. The json values are enums, so it's easy and safe to check the type. Json values can be read from any character iterator, which means that you could use it for files, user input, strings, and more.

The example application uses a file as input and prints the json value to the screen.
