# Promptier 📚

This project is a showcase of a prompt library powered by [Rust](https://www.rust-lang.org) and [Poem](https://github.com/poem-web/poem).
It offers an interface compatible with [OpenAI](https://openai.com) to store, retrieve and delete your prompts form [MongoDB](https://www.mongodb.com).


## Running

You can run this package as a standalone using either cargo or Docker. The project can be configured using a file or environment variables. Make sure to check the [configuration section](#configuration) before running the application.

### Docker:

Running in docker is a breeze with the provided compose file, yuo just need to run:

```bash
docker compose up app
```

### Cargo:

Running directly with cargo requires you to previously setup a mongodb to use. Check the [configuration section](#configuration) on the variables need for the mongo connection.

Once you have setup a MongoDB database you can run the command:

```bash
cargo run --release
```



## Configuration

Current variables can be seen in the file [example.env](./example.env). It provides the following variables:

```bash
# Application port
PORT=8080

# Application log level
LOG_LEVEL="debug"

# URI to which mongoDB will connect
DB_URI="mongodb://mongo:27017"

# Name of the database to be used as root database
DB_NAME="promptier"
```

Modify them per your needs and rename to `.env`. The application will only read variables from a file named `.env`.