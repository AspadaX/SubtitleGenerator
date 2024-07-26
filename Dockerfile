# Start from a NVIDIA CUDA base image that includes the necessary CUDA runtime
FROM nvidia/cuda:12.4.1-runtime-ubuntu22.04

# Replace sources list to use Tsinghua University mirror
RUN sed -i 's@http://archive.ubuntu.com/ubuntu/@https://mirrors.tuna.tsinghua.edu.cn/ubuntu/@g' /etc/apt/sources.list

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y \
    libmysqlclient-dev \
    clang \
    cmake \
    ffmpeg \
    libasound2-dev \
    python3.11 \
    python3-pip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory in the container
WORKDIR /app

# Copy the pre-built Rust backend binary and other necessary files
COPY ./core /app

# Copy the start script
COPY ./start.sh /app

# Copy the requirements file into the container
COPY requirements.txt .

# Copy the model file
COPY ./models/x-ggml-model.zh.bin /app

# Install the dependencies
RUN pip install --no-cache-dir -r requirements.txt \
    -i https://pypi.tuna.tsinghua.edu.cn/simple

# Copy the application code into the container
COPY app.py .

# Expose the port that Streamlit runs on
EXPOSE 8501

RUN chmod +x /app/start.sh

# Run the start script
CMD ["./start.sh"]