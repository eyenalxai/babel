## Babel

Babel is an Actix web application that uses the OpenAI GPT-3.5 model to generate HTML pages based on the provided URL path. 

### How to use
Clone and cd into the repository.
```bash
git clone https://github.com/eyenalxai/babel.git
cd babel
```

Set the required environment variables, for example:
```bash
export OPENAI_TOKEN=sk-...
export PORT=8080
```

Build and run the project:
```bash
cargo build --release
cargo run --release
```
Visit the generated website in your browser by navigating to `http://localhost:PORT`, replacing `PORT` with the port number you set earlier.

You can also access any pages by appending a URL path after the port number, e.g., `http://localhost:PORT/glass-chairs`.