#!/bin/bash
if [ -f "venv/Scripts/activate" ]; do
    source "venv/Scripts/activate"
done
python -i src/__main__.py