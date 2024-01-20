
![build](https://github.com/ramiroaisen/openstream-rs/actions/workflows/cargo-build.yml/badge.svg)
![unit tests](https://github.com/ramiroaisen/openstream-rs/actions/workflows/cargo-unit-tests.yml/badge.svg)
![integration tests](https://github.com/ramiroaisen/openstream-rs/actions/workflows/cargo-integration-tests.yml/badge.svg)
![front server build](https://github.com/ramiroaisen/openstream-rs/actions/workflows/front-server-build.yml/badge.svg)
![front server unit tests](https://github.com/ramiroaisen/openstream-rs/actions/workflows/front-server-unit-tests.yml/badge.svg)
![front studio build](https://github.com/ramiroaisen/openstream-rs/actions/workflows/front-studio-build.yml/badge.svg)
![front studio typecheck](https://github.com/ramiroaisen/openstream-rs/actions/workflows/front-studio-typecheck.yml/badge.svg)
![front admin build](https://github.com/ramiroaisen/openstream-rs/actions/workflows/front-admin-build.yml/badge.svg)
![front admin typecheck](https://github.com/ramiroaisen/openstream-rs/actions/workflows/front-admin-typecheck.yml/badge.svg)



# Openstream Radio Streaming Server

Openstream is a modern radio streaming server.
Modern being scalable, multi-tenant and API-controlled.

You can choose to run your own openstream server on-premise or to create a broadcaster account at [Openstream Studio](https://studio.openstream.fm)

You can also access the API documentation at [Openstream API Documentation](https://api.openstream.fm/docs). If you are running Openstream on-premise just change the `baseUrl` of the API server.


## Scalable
Openstream server(s) can scale not only vertically but also horizontally.

Openstream stores all of its data in a (possibly) sharded + replicated [MongoDB](https://www.mongodb.com) database, so you can set up your cluster as large as you want to.

Openstream nodes will operate in cooperation with each other if they share the same MongoDB database.

You can just add more Openstream nodes to your cluster to add more capacity to the hole system.


## Security

Openstream is written in the [Rust Programming Language](https://www.rust-lang.org). Rust's novel memory ownership model prevent many form of memory-related bugs that would otherwise escalate to security issues.

Access to Openstream API is provided to users and administrators via Access Tokens that are needed to access the API resources.

Access Tokens can be obtained with a username-password pair of and admin or user and are bound to the resources owned by that scope.


## Portability

Currently Openstream is meant to run in Linux systems but it's written in a portable language and can be ported to another Operative Systems with ease. 

## Open Source
Openstream is licensed under the GNU General Public License v3.0

## Contact
You can reach us at opensource@openstream.fm if in need for help or advice.
