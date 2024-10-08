

# A Streaming Platform for the Future


## Docker Setup

### Prerequisites
Ensure you have the following installed on your system before running the project:
- Docker
- Docker Compose

### Getting Started
1. Clone the Repository
```bash
git clone https://github.com/nguyenlean96/be-streaming-rust-actix-web.git
cd be-streaming-rust-actix-web
```

2. Build and Run the Docker Containers
To build and run the Docker containers, execute the following command in the project root directory:
```bash
docker-compose up --build
```

This command will:
- BUild an Ubuntu-based Docker image
- Install and configure FFmpeg for media streaming
- Setup Redis for caching
- Launch MinIO as an S3-compatible object storage server

3. Access the Services
- **Backend Streaming**: Once the containers are up, this app will be available at `http://localhost:8080`
- **Redis**: Redis will be running on `localhost:6379`
- **MinIO**: MinIO will be running on `http://localhost:9000`. You can access the MinIO dashboard with the following credentials:
  - Access Key: `minioadmin`
  - Secret Key: `minioadmin`

4. Environment Variables
This setup uses the following environment variables for **MinIO**:
- `AWS_ACCESS_KEY_ID=minioadmin`
- `AWS_SECRET_ACCESS_KEY=minioadmin`
- `AWS_ENDPOINT=http://minio:9000`

These variables are passed automatically from the `docker-compose.yml` file. If you need to change them, you can do so in the `docker-compose.yml` file.

5. Building the Project
Inside the Docker container, this project will be built using the following command, which is defined in `Dockerfile`:
```bash
  cargo build --release
```

6. Stopping the containers
To stop the containers, press `Ctrl + C` (Windows) or `Cmd + C` (MacOS) in the terminal where `docker-compose` is running, or run:
```bash
docker-compose down
```

### Troubleshooting
- **Redis Connection Issues**: Ensure that Redis is running correctly inside Docker by checking the logs with:
```bash
docker logs redis
```

- **MinIO Issues**: If you have trouble accessing MinIO, ensure that the MinIO service is running:
```bash
docker logs minio
```

- **FFmpeg Issues**: If you encounter issues with FFmpeg, verify that FFmpeg is installed correctly. Run the following command inside the container to check the FFmpeg installation:
```bash
docker exec -it be-streaming-rust-actix-web /bin/bash/ffmpeg -version
```

### Additional Notes
- **Volumes**: The Docker setup includes volume mounting for the application, allowing you to modify files on your local machine and have them reflected inside the container.
- **Ports**: Ensure that ports `8080`, `6379`, and `9000` are not in use on your local machine before running the Docker containers.