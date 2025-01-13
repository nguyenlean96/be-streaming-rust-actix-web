# Docker deployment

## Prerequisites

- Docker

- Docker Compose (optional)

## Explanation

This `Dockerfile` is a **multi-stage** builder

- **Stage 1**: Build the application using `rust:1-slim-bookworm` image to compile the project

- **Stage 2**: By `run stage`, uses a minimal `bitnami/minideb:bookworm` image for running the compiled binary, significantly reducing image size.

- **Next Stage**: For **Non-Root** User, the final container runs the binary as a non-root user (`USER 1001`), a best practice for security.

- **Next Stage**: Then expose port `8080` and  runs the binary <image-name>.

## Build and run the Docker image

1. Clone the repository

2. Build the Docker image
    ```bash
    docker build -t <image-name> .
    ```

3. Run the Docker image
    
    ```bash
    docker run <image-name>
    ```

    ### **Note:**

    - To run Docker Container in the background, use `-d` flag to use **Detached Mode**.
    
    ```bash
    docker run -d <...options> <image-name>
    ```

    - As there is a `.env` with a dedicated `APP_PORT` variable, you can pass the port as
    
    ```bash
    docker run --env-file .env -p $(grep APP_PORT .env | cut -d '=' -f2):8080 forest-backend
    ```

    ### **Dynamic Mapping Based on Container**'s `APP_PORT`:

    - If the app uses `APP_PORT` inside the container, you can map the same variable:
  
    ```bash
    docker run --env-file .env -p $(grep APP_PORT .env | cut -d '=' -f2):$(grep APP_PORT .env | cut -d '=' -f2) <image-name>
    ```

### OPTIONAL: Using `docker buildx` for multi-architecture builds

If you're running **Docker BuildKit** (via `docker buildx`), the syntax is the same, but explicitly calls the BuildKit feature:

```bash
docker buildx build -t <image-name>
```