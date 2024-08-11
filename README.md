[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/TXciPqtn)
# Rustwebserver

Detail the homework implementation.



contentType function:
to check the type of function
and return the type as the documentation
example 
for txt
it have to return "text / plain; charset = utf-8 "
then i used match for the types,
 #[tokio::main]
to make the main function async,
then start reading the command line and make the args[1] its for port
2 its for folder _root then create tcp listener and loop to handle the 
connection
connections function:
to read the daa and handle he http request and process reuqest method 
to match the metod if its get post check the path if start with scripts
then parse request function its start by spliting the request to header part and message
and extracting the requesit line from header example
GET/ index.html HTTP/1.1 then it will return the value
handlegetfunction it will combain the root directory and ,converts the requested path to handle anhy links and onfirm its valid path and if its not exists to return 404 not found
if the path valid it will call the process_request_path function 


parse_request fn to arrange and split the message parts in the header
arrange the header how it look make the header as hashmap

hadnle get fn to create the path 
start by to check if its available or no
if no return 404 error or 403 error
then check the metdata for the request
then send, handle file response and send error fn forcheck the response nad errors
send binary response
it will get the header ready with the format and the parts
to write thje header and sedn to the stream 
handle directory listing
to create the html for the directory lsiting
handle script it will split the path 
and handle the queery params
check if files exists etc...



