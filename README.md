# An intermediator for a Fantasy Premier League draft league

## High level architecture

1. Read config.toml to configure the context of the program such as ID of the league we want information from.
2. Sets up a fetch thread that in intervals fetches new information from the FPL api
3. Sets up a compute thread that in intervals computes the values that we want to provide in the intermediate api
4. Use [rocket.rs](https://rocket.rs/) to set up a web server that serves selected values computed in the compute thread
