# Setup for windows

## Requirements

- MSYS2
  - [leptonica](https://packages.msys2.org/base/mingw-w64-leptonica)
    - mingw-w64-x86_64-leptonica
  - [tesseract](https://packages.msys2.org/base/mingw-w64-tesseract)
    - mingw-w64-x86_64-tesseract-ocr
  - [tessdata](https://packages.msys2.org/base/mingw-w64-tesseract-data)
    - mingw-w64-x86_64-tesseract-data-eng
  - [pkg-config](https://packages.msys2.org/base/mingw-w64-pkg-config) (optional: but useful for checking if the required libraries are installed)
    - mingw-w64-x86_64-pkg-config

## Steps

1. Download MSYS2 from <https://www.msys2.org/>
   1. Run the installer and uncheck the "Run MSYS2 now" checkbox
   2. Open a command prompt and run the folllowing commands
      1. Assuming you installed MSYS2 to the default location

         ```cmd
         setx path C:\msys64\mingw64\bin
         setx path C:\msys64\usr\bin
         setx TESSDATA_PREFIX C:\msys64\mingw64\share\tessdata
         ```

         This will add the MSYS2 binaries to your path so you can run them from the command prompt
   3. Close the command prompt and open a new one to apply the changes to your path variable
   4. Run the following command to install the required packages

      ```cmd
      pacman -S mingw-w64-x86_64-leptonica mingw-w64-x86_64-tesseract mingw-w64-x86_64-tesseract-data-eng mingw-w64-x86_64-pkg-config
      ```

   5. Run the following command to check if the required libraries are installed

      ```cmd
      pkg-config --cflags --libs lept tesseract
      ```

      you should see something like this

      ```cmd
      -IC:/msys64/mingw64/include -IC:/msys64/mingw64/include/leptonica -LC:/msys64/mingw64/lib -lleptonica -ltesseract -larchive -lcurl
      ```
