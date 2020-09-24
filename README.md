## Concept
Draught of FPL is an intermediator for a [Fantasy Premier League Draft League](https://draft.premierleague.com)

While the official site has almost all the good information that you could want, it falls short in
some aspects, most notably an updated overview of the entire league. The main feature of this
application is to provide a view of the league table that is updated during the games. 
This is done using the API used and provided by the official web page.

All APIs from the leagues are publicly available and the only necessity for logging in with 
credentials is to find your league ID

## High level architecture

1. Read Config.toml to configure the context of the program such as ID of the league we want information from.
2. Sets up a fetch thread that in intervals fetches new information from the FPL api
3. Sets up a compute thread that in intervals computes the values that we want to provide in the intermediate api
4. Use [rocket.rs](https://rocket.rs/) to set up a web server that serves selected values computed in the compute thread

### Configuring the application

The configuration for this server is done using a .toml file (default ./Config.toml) which is
serialized using a [TOML](https://toml.io/) parser when the server starts. The structure of this configuration file is
best observed in the code. 

The only mandatory element of the configuration file is the league code as 
```
league_id = 1337
```

#### Configuring rocket.rs server

See rocket.rs own [configuration tutorial](https://rocket.rs/v0.4/guide/configuration/#rockettoml)
where things like server port can be configured. This is done using the Rocket.toml configuration 
file.

## Getting started

To run the server it should suffice to configure and use [cargo](https://doc.rust-lang.org/cargo/)
to either run the server directly or build a binary. `Note that the configuration file is relative
to the execution`

### Getting league code from FPL

Go to [FPL Draft website](https://draft.premierleague.com) and login with your credentials.

Open https://draft.premierleague.com/api/bootstrap-dynamic in the browser and find the 
set called "leagues" and select the ID of the league you want to set up the server.
```  
"leagues": [
       {
         "id": 1337,
         "name": "Example League",
         "scoring": "c"
       },
```

### Local database

If you don't want to stress the official FPL servers there is an option of fetching all the 
information from the file system where the path to an `/api` is used as the prefix and all 
the requests will be directed towards the path afterwards. Example `/bootstrap-static` is read
from a file called: `/fpl/api/bootstrap-static`

Reading from files instead of fetching from Fpl API is configured by setting `local_fetch` to true
like: `local_fetch = true` 
in the configuration [TOML](https://toml.io/). This disabled by default.

The API root is by default `/fpl/api` but can be configured by setting the `local_url` to the
root of the local file database. Example `local_url = /fpl/api` 

#### Building local database

Inside this repository is a directory called `db` and within it a shell script `create-db.sh`.
This script will build the local database in the directory from where it is run. It needs the
league id as the first argument, and requires `curl` and `jq` to be installed on the system. 
The database is of course frozen in time, and it is recommended to remove the local database 
before running the script again, as it tries to not overwrite existing files.

Example:

```
cd db
./create-db.sh 1337
```

builds a directory called `api` under which the important endpoints should be available. 

```
r@u:~/db$ tree 
.
├── api
│   ├── bootstrap-static
│   ├── entry
│   │   ├── 55
│   │   │   ├── event
│   │   │   │   ├── 1
│   │   │   │   └── 2
│   │   │   └── public
│   │   ├── 85
│   │   │   ├── event
│   │   │   │   ├── 1
│   │   │   │   └── 2
│   │   │   └── public
│   │   └── 105
│   │       ├── event
│   │       │   ├── 1
│   │       │   └── 2
│   │       └── public
│   ├── event
│   │   └── 2
│   │       └── live
│   ├── game
│   └── league
│       └── 1337
│           └── details
└── create-db.sh

18 directories, 23 files

```

## Response: Table Structure

Explanations for the values within the table exist as comments in the table code.
