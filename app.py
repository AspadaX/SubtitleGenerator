import streamlit as st
import requests
import base64
import json

# Title of the app
st.title("SubtitleGenerator")

# Language selection dropdown
languages = ["zh", "en", "jp"]
selected_language = st.selectbox("Choose a language 选择语言", languages)

# File uploader
uploaded_file = st.file_uploader("Choose a video file 选择视频文件", type=["mp4", "mkv", "avi", "mov"])

if uploaded_file is not None:
    # Show a progress bar
    progress_bar = st.progress(0)

    # API endpoint
    url = "http://localhost:10002/upload"

    # Prepare the file and JSON data to be sent in a multipart/form-data request
    files = {
        'file': (uploaded_file.name, uploaded_file.getvalue(), uploaded_file.type),
        'json': (None, json.dumps({"language": selected_language}), 'application/json'),
    }

    # Function to update the progress bar
    def upload_progress(monitor):
        progress = int((monitor.bytes_read / monitor.len) * 100)
        progress_bar.progress(progress)

    # Upload the file
    with st.spinner('Processing...'):
        response = requests.post(url, files=files, stream=True)
        progress_bar.progress(100)

    if response.status_code == 200:
        srt_base64 = response.text
        srt_content = base64.b64decode(srt_base64)
        
        # Save the SRT content to a file
        with open("subtitles.srt", "wb") as srt_file:
            srt_file.write(srt_content)
        
        st.success("Subtitles generated successfully. Download below 字幕已生成，下方下载:")
        st.download_button(
            label="Download Subtitles 下载字幕", 
            data=srt_content, 
            file_name="subtitles.srt", 
            mime="text/srt"
        )
    else:
        st.error(
            f"Failed to generate subtitles. due to: {response.content.decode()}. Error code: {response.status_code}"
        )