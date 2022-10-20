# Introduction

This is a microservice to be used with [actix-web-examples](https://github.com/shafiquejamal/actix-web-examples). 

This service does very little: it simply listens for two particular messages on a topic (can be specified via command line arguments), and respond with fixed responses:
- if the incoming message requests "person" data, it response with a `Person` object.
- if the incoming message requests "persons" data, it response with a vector of `Person` objects.

This could be done better, but exists just to demo something else. 

# References

- https://github.com/fede1024/rust-rdkafka
- https://mkaz.blog/working-with-rust/command-line-args/
