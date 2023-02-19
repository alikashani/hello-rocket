# Build Docker image
docker build -t rust-debian .
# Run built Docker image
docker run -p 8000:8000 rust-debian
