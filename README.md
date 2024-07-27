
# Subtitle Generator

<p align="center">
  <img src="https://github.com/AspadaX/SubtitleGenerator/blob/main/assets/7271-ezgif.com-video-to-gif-converter.gif" alt="SubtitleGenerator GIF">
</p>

Upload your video and get subtitles. 

## Table of Contents

- [Subtitle Generator](#subtitle-generator)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Requirements](#requirements)
  - [Installation](#installation)
    - [Prepare the model file](#prepare-the-model-file)
    - [Docker Installation](#docker-installation)
  - [Usage](#usage)
  - [Project Structure](#project-structure)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgments](#acknowledgments)

## Features

- Subtitle generation using a pre-trained model.
- Easy-to-use web interface built with Streamlit.
- Efficient processing with NVIDIA CUDA.

## Requirements

- Docker
- NVIDIA GPU and drivers compatible with CUDA 12.4.1

## Installation

### Prepare the model file

1. Download the [model](https://hf-mirror.com/ChrisZais/whisperapp/resolve/main/x-ggml-model.zh.bin?download=true) and place the weight to `./model`

### Docker Installation

1. **Clone the Repository**

   ```sh
   git clone https://github.com/yourusername/subtitle-generator.git
   cd subtitle-generator
   ```

2. **Build the Docker Image**

   ```sh
   docker compose up -d
   ```

This will start the application, and you can access the Streamlit interface at [http://localhost:8501](http://localhost:8501).

## Usage

1. **Access the Web Interface**

   Open your web browser and navigate to [http://localhost:8501](http://localhost:8501).

2. **Upload Video**

   Use the provided interface to upload your video file.

3. **Generate Subtitles**

   The backend will process the video, and subtitles will be generated and downloaded to your local computer.

## Project Structure

```plaintext
.
├── Dockerfile
├── README.md
├── app.py
├── core
│   └── ... (pre-built Rust backend binary and necessary files)
├── requirements.txt
└── start.sh
```

- `Dockerfile`: The Dockerfile to build the project container.
- `README.md`: This README file.
- `app.py`: The Streamlit application code.
- `core/`: Directory containing the pre-built Rust backend binary.
- `requirements.txt`: Python dependencies file.
- `start.sh`: Script to start the backend and frontend.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.

## License

This project is for non-commercial usages. For commercial usages, please contact the author: `baoxinyuworks@163.com`

## Acknowledgments

- NVIDIA for providing the CUDA base image.
- Streamlit for the easy-to-use web application framework.
- Tsinghua University for their software mirror.
