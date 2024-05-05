#!/bin/bash
if [ -f "venv/Scripts/activate" ]; then
    source "venv/Scripts/activate"
fi
python -i src/__main__.py
