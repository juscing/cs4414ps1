Title: Problem Set 1 Answers
Author: Justin Ingram

Problem 1.
My User-Agent is:
User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1700.77 Safari/537.36

The full text with explanations:
Received request :
GET /favicon.ico HTTP/1.1 --Retrieving the favicon image (the icon for the tab)
Host: localhost:4414 --The url requested
Connection: keep-alive --The type of connection
Accept: */* --The type of connections to accept, in this case all
User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1700.77 Safari/537.36 --The user agent saying I am using Chrome on Linux in an X11 Environment
Accept-Encoding: gzip,deflate,sdch --The types of compressions to send
Accept-Language: en-US,en;q=0.8 --The types of languages in the request


Problem 2.
Rust probably thinks this is unsafe because of concurrency issues. A web server needs to handle requests concurrently and therefore the varaible may not increment correctly because it creates a race condition.