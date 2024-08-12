[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/TXciPqtn)
# Rustwebserver

Detail the homework implementation.



then i used match for the types,
 #[tokio::main]
to make the main function async,
then start reading the command line and make the args[1] its for port
2 its for folder _root then create tcp listener and loop to handle the 
for connection function i start by creating a buffer from 0 to 8192 bytes then reading the data from stream ,convert the buffer to string using utf-8 encoding then create the request as line,header,body, make the request line as method path and http,check the request if its get or post if non of them it will return method not allowed...
get function : starting by convert the root to pathbuf,triming any / in the path ,and request the path if success it will be ok else it will return error and it will get the metadata for the reuqested path ...
directory listing folder to create the file of folders and the name using html,handle script function its managing the http request on the server and check the script path then check if the script exist and excute it wuth reading the headers and method if its get or post and if script not found it will retourn 404 or 500 
get contnet type its to check the extension of file and return the same of the docuemtnaiton
example if its txt it will return text/pain; charset=utf-8 as string
