## Issue 1994

https://github.com/SergioBenitez/Rocket/issues/1994

To reproduce the issue...

### Run server

cargo run --bin server

### Run HTTP/2 client

cargo run --bin client

which is different to 

### Run HTTP/1.1 client

cargo run --bin client -- --http1

## Description

When requesting an invalid Uri like `GET %2Fhello%2Fworld` on an hyper HTTP/1.1 connection the service function doesn't get called and hyper itself returns a BadRequest reponse. 

In a __HTTP/2__ connection the `hyper.uri` is set to something like _http://127.0.0.1:8000%2Fhello%2Fworld_ (without slash) and this gets passed to the service function.  
`hyper.uri.path_and_query()` makes from this __magically__ __/%2Fhello%2Fworld__ (now with slash).
