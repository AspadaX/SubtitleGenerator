#!/bin/bash

# Start the core binary in the background
./core &

# Start the Streamlit app
streamlit run app.py