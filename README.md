### Gmailnator server

A local http server built with rocket that offers great interoperability.

### Background :

After creating the [gmailnator crate](https://github.com/Y0GAAAA/gmailnator) for Rust, I realized that support for selenium and other tools that would come in handy in a potential automation process was very limited within the Rust environment, that's why this repository exists. 

## Quick start :

### Configuration :

When starting the server, if no configuration file is found the default one will be created in the working directory with the name : <b>settings.ini</b><br>
<br>
<b>Fields :</b> 

`port` The port on which the http server will listen, default is 80 <br>
`address_expiration` The duration in seconds between the address queue renewals, default is 82800 (23 hours)

### Endpoints : 

<b>Note :</b> The local address/host is managed by Rocket and changes depending on whether the build is a release or a debug build, check the Rocket banner that is printed at program startup.
<br><br><b>Note 2 :</b> The "real" gmailnator server is a huge bottleneck, it is slow. A `/get_messages` call can take up to a few seconds, drink some tea and relax 🍵
<br><br><b>GET</b> `/new_address`                         Responds with a new gmailnator address in a raw string format.
<br>                                                        Example response : 
```wildtmp+zqeia@gmail.com```
<br><br><b>GET</b> `/get_messages/<address>`      Responds with a json string containing the messages in the inbox.
<br>                                                       Example response : 
```json
[
    {
        "subject": "first message",
        "raw_content": "contenu incroyable"
    },
    {
        "subject": "second message",
        "raw_content": "what's up ?"
    }
]
```
