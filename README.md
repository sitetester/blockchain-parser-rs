`docker-compose up`

This repository is based on [tokio](https://tokio.rs) (the most popular & stable async environment for Rust). It launches multiple [tasks](https://tokio.rs/tokio/tutorial/spawning) which are like Go [goroutines](https://tour.golang.org/concurrency/1).
DB schema is handled through [Diesel](https://diesel.rs)

Currently blocks are parsed in range of 50. This is a reasonable value, since remote server usually not responds in time in case of a higher value,
so we could get some unexpected errors in that case.