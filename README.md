# b64 as for base64

This is my implementation of the base64 encoding. This crate was initially 
created to train my new rust skills. My first attempt at programming such an 
Encoder/Decoder was in C#. However, it was slow and memory intensive. 
Fortunately, I managed to reduce memory usage and improve run speed.

Warning: My implementation of a base64 decoder needs paddding to work as
attended. If you notice any unexpected behaviour please open an issue so I can
look into it.

# Sources
- Base64: [Wikipedia](https://en.wikipedia.org/wiki/Base64)
- Base64 Encoding: [Video by schenken](https://www.youtube.com/watch?v=aUdKd0IFl34)
